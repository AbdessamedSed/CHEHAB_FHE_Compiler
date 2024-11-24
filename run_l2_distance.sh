#!/bin/bash
# Job name
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=32
#SBATCH --time=24:00:00
# Memory allocation per CPU core
#SBATCH -o l2_distance.out
#SBATCH -e l2_distance.err

# Load necessary modules or libraries
module load gcc

cd /scratch/as20733/CHEHAB/build/benchmarks/l2_distance

./l2_distance 1 0 1 1 16


