#!/bin/bash
# Job name
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=32
#SBATCH --time=05:00:00
# Memory allocation per CPU core
#SBATCH -o box_blur.out
#SBATCH -e box_blur.err

# Load necessary modules or libraries
module load gcc

cd /scratch/as20733/CHEHAB/build/benchmarks/box_blur

./box_blur 1 0 1 1 16


