include(FetchContent)

FetchContent_Declare(
  pybind11
  GIT_REPOSITORY https://github.com/pybind/pybind11.git
  GIT_TAG        v2.13.6
)
FetchContent_MakeAvailable(pybind11)

include_directories(
        ${INCLUDE_DIRECTORIES}
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${CMAKE_CURRENT_SOURCE_DIR}/../../include
)

pybind11_add_module(dashbls_py ${CMAKE_CURRENT_SOURCE_DIR}/pythonbindings.cpp)
set_target_properties(dashbls_py PROPERTIES OUTPUT_NAME dashbls)
target_link_libraries(dashbls_py PRIVATE dashbls)
