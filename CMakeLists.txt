cmake_minimum_required(VERSION 2.6)
include(ExternalProject)

project(BitDMX C CXX)


# Download and compile external libraries
set(EXTERNAL_INSTALL_LOCATION ${CMAKE_BINARY_DIR}/lib)
ExternalProject_Add(
    json
    GIT_REPOSITORY git@github.com:nlohmann/json.git
    CMAKE_ARGS -DBuildTests=OFF -DCMAKE_INSTALL_PREFIX=${EXTERNAL_INSTALL_LOCATION}
)


#### BitDMX Binary ####
# Set name and source path
set(EXECUTABLE_NAME ${PROJECT_NAME})
set(SOURCE_PATH ${PROJECT_SOURCE_DIR}/src)

# Collect sources and create executable target
file(GLOB BITDMX_SRC ${SOURCE_PATH}/*.hpp ${SOURCE_PATH}/*.cpp)
add_executable(${EXECUTABLE_NAME} ${BITDMX_SRC})

# Link against JSON library
include_directories(${EXTERNAL_INSTALL_LOCATION}/include/nlohmann)
add_dependencies(${EXECUTABLE_NAME} json)

# Link against external SFML
set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake_modules" ${CMAKE_MODULE_PATH})
find_package(SFML 2 REQUIRED system window graphics network audio)
if(SFML_FOUND)
  include_directories(${SFML_INCLUDE_DIR})
  target_link_libraries(${EXECUTABLE_NAME} ${SFML_LIBRARIES})
endif()

# Install target
install(TARGETS ${EXECUTABLE_NAME} DESTINATION bin)

