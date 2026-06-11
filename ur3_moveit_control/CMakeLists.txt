cmake_minimum_required(VERSION 3.8)
project(ur3_moveit_control)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(moveit_ros_planning_interface REQUIRED)
find_package(geometry_msgs REQUIRED)

include_directories(include)

add_library(ur3_motion_interface
  src/ur3_motion_interface.cpp
)

ament_target_dependencies(ur3_motion_interface
  rclcpp
  moveit_ros_planning_interface
  geometry_msgs
)

add_executable(ur3_demo_node
  src/ur3_demo_node.cpp
)

target_link_libraries(ur3_demo_node
  ur3_motion_interface
)

ament_target_dependencies(ur3_demo_node
  rclcpp
  moveit_ros_planning_interface
  geometry_msgs
)

install(
  TARGETS
    ur3_motion_interface
    ur3_demo_node
  ARCHIVE DESTINATION lib
  LIBRARY DESTINATION lib
  RUNTIME DESTINATION lib/${PROJECT_NAME}
)

install(
  DIRECTORY include/
  DESTINATION include
)

ament_package()