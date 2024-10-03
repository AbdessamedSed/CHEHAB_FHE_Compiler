

add_executable(benchmark benchmark.cpp)
target_link_libraries(benchmark PRIVATE fheco)

file(MAKE_DIRECTORY ${CMAKE_BINARY_DIR}/benchmarks/unstruct_benc/he)

set(SOURCE_FILE_1 ${CMAKE_SOURCE_DIR}/benchmarks/unstruct_benc/he/main.cpp)
set(DESTINATION_FILE_1 ${CMAKE_BINARY_DIR}/benchmarks/unstruct_benc/he/main.cpp)
set(SOURCE_FILE_2 ${CMAKE_SOURCE_DIR}/benchmarks/unstruct_benc/he/utils.cpp)
set(DESTINATION_FILE_2 ${CMAKE_BINARY_DIR}/benchmarks/unstruct_benc/he/utils.cpp)
set(SOURCE_FILE_3 ${CMAKE_SOURCE_DIR}/benchmarks/unstruct_benc/he/utils.hpp)
set(DESTINATION_FILE_3 ${CMAKE_BINARY_DIR}/benchmarks/unstruct_benc/he/utils.hpp)
set(SOURCE_FILE_4 ${CMAKE_SOURCE_DIR}/benchmarks/unstruct_benc/he/CMakeLists.txt)
set(DESTINATION_FILE_4 ${CMAKE_BINARY_DIR}/benchmarks/unstruct_benc/he/CMakeLists.txt)

add_custom_target(copy_my_file_1_unsturct_ben ALL
    COMMAND ${CMAKE_COMMAND} -E copy ${SOURCE_FILE_1} ${DESTINATION_FILE_1}
    DEPENDS ${SOURCE_FILE_1}
    COMMENT "Copying ${SOURCE_FILE_1} to ${DESTINATION_FILE_1}"
)

add_custom_target(copy_my_file_2_unsturct_ben ALL
    COMMAND ${CMAKE_COMMAND} -E copy ${SOURCE_FILE_2} ${DESTINATION_FILE_2}
    DEPENDS ${SOURCE_FILE_2}
    COMMENT "Copying ${SOURCE_FILE_2} to ${DESTINATION_FILE_2}"
)

add_custom_target(copy_my_file_3_unsturct_ben ALL
    COMMAND ${CMAKE_COMMAND} -E copy ${SOURCE_FILE_3} ${DESTINATION_FILE_3}
    DEPENDS ${SOURCE_FILE_3}
    COMMENT "Copying ${SOURCE_FILE_3} to ${DESTINATION_FILE_3}"
)

add_custom_target(copy_my_file_4_unsturct_ben ALL
    COMMAND ${CMAKE_COMMAND} -E copy ${SOURCE_FILE_4} ${DESTINATION_FILE_4}
    DEPENDS ${SOURCE_FILE_4}
    COMMENT "Copying ${SOURCE_FILE_4} to ${DESTINATION_FILE_4}"
)
