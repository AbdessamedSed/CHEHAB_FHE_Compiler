import os
import shutil
import subprocess
import csv
import re
import statistics

# Specify the parent folder containing the benchmarks and build subfolders
benchmarks_folder = "benchmarks"
build_folder = os.path.join("build", "benchmarks")
# to be run after with slot_count = 8 for both matrix_mul and rober_cross
output_csv = "results.csv"
# vectorization_csv = "vectorization.csv"
operations = ["add", "sub", "multiply_plain", "rotate_rows", "square", "multiply"]
infos = ["benchmark"]
additional_infos =[ "Depth", "Multplicative Depth","compile_time( ms )", "execution_time (ms)"]
infos.extend(operations)
infos.extend(additional_infos)

with open(output_csv, mode='w', newline='') as file:
    writer = csv.writer(file)
    writer.writerow(infos)

try:
    print("run=> cmake', '-S', '.', '-B', 'build' ")
    result = subprocess.run(
        ['cmake', '-S', '.', '-B', 'build'],
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        universal_newlines=True 
    )
    print("run=> 'cmake', '--build', 'build'")
    result = subprocess.run(
        ['cmake', '--build', 'build'],
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        universal_newlines=True 
    )
except subprocess.CalledProcessError as e:
    stdout_message = e.stdout if e.stdout else "No stdout."
    stderr_message = e.stderr if e.stderr else "No stderr."
    print(f"Command '{' '.join(e.cmd)}' failed with return code {e.returncode}")
    print(f"Stdout:\n{stdout_message}")
    print(f"Stderr:\n{stderr_message}")
    exit(1) 

# Iterate through each item in the benchmarks folder
# "hamming_dist","poly_reg","lin_reg","l2_distance","dot_product","box_blur"
# "box_blur","gx_kernel","gy_kernel","sobel","roberts_cross","matrix_mul"
# benchmark_folders = ["dot_product"]
# benchmark_folders = ["gx_kernel", "gy_kernel", "roberts_cross", "matrix_mul"]
benchmark_folders = ["dot_product"]

