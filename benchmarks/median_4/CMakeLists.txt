
file(MAKE_DIRECTORY ${CMAKE_BINARY_DIR}/benchmarks/median_3/he)
add_executable(median_3 median_3.cpp)
target_link_libraries(median_3 PRIVATE fheco)
 
# Define the source file and destination file

set(SOURCE_FILE_2 ${CMAKE_SOURCE_DIR}/benchmarks/median_3/he/utils.cpp)
set(DESTINATION_FILE_2 ${CMAKE_SOURCE_DIR}/build/benchmarks/median_3/he/utils.cpp)

set(SOURCE_FILE_3 ${CMAKE_SOURCE_DIR}/benchmarks/median_3/he/utils.hpp)
set(DESTINATION_FILE_3 ${CMAKE_SOURCE_DIR}/build/benchmarks/median_3/he/utils.hpp)

set(SOURCE_FILE_4 ${CMAKE_SOURCE_DIR}/benchmarks/median_3/he/CMakeLists.txt)
set(DESTINATION_FILE_4 ${CMAKE_SOURCE_DIR}/build/benchmarks/median_3/he/CMakeLists.txt)
 
set(SOURCE_FILE_5 ${CMAKE_SOURCE_DIR}/benchmarks/median_3/fhe_io_example.txt)
set(DESTINATION_FILE_5 ${CMAKE_SOURCE_DIR}/build/benchmarks/median_3/fhe_io_example.txt)

set(SOURCE_FILE_6 ${CMAKE_SOURCE_DIR}/benchmarks/median_3/generate_median_3.py)
set(DESTINATION_FILE_6 ${CMAKE_SOURCE_DIR}/build/benchmarks/median_3/generate_median_3.py)

add_custom_target(copy_my_file_6median_3 ALL 
    COMMAND ${CMAKE_COMMAND} -E copy ${SOURCE_FILE_6} ${DESTINATION_FILE_6}
    DEPENDS ${SOURCE_FILE}
    COMMENT "Copying ${SOURCE_FILE_6} to ${DESTINATION_FILE_6}"
)
add_custom_target(copy_my_file_2median_3 ALL
    COMMAND ${CMAKE_COMMAND} -E copy ${SOURCE_FILE_2} ${DESTINATION_FILE_2}
    DEPENDS ${SOURCE_FILE}
    COMMENT "Copying ${SOURCE_FILE_2} to ${DESTINATION_FILE_2}"
)
add_custom_target(copy_my_file_3median_3 ALL
    COMMAND ${CMAKE_COMMAND} -E copy ${SOURCE_FILE_3} ${DESTINATION_FILE_3}
    DEPENDS ${SOURCE_FILE}
    COMMENT "Copying ${SOURCE_FILE_3} to ${DESTINATION_FILE_3}"
)
add_custom_target(copy_my_file_5median_3 ALL 
    COMMAND ${CMAKE_COMMAND} -E copy ${SOURCE_FILE_5} ${DESTINATION_FILE_5}
    DEPENDS ${SOURCE_FILE}
    COMMENT "Copying ${SOURCE_FILE_5} to ${DESTINATION_FILE_5}"
)
set(SOURCE_FILE_4 ${CMAKE_SOURCE_DIR}/benchmarks/median_3/he/CMakeLists.txt)
set(DESTINATION_FILE_4 ${CMAKE_SOURCE_DIR}/build/benchmarks/median_3/he/CMakeLists.txt)
add_custom_target(copy_my_file_4median_3 ALL
    COMMAND ${CMAKE_COMMAND} -E copy ${SOURCE_FILE_4} ${DESTINATION_FILE_4}
    DEPENDS ${SOURCE_FILE}
    COMMENT "Copying ${SOURCE_FILE_4} to ${DESTINATION_FILE_4}"
)