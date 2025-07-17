import os
import shutil
import subprocess
import csv
import re
import statistics
import time

# --- SCRIPT CONFIGURATION ---

# Specify the parent folder containing the benchmarks and build subfolders
benchmarks_folder = "benchmarks"
build_folder = os.path.join("build", "benchmarks")

# --- Output Files ---
output_csv = "results.csv"
beam_search_csv = "beam_search_results.csv"

# --- BEAM SEARCH PARAMETERS ---
BEAM_WIDTH = 5
SEARCH_DEPTH = 10
BRANCHING_FACTOR = 20

# --- Benchmark Configuration ---
benchmark_folders = ["dot_product", "l2_distance", "hamming_distance"]
# Number of times to run the entire process for each benchmark configuration
iterations = 10
# Number of times to retry a failed benchmark run before giving up
MAX_RETRIES = 10
slot_counts = [4, 8, 16, 32]

# --- Static Definitions ---
operations = ["add", "sub", "multiply_plain", "rotate_rows", "square", "multiply"]
infos = ["benchmark"]
additional_infos = ["Depth", "Multiplicative Depth", "compile_time( ms )", "execution_time (ms)"]
infos.extend(operations)
infos.extend(additional_infos)

# --- INITIALIZE CSV FILES ---
with open(output_csv, mode='w', newline='') as file:
    writer = csv.writer(file)
    writer.writerow(infos)

beam_infos = ["benchmark_id", "slot_count", "run_iteration", "beam_search_iteration", "costs"]
with open(beam_search_csv, mode='w', newline='') as file:
    writer = csv.writer(file)
    writer.writerow(beam_infos)

