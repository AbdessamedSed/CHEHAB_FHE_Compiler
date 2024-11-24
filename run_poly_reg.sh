#!/bin/bash
# Job name
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=32
#SBATCH --time=24:00:00
# Memory allocation per CPU core
#SBATCH -o poly_reg.out
#SBATCH -e poly_reg.err

# Load necessary modules or libraries
module load gcc

cd /scratch/as20733/CHEHAB/build/benchmarks/poly_reg

./poly_reg 1 0 1 1 64


