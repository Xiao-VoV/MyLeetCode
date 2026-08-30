cmake_minimum_required(VERSION 3.16)
project(leetcode-cpp LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# 自动收集 src 下所有 .cpp 文件
file(GLOB_RECURSE SOURCES "src/*.cpp")

# 编译为库，这样每个文件都能获得语法提示
add_library(leetcode_solutions ${SOURCES})