###############################
### specify the number of iteration
iterations = 1
for subfolder_name in benchmark_folders:
    benchmark_path = os.path.join(benchmarks_folder, subfolder_name)
    build_path = os.path.join(build_folder, subfolder_name)
    optimization_time = ""
    execution_time = ""
    depth = ""
    multiplicative_depth = ""
    if os.path.isdir(build_path):
        ###############################################
        ##### loop over specified slot_counts #########
        slot_counts= [4]
        window_size = 0
        for slot_count in slot_counts :
            print("****************************************************************")
            print(f"*****run {subfolder_name} , for slot_count : {slot_count}******")
            operation_stats = {
            "add": [], "sub": [], "multiply_plain": [], "rotate_rows": [],
            "square": [], "multiply": [], "Depth": [], "Multiplicative Depth": [],
            "compile_time (ms)": [], "execution_time (ms)": []
            }
            try:
                print(f"Generating io_file for {subfolder_name} with slot_count {slot_count}")
                pro = subprocess.Popen(['python3', f'generate_{subfolder_name}.py', '--slot_count', str(slot_count)], cwd=build_path)
                pro.wait()
                if pro.returncode != 0:
                    print(f"Error generating io_file for {subfolder_name}. Return code: {pro.returncode}")
                    
                    continue 
            except FileNotFoundError:
                print(f"generate_{subfolder_name}.py not found in {build_path}")
                continue
            except Exception as e:
                print(f"An error occurred during io_file generation for {subfolder_name}: {e}")
                continue

            ######################################
            for iteration_num in range(iterations): # 
                print(f"===> Running iteration : {iteration_num + 1}")
                command = f"./{subfolder_name} 1 {window_size} 1 1 {slot_count}"
                try:
                    print(f"Running command in {build_path}: {command}")
                    result = subprocess.run(
                        command, shell=True, check=True,
                        stdout=subprocess.PIPE,
                        stderr=subprocess.PIPE,
                        universal_newlines=True,
                        cwd=build_path
                    )
                    lines = result.stdout.splitlines()

                    for line in lines:
                        if 'ms' in line: # 
                            try:
                                optimization_time_val = float(line.split()[0])
                                operation_stats["compile_time (ms)"].append(optimization_time_val)
                                print(f"Compile time found: {optimization_time_val} ms")
                                break
                            except (ValueError, IndexError):
                                print(f"Could not parse compile time from line: {line}")


                    depth_match = re.search(r'max:\s*\((\d+),\s*(\d+)\)', result.stdout)
                    if depth_match:
                        depth_val = int(depth_match.group(1))
                        multiplicative_depth_val = int(depth_match.group(2))
                        operation_stats["Depth"].append(depth_val)
                        operation_stats["Multiplicative Depth"].append(multiplicative_depth_val)
                        print(f"Depth: {depth_val}, Multiplicative Depth: {multiplicative_depth_val}")
                    else:
                        print(f"Could not find depth information in output for {subfolder_name}")
                       

                except subprocess.CalledProcessError as e:
                    stdout_message = e.stdout if e.stdout else "No stdout."
                    stderr_message = e.stderr if e.stderr else "No stderr."
                    print(f"Command '{e.cmd}' in {build_path} failed with error:\n{stderr_message}")
                    print(f"Stdout was:\n{stdout_message}")
                    continue # 

                build_path_he = os.path.join(build_path, "he")
                try:
                    print(f"Building FHE code in {build_path_he}")
                    result_cmake_config = subprocess.run(['cmake', '-S', '.', '-B', 'build'],
                        cwd=build_path_he,
                        check=True,
                        stdout=subprocess.PIPE,
                        stderr=subprocess.PIPE,
                        universal_newlines=True)
               

                    result_cmake_build = subprocess.run(['cmake', '--build', 'build'], cwd=build_path_he, universal_newlines=True,
                        check=True,
                        stdout=subprocess.PIPE,
                        stderr=subprocess.PIPE)
                 


                    build_path_he_build = os.path.join(build_path_he, "build")
                    fhe_command = f"./main"
                    print(f"Running FHE command in {build_path_he_build}: {fhe_command}")
                    result_fhe_run = subprocess.run(
                        fhe_command, shell=True, check=True,
                        stdout=subprocess.PIPE,
                        stderr=subprocess.PIPE,
                        universal_newlines=True,
                        cwd=build_path_he_build
                    )
                    print("**fhe run done**")
                    # print(f"FHE run stdout:\n{result_fhe_run.stdout}")
                    # print(f"FHE run stderr:\n{result_fhe_run.stderr}")

                    lines = result_fhe_run.stdout.splitlines()
                    execution_time_found = False
                    for line in lines:
                        if 'ms' in line:
                            try:
                                execution_time_val = float(line.split()[0])
                                operation_stats["execution_time (ms)"].append(execution_time_val)
                                print(f"Execution time found: {execution_time_val} ms")
                                execution_time_found = True
                                break
                            except (ValueError, IndexError):
                                print(f"Could not parse execution time from line: {line}")
                    if not execution_time_found:
                        print(f"Execution time (ms) not found in FHE output for {subfolder_name}")
                        # operation_stats["execution_time (ms)"].append(None)


                except subprocess.CalledProcessError as e:
                    stdout_message = e.stdout if e.stdout else "No stdout."
                    stderr_message = e.stderr if e.stderr else "No stderr."
                    print(f"Failed in building/running FHE code for benchmark: {subfolder_name}")
                    print(f"Command '{' '.join(e.cmd) if isinstance(e.cmd, list) else e.cmd}' failed with return code {e.returncode}")
                    print(f"Stdout:\n{stdout_message}")
                    print(f"Stderr:\n{stderr_message}")
                    continue #
                except FileNotFoundError:
                    print(f"CMake or FHE executable not found in {build_path_he} or {build_path_he_build}")
                    continue

                file_name = os.path.join(build_path_he, "_gen_he_fhe.cpp")
                try:
                    with open(file_name, "r") as file_cpp: #
                        file_content = file_cpp.read()
                        for op in operations:
                            nb_occurrences = len(re.findall(rf'\b{op}\b', file_content)) #
                            # print(f"==> {op}: {nb_occurrences}")
                            operation_stats[op].append(int(nb_occurrences))
                except FileNotFoundError:
                    print(f"Generated C++ file not found: {file_name}")
                    for op in operations:
                        # 
                        pass 
                    continue


            ####################################################################
            bench_name = f"{subfolder_name}_{slot_count}"
            row=[bench_name]
            print(f"\nAggregated stats for {bench_name}:")
            for key_stat, values in operation_stats.items(): #
                print(f"{key_stat} ==> {values}")
                if values:
                    numeric_values = [v for v in values if isinstance(v, (int, float))]
                    if numeric_values:
                        row.append(statistics.median(numeric_values))
                    else:
                        row.append(None) # 
                else:
                    row.append(None) # 
            #####################################################################
            #######################################################################
            if any(val is not None for val in row[1:]): #
                with open(output_csv, mode='a', newline='') as file_csv_out: #
                    writer = csv.writer(file_csv_out)
                    writer.writerow(row)
                print(f"Appended to CSV: {row}")
            else:
                print(f"No data collected for {bench_name}, not writing to CSV.")

print(f"Script finished. Results are in {output_csv}")