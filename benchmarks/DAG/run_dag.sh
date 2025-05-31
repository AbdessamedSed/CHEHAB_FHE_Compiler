#!/bin/bash

# --- Configuration ---
# Répertoire source du projet (où se trouve le CMakeLists.txt principal)
# Si le script est à la racine du projet, SOURCE_DIR="."
# Si le script est dans benchmarks/ et que le CMakeLists.txt principal est un niveau au-dessus:
SOURCE_DIR=".." # Ou "." si le CMakeLists.txt principal est dans benchmarks/
# Répertoire de build (relatif au répertoire source)
BUILD_SUBDIR="build"
# Chemin vers l'exécutable du générateur de DAG (relatif au répertoire de build)
GENERATOR_EXE_RELPATH="benchmarks/DAG/GenerateDagProgram" # Adaptez si votre CMakeLists.txt dans DAG/ change le nom de la cible
# Chemin vers l'exécutable du benchmark DAG (relatif au répertoire de build)
BENCHMARK_EXE_RELPATH="benchmarks/DAG/DagFheBenchmark" # Adaptez si votre CMakeLists.txt dans DAG/ change le nom de la cible


# --- Logique du Script ---
DAG_TYPE=$1
# Tous les arguments à partir du deuxième sont pour le main de FHECO
FHECO_MAIN_ARGS="${@:2}" 

# Vérifier si le type de DAG est fourni
if [ -z "$DAG_TYPE" ]; then
  echo "Usage: $0 <DAG_TYPE> [fheco_main_args...]"
  echo "  <DAG_TYPE> can be F_F, F_E, E_F, E_E, or a custom string your DAG.cpp::main expects."
  echo "Example: $0 E_F true 0 true false 1 true"
  exit 1
fi

# Chemin complet vers le répertoire de build
PROJECT_ROOT_DIR=$(cd "$(dirname "$0")/$SOURCE_DIR" && pwd) # Chemin absolu de la racine du projet
BUILD_DIR_ABSOLUTE="${PROJECT_ROOT_DIR}/${BUILD_SUBDIR}"

echo "Project Root: ${PROJECT_ROOT_DIR}"
echo "Build Directory: ${BUILD_DIR_ABSOLUTE}"

# 1. Créer le répertoire de build s'il n'existe pas
mkdir -p "${BUILD_DIR_ABSOLUTE}"

# 2. Reconfigurer CMake pour le bon type de DAG et builder
echo "--- Configuring and Building for DAG Type: ${DAG_TYPE} ---"
# L'option -S spécifie le répertoire source, -B le répertoire de build
# Le -DDAG_TYPE_ARG est passé au CMakeLists.txt qui se trouve dans DAG/ (s'il est inclus par le principal)
# ou au CMakeLists.txt principal qui le propage.
if ! cmake -S "${PROJECT_ROOT_DIR}" -B "${BUILD_DIR_ABSOLUTE}" -DDAG_TYPE_ARG=${DAG_TYPE} ; then
    echo "CMake configuration failed."
    exit 1
fi

# Compiler la cible spécifique du benchmark DAG. 
# Si votre CMakeLists.txt dans DAG/ nomme la cible DagFheBenchmark:
# Ou si vous avez une cible globale qui construit tout.
# Pour être plus précis, on peut cibler l'exécutable du benchmark.
# Assurez-vous que le nom de la cible est correct.
TARGET_BENCHMARK_NAME="DagFheBenchmark" # Le nom de la cible add_executable dans DAG/CMakeLists.txt
if ! cmake --build "${BUILD_DIR_ABSOLUTE}" --target ${TARGET_BENCHMARK_NAME} ; then
    echo "CMake build failed for target ${TARGET_BENCHMARK_NAME}."
    exit 1
fi

# 3. Exécuter le benchmark compilé
GENERATOR_FULL_PATH="${BUILD_DIR_ABSOLUTE}/${GENERATOR_EXE_RELPATH}"
BENCHMARK_FULL_PATH="${BUILD_DIR_ABSOLUTE}/${BENCHMARK_EXE_RELPATH}"

if [ ! -f "${BENCHMARK_FULL_PATH}" ]; then
    echo "Error: Benchmark executable ${BENCHMARK_FULL_PATH} not found after build."
    echo "Check GENERATOR_EXE_RELPATH, BENCHMARK_EXE_RELPATH, and CMake target names."
    exit 1
fi

echo "--- Running ${TARGET_BENCHMARK_NAME} (Type: ${DAG_TYPE}) with FHECO args: ${FHECO_MAIN_ARGS} ---"
# Exécuter depuis le répertoire où se trouve l'exécutable, car il s'attendra
# à trouver fhe_io_example_dag.txt (généré) dans son répertoire de travail (build/DAG/).
BENCHMARK_DIR=$(dirname "${BENCHMARK_FULL_PATH}")
BENCHMARK_EXE_NAME=$(basename "${BENCHMARK_FULL_PATH}")

(cd "${BENCHMARK_DIR}" && ./${BENCHMARK_EXE_NAME} ${FHECO_MAIN_ARGS})
# Le code de retour de l'exécution du benchmark est capturé par $?
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo "Benchmark execution failed with exit code ${EXIT_CODE}."
else
    echo "Benchmark execution completed."
fi

echo "--- Done ---"
exit $EXIT_CODE