# --- 1. BUILD THE PROJECT ---
try:
    print("--- Building Project with CMake ---")
    print("Running=> cmake -S . -B build")
    subprocess.run(['cmake', '-S', '.', '-B', 'build'], check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    
    print("Running=> cmake --build build")
    subprocess.run(['cmake', '--build', 'build'], check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    print("--- Build Complete ---")
except subprocess.CalledProcessError as e:
    print(f"FATAL: CMake build failed. Aborting script.")
    print(f"Command '{' '.join(e.cmd)}' failed with return code {e.returncode}")
    print(f"Stdout:\n{e.stdout or 'No stdout.'}")
    print(f"Stderr:\n{e.stderr or 'No stderr.'}")
    exit(1)

# --- 2. RUN BENCHMARKS ---
for subfolder_name in benchmark_folders:
    build_path = os.path.join(build_folder, subfolder_name)
    
    if not os.path.isdir(build_path):
        print(f"Warning: Build path not found for {subfolder_name}. Skipping.")
        continue
        
    for slot_count in slot_counts:
        print("****************************************************************")
        print(f"***** Preparing Benchmark: {subfolder_name}, Slot Count: {slot_count} *****")
        
        operation_stats = {key: [] for key in operations + additional_infos}

        for iteration_num in range(iterations):
            print(f"===> Starting Run: {iteration_num + 1}/{iterations}")

            # *** NEW: RETRY LOOP ***
            for attempt in range(MAX_RETRIES):
                try:
                    print(f"--- Attempt {attempt + 1}/{MAX_RETRIES} ---")

                    # --- Generate IO file ---
                    print(f"Generating io_file for {subfolder_name}...")
                    pro = subprocess.Popen(['python3', f'generate_{subfolder_name}.py', '--slot_count', str(slot_count)], cwd=build_path)
                    pro.wait()
                    if pro.returncode != 0:
                        raise Exception(f"io_file generation failed with return code {pro.returncode}")

                    # --- Run Main Executable ---
                    window_size = 0
                    command = f"./{subfolder_name} 1 {window_size} 1 1 {slot_count} {BEAM_WIDTH} {SEARCH_DEPTH} {BRANCHING_FACTOR}"
                    
                    print(f"Running command: {command}")
                    result = subprocess.run(
                        command, shell=True, check=True,
                        stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True, cwd=build_path
                    )

                    # --- Parse Beam Search Costs ---
                    benchmark_id = f"{subfolder_name}_{slot_count}"
                    for line in result.stderr.splitlines():
                        match = re.search(r"Iteration (\d+): Top \d+ costs = \[(.*?)\]", line)
                        if match:
                            beam_iter_num, costs_str = match.groups()
                            with open(beam_search_csv, mode='a', newline='') as bs_file:
                                writer = csv.writer(bs_file)
                                writer.writerow([benchmark_id, slot_count, iteration_num + 1, beam_iter_num, f"\"[{costs_str}]\""])

                    # --- Parse Other Stats ---
                    for line in result.stdout.splitlines():
                        if 'ms' in line:
                            operation_stats["compile_time( ms )"].append(float(line.split()[0]))
                            break
                    depth_match = re.search(r'max:\s*\((\d+),\s*(\d+)\)', result.stdout)
                    if depth_match:
                        operation_stats["Depth"].append(int(depth_match.group(1)))
                        operation_stats["Multiplicative Depth"].append(int(depth_match.group(2)))

                    # --- Build and Run FHE ---
                    build_path_he = os.path.join(build_path, "he")
                    print("Building and running FHE code...")
                    subprocess.run(['cmake', '-S', '.', '-B', 'build'], cwd=build_path_he, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
                    subprocess.run(['cmake', '--build', 'build'], cwd=build_path_he, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
                    result_fhe_run = subprocess.run(f"./main", shell=True, check=True, cwd=os.path.join(build_path_he, "build"),
                                                    stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
                    for line in result_fhe_run.stdout.splitlines():
                        if 'ms' in line:
                            operation_stats["execution_time (ms)"].append(float(line.split()[0]))
                            break

                    # --- Parse C++ File ---
                    file_name = os.path.join(build_path_he, "_gen_he_fhe.cpp")
                    with open(file_name, "r") as file_cpp:
                        file_content = file_cpp.read()
                        for op in operations:
                            operation_stats[op].append(len(re.findall(rf'\b{op}\b', file_content)))
                    
                    # If we get here, the entire process was successful.
                    print(f"--- Attempt {attempt + 1} Succeeded. ---")
                    break  # Exit the retry loop

                except Exception as e:
                    print(f"--- Attempt {attempt + 1} FAILED for {subfolder_name} (slot_count {slot_count}). ---")
                    print(f"Error Type: {type(e).__name__}")
                    
                    # Provide more details for subprocess errors
                    if isinstance(e, subprocess.CalledProcessError):
                        print(f"Stderr: {e.stderr or 'N/A'}")
                        print(f"Stdout: {e.stdout or 'N/A'}")
                    else:
                        print(f"Details: {e}")

                    if attempt < MAX_RETRIES - 1:
                        print("--- Retrying in 1 second... ---")
                        time.sleep(1) # Small delay before retrying
                    else:
                        print(f"--- All {MAX_RETRIES} retries failed. Giving up on this benchmark run. ---")

            else:  # This `else` belongs to the `for attempt...` loop
                # It only runs if the loop completes without a `break`, meaning all retries failed.
                print(f"CRITICAL: Could not complete run for {subfolder_name} (slot_count {slot_count}) after {MAX_RETRIES} attempts.")
                continue # Skip to the next main iteration

        # --- AGGREGATE AND WRITE RESULTS ---
        bench_name_id = f"{subfolder_name}_{slot_count}"
        row = [bench_name_id]
        print(f"\n--- Aggregating stats for {bench_name_id} ---")
        
        for key_stat in infos[1:]:
            values = operation_stats.get(key_stat, [])
            if values:
                row.append(statistics.median([v for v in values if isinstance(v, (int, float))]))
            else:
                row.append(None)
        
        if any(val is not None for val in row[1:]):
            with open(output_csv, mode='a', newline='') as file_csv_out:
                writer = csv.writer(file_csv_out)
                writer.writerow(row)
            print(f"Appended to {output_csv}: {row}")
        else:
            print(f"No data collected for {bench_name_id}, not writing to CSV.")

print(f"\nScript finished. Results are in '{output_csv}' and '{beam_search_csv}'")