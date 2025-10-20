cmake_minimum_required(VERSION 3.10)
project(qi_runtime VERSION 0.1.0 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Include directories
include_directories(include)

# Source files
set(RUNTIME_SOURCES
    src/memory.c
    src/strings.c
    src/errors.c
    src/platform.c
)

# Create static library
add_library(qi_runtime STATIC ${RUNTIME_SOURCES})

# Set output directory
set_target_properties(qi_runtime PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}
)

# Installation
install(TARGETS qi_runtime
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
)

install(DIRECTORY include/ DESTINATION include)
