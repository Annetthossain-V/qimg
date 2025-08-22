cmake_minimum_required(VERSION 3.10)
project(qimg LANGUAGES CXX)

file(GLOB SRC_FILES "${CMAKE_SOURCE_DIR}/src/*.cxx")

add_executable(${PROJECT_NAME} ${SRC_FILES})

include_directories($ENV{HOME}/include/opencv4)
link_directories($ENV{HOME}/lib/opencv4)

file(GLOB OPENCV_LIBS "$ENV{HOME}/lib/opencv4/*.so")
target_link_libraries(${PROJECT_NAME} ${OPENCV_LIBS})

target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_11)
