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
output_csv = "polynomials_results.csv" 
beam_search_csv = "polynomials_beam_search_results.csv"

# --- BEAM SEARCH PARAMETERS ---
# These will be passed to the Rust executable and can be changed here
BEAM_WIDTH = 5
SEARCH_DEPTH = 10
BRANCHING_FACTOR = 20

# --- Benchmark Configuration ---
# Number of times to retry a failed benchmark run before giving up
MAX_RETRIES = 10
# Number of times to run the entire process for each benchmark configuration
iterations = 10
# The specific benchmark folder to run
polynomial_folders = ["polynomials_coyote"] 
# Test configurations
depths = [5, 6, 7, 8]  # Updated as requested
regimes = ["50-50","100-50", "100-100"]

# --- Static Definitions ---
operations = ["add", "sub", "multiply_plain", "rotate_rows", "square", "multiply"]
infos = ["benchmark"]
additional_infos =[ "Depth", "Multiplicative Depth","compile_time( ms )", "execution_time (ms)"]
infos.extend(operations)
infos.extend(additional_infos)

# --- INITIALIZE CSV FILES ---
with open(output_csv, mode='w', newline='') as file:
    writer = csv.writer(file)
    writer.writerow(infos)
    
beam_infos = ["benchmark_id", "regime", "depth", "run_iteration", "beam_search_iteration", "costs"]
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
for subfolder_name in polynomial_folders: 
    build_path = os.path.join(build_folder, subfolder_name)
    if not os.path.isdir(build_path):
        print(f"Warning: Build path not found for {subfolder_name}. Skipping.")
        continue

    for regime in regimes :
        for tree_depth in depths : 
            benchmark_name_id = f'tree_{regime}-{tree_depth}'
            print("****************************************************************")
            print(f"***** Preparing Benchmark: {benchmark_name_id} *****")
            
            operation_stats = {key: [] for key in operations + additional_infos}

            for i in range(iterations):
                print(f"===> Starting Run: {i+1}/{iterations}")

                # *** RETRY LOOP ***
                for attempt in range(MAX_RETRIES):
                    try:
                        print(f"--- Attempt {attempt + 1}/{MAX_RETRIES} ---")

                        # --- Construct the command with beam search parameters ---
                        # Format: ./executable <tree_depth> <run_iter> <regime> [beam_width] [search_depth] [branching_factor]
                        command = f"./{subfolder_name} {tree_depth} {i+1} {regime} {BEAM_WIDTH} {SEARCH_DEPTH} {BRANCHING_FACTOR}"
                        
                        print(f"Running command: {command}")
                        result = subprocess.run(
                            command, shell=True, check=True, 
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, 
                            universal_newlines=True, cwd=build_path
                        )

                        # --- A. Parse Beam Search Costs ---
                        for line in result.stderr.splitlines():
                            match = re.search(r"Iteration (\d+): Top \d+ costs = \[(.*?)\]", line)
                            if match:
                                beam_iter_num, costs_str = match.groups()
                                with open(beam_search_csv, mode='a', newline='') as bs_file:
                                    writer = csv.writer(bs_file)
                                    writer.writerow([benchmark_name_id, regime, tree_depth, i + 1, beam_iter_num, f"\"[{costs_str}]\""])

                        # --- B. Parse Other Stats ---
                        for line in result.stdout.splitlines():
                            if 'ms' in line:
                                operation_stats["compile_time( ms )"].append(float(line.split()[0]))
                                break
                        depth_match = re.search(r'max:\s*\((\d+),\s*(\d+)\)', result.stdout)
                        if depth_match:
                            operation_stats["Depth"].append(int(depth_match.group(1)))
                            operation_stats["Multiplicative Depth"].append(int(depth_match.group(2)))

                        # --- C. Build and Run FHE ---
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

                        # --- D. Parse C++ File ---
                        file_name = os.path.join(build_path_he, "_gen_he_fhe.cpp")
                        with open(file_name, "r") as file: 
                            file_content = file.read()
                            for op in operations:
                                operation_stats[op].append(len(re.findall(rf'\b{op}', file_content)))

                        print(f"--- Attempt {attempt + 1} Succeeded. ---")
                        break # Success, exit retry loop
                    
                    except Exception as e:
                        print(f"--- Attempt {attempt + 1} FAILED for {benchmark_name_id} (Run {i+1}). ---")
                        print(f"Error Type: {type(e).__name__}")
                        if isinstance(e, subprocess.CalledProcessError):
                            print(f"Stderr: {e.stderr or 'N/A'}")
                        else:
                            print(f"Details: {e}")
                        if attempt < MAX_RETRIES - 1:
                            print("--- Retrying in 1 second... ---")
                            time.sleep(1)
                        else:
                            print(f"--- All {MAX_RETRIES} retries failed. ---")
                
                else: # This belongs to the `for attempt...` loop
                    print(f"CRITICAL: Could not complete run for {benchmark_name_id} (Run {i+1}) after {MAX_RETRIES} attempts.")
                    continue

            # --- 3. AGGREGATE AND WRITE RESULTS ---
            row = [benchmark_name_id]
            print(f"\n--- Aggregating stats for {benchmark_name_id} ---")
            
            for key in infos[1:]:
                values = operation_stats.get(key, [])
                if values:
                    # Use median for timings and mean for counts/depths
                    if "time" in key:
                         row.append(statistics.median([v for v in values if isinstance(v, (int, float))]))
                    else:
                         row.append(statistics.mean([v for v in values if isinstance(v, (int, float))]))
                else:
                    row.append(None)

            if any(val is not None for val in row[1:]):
                with open(output_csv, mode='a', newline='') as file:
                    writer = csv.writer(file)
                    writer.writerow(row)
                print(f"Appended to {output_csv}: {row}")
            else:
                print(f"No data collected for {benchmark_name_id}, not writing to CSV.")

print(f"\nScript finished. Results are in '{output_csv}' and '{beam_search_csv}'")