cmake_minimum_required(VERSION 3.10)
project(DagBenchmarkSuite)

# --- Étape 1: Compiler le générateur de code C++ (DAG.cpp) ---
# DAG.cpp se trouve dans le répertoire courant (CMAKE_CURRENT_SOURCE_DIR)
add_executable(GenerateDagProgram DAG.cpp)
# Le générateur lui-même n'a pas besoin de se lier à fheco.

# --- Définir les chemins pour les fichiers générés par GenerateDagProgram ---
set(GENERATED_FHE_PROGRAM_CPP ${CMAKE_CURRENT_BINARY_DIR}/fhe_generated_dag.cpp)
set(GENERATED_FHE_IO_FILE ${CMAKE_CURRENT_BINARY_DIR}/fhe_io_example_dag.txt)

# --- Configurer les arguments pour le générateur ---
set(DAG_TYPE_ARG "F_E" CACHE STRING "Type of DAG: F_F, F_E, E_F, E_E")
# Vous pouvez ajouter d'autres options CACHE pour les paramètres de DAG.cpp::main
# set(DAG_NUM_INPUTS 3 CACHE STRING "Number of initial inputs")
# set(DAG_TARGET_DEPTH 5 CACHE STRING "Target depth")
# set(DAG_P_REUSE 0.5 CACHE STRING "Reuse probability")
# set(DAG_OPS_PER_LEVEL 2 CACHE STRING "Operations per level")

# --- Étape 2: Exécuter le générateur pour créer les fichiers sources du benchmark ---
add_custom_command(
    OUTPUT ${GENERATED_FHE_PROGRAM_CPP} ${GENERATED_FHE_IO_FILE}
    COMMAND $<TARGET_FILE:GenerateDagProgram> ${DAG_TYPE_ARG} # Passez d'autres args ici si besoin
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR} 
    DEPENDS GenerateDagProgram 
    COMMENT "Generating FHE DAG source files (${DAG_TYPE_ARG})..."
    VERBATIM # Important pour gérer correctement les arguments de la commande
)

# Cible pour s'assurer que la commande personnalisée est exécutée
add_custom_target(GeneratedFheDagSourceFiles
    DEPENDS ${GENERATED_FHE_PROGRAM_CPP} ${GENERATED_FHE_IO_FILE}
)

# --- Étape 3: Compiler le programme FHECO généré ---
add_executable(DagFheBenchmark ${GENERATED_FHE_PROGRAM_CPP})

# Ajouter utils.cpp et le répertoire include pour utils.hpp si nécessaire
set(UTILS_CPP ${CMAKE_CURRENT_SOURCE_DIR}/he/utils.cpp)
set(UTILS_HPP_DIR ${CMAKE_CURRENT_SOURCE_DIR}/he)

if(EXISTS ${UTILS_CPP})
    target_sources(DagFheBenchmark PRIVATE ${UTILS_CPP})
    message(STATUS "Added ${UTILS_CPP} to DagFheBenchmark sources.")
endif()
if(IS_DIRECTORY ${UTILS_HPP_DIR}) # Vérifier si le dossier existe
    target_include_directories(DagFheBenchmark PRIVATE ${UTILS_HPP_DIR})
    message(STATUS "Added ${UTILS_HPP_DIR} to DagFheBenchmark include directories.")
endif()

# Lier DagFheBenchmark avec la bibliothèque FHECO
# FHECO doit être trouvé par CMake (généralement configuré dans un CMakeLists.txt parent)
target_link_libraries(DagFheBenchmark PRIVATE fheco) 

# S'assurer que les fichiers sources sont générés AVANT de compiler DagFheBenchmark
add_dependencies(DagFheBenchmark GeneratedFheDagSourceFiles)

# Optionnel: Cible pour exécuter le benchmark
# add_custom_target(RunDagFheBenchmark
#     COMMAND $<TARGET_FILE:DagFheBenchmark> # Ajoutez les arguments pour le main de DagFheBenchmark ici
#     DEPENDS DagFheBenchmark
#     WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR} # Pour trouver fhe_io_example_dag.txt
#     COMMENT "Running DagFheBenchmark..."
#     USES_TERMINAL
# )

# Les anciennes commandes "copy_my_file_XDAG" sont supprimées car la gestion des 
# fichiers utils est faite via target_sources/target_include_directories, et
# le fichier IO est généré.