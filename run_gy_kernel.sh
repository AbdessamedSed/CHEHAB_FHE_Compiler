#!/bin/bash
# Job name
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=32
#SBATCH --time=24:00:00
# Memory allocation per CPU core
#SBATCH -o gy_kernel.out
#SBATCH -e gy_kernel.err

# Load necessary modules or libraries
module load gcc

cd /scratch/as20733/CHEHAB/build/benchmarks/gy_kernel

./gy_kernel 1 0 1 1 8
