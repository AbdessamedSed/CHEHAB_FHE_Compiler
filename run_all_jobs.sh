#!/bin/bash

# List of job scripts to submit with sbatch
jobs=(
    "run_box_blur.sh"
    "run_dot_product.sh"
    "run_gx_kernel.sh"
    "run_gy_kernel.sh"
    "run_hamming_dist.sh"
    "run_l2_distance.sh"
    "run_lin_reg.sh"
    "run_matrix_mul.sh"
    "run_poly_reg.sh"
    "run_roberts_cross.sh"
)

# Loop through each job and submit it with sbatch
for job in "${jobs[@]}"; do
    echo "Submitting $job..."
    sbatch "$job"
    
    # Check if sbatch submission was successful
    if [ $? -ne 0 ]; then
        echo "Error submitting $job. Exiting."
        exit 1
    fi
    echo "$job submitted successfully."
done

echo "All jobs submitted."
