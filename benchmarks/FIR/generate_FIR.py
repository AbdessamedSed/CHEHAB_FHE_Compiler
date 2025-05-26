import numpy as np
import argparse

# Create the parser
parser = argparse.ArgumentParser(description="Get io_file generation parameters")
parser.add_argument("--slot_count", required=True, type=int, help="Slot count")

# Parse arguments
args = parser.parse_args()
function_slot_count = args.slot_count

# Configuration flags
is_vectorization_activated = True
is_cipher = 1
is_signed = 1

# Hardcoded filter
filter = [1, 2, 3]
filter_size = len(filter)

# Generate random signal
signal = np.random.randint(0, 10, function_slot_count)
output = np.zeros(function_slot_count, dtype=int)

# Perform convolution
for i in range(function_slot_count):
    acc = 0
    for j in range(filter_size):
        if (i - j >= 0):
            acc += signal[i - j] * filter[j]
    output[i] = acc

# Vectorized mode
if is_vectorization_activated:
    with open("fhe_io_example.txt", "w") as file:
        nb_inputs = function_slot_count
        nb_outputs = function_slot_count
        header = f"1 {nb_inputs} {nb_outputs}\n"
        file.write(header)

        rows = []

        for i in range(function_slot_count):
            line = f"s_{i} {is_cipher} {is_signed} {signal[i]}\n"
            rows.append(line)

        for i in range(function_slot_count):
            line = f"output_{i} {is_cipher} {int(output[i])}\n"
            rows.append(line)

        file.writelines(rows)

# Non-vectorized mode
else:
    c0 = np.random.randint(0, 10, function_slot_count)
    # to develop
    output = np.zeros(function_slot_count, dtype=int)

    for i in range(function_slot_count):
        acc = 0
        for j in range(filter_size):
            if (i - j >= 0):
                acc += c0[i - j] * fitler[j]
        output[i] = acc

    with open("fhe_io_example.txt", "w") as file:
        nb_inputs = 1
        nb_outputs = 1
        header = f"{function_slot_count} {nb_inputs} {nb_outputs}\n"
        file.write(header)

        rows = []
        row = f"c0 {is_cipher} {is_signed} " + " ".join(str(num) for num in c0) + "\n"
        rows.append(row)
        row = f"c_result {is_cipher} " + " ".join(str(num) for num in output) + "\n"
        rows.append(row)

        file.writelines(rows)
