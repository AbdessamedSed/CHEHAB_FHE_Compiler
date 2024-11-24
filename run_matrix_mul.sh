#!/bin/bash
# Job name
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=32
#SBATCH --time=24:00:00
# Memory allocation per CPU core
#SBATCH -o matrix_mul.out
#SBATCH -e matrix_mul.err

# Load necessary modules or libraries
module load gcc

cd /scratch/as20733/CHEHAB/build/benchmarks/matrix_mul

./matrix_mul 1 0 1 1 8


