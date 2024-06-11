cmake_minimum_required(VERSION 3.27.0)

project(faminecore-desktop)

add_library(faminecore-desktop SHARED window.cpp)
target_link_libraries(faminecore-desktop glfw)

install(TARGETS faminecore-desktop DESTINATION .)