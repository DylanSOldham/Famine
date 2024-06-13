cmake_minimum_required(VERSION 3.27.0)

project(faminecore-desktop)

find_package(GLEW REQUIRED)
find_package(glfw3 REQUIRED)

add_library(faminecore-desktop SHARED window.cpp)
target_link_libraries(faminecore-desktop glfw GLEW::GLEW)

install(TARGETS faminecore-desktop DESTINATION .)