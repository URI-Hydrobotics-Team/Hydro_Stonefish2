# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target stonefish_ros2::stonefish_ros2
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${stonefish_ros2_TARGETS}.
if(stonefish_ros2_TARGETS AND NOT TARGET stonefish_ros2::stonefish_ros2)
  add_library(stonefish_ros2::stonefish_ros2 INTERFACE IMPORTED)
  set_target_properties(stonefish_ros2::stonefish_ros2 PROPERTIES
    INTERFACE_LINK_LIBRARIES "${stonefish_ros2_TARGETS}")
endif()
