# cmake_minimum_required(VERSION 3.12)
# project(squeakr)

# set(CMAKE_C_STANDARD 11)
# set(CMAKE_C_STANDARD_REQUIRED ON)

# set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# set(SRC_DIR src)
# file(GLOB SRCS "${SRC_DIR}/*.c")
# add_library(squeakr SHARED ${SRCS})

# target_link_libraries(squeakr m)

# install(TARGETS squeakr LIBRARY DESTINATION lib)


cmake_minimum_required(VERSION 3.12)
project(squeakr)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

set(CMAKE_POSITION_INDEPENDENT_CODE ON)

set(SRC_DIR src)
file(GLOB SRCS "${SRC_DIR}/*.c")
add_library(squeakr STATIC ${SRCS})

install(TARGETS squeakr ARCHIVE DESTINATION lib)