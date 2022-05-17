find_package(ros_environment REQUIRED)

set(ROS_VERSION $ENV{ROS_VERSION})

if(${ROS_VERSION} EQUAL 1)
  cmake_minimum_required(VERSION 2.8.3)
  project(foxglove_msgs)

  # Default to C++11
  if(NOT CMAKE_CXX_STANDARD)
    set(CMAKE_CXX_STANDARD 11)
  endif()

  find_package(catkin REQUIRED
    COMPONENTS
    message_generation
    visualization_msgs
    geometry_msgs
  )

  file(GLOB _generated_ros1_schemas RELATIVE "${CMAKE_CURRENT_SOURCE_DIR}/ros1msg" ros1msg/*)
  add_message_files(
    DIRECTORY msg
    FILES
    ImageMarkerArray.msg
  )
  add_message_files(
    DIRECTORY ros1msg
    FILES
    ${_generated_ros1_schemas}
  )

  generate_messages(DEPENDENCIES visualization_msgs geometry_msgs)

  catkin_package(CATKIN_DEPENDS message_runtime)

elseif(${ROS_VERSION} EQUAL 2)
  cmake_minimum_required(VERSION 3.5)
  project(foxglove_msgs)

  if(NOT CMAKE_CXX_STANDARD)
    set(CMAKE_CXX_STANDARD_REQUIRED ON)
    set(CMAKE_CXX_STANDARD 14)
  endif()

  if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
    add_compile_options(-Wall -Wextra -Wpedantic)
  endif()

  find_package(ament_cmake REQUIRED)
  find_package(visualization_msgs REQUIRED)
  find_package(geometry_msgs REQUIRED)
  find_package(rosidl_default_generators REQUIRED)

  file(GLOB _generated_ros2_schemas RELATIVE "${CMAKE_CURRENT_SOURCE_DIR}" ros2msg/*)
  rosidl_generate_interfaces(${PROJECT_NAME}
    msg/ImageMarkerArray.msg
    ${_generated_ros2_schemas}
    DEPENDENCIES visualization_msgs geometry_msgs
    ADD_LINTER_TESTS
  )

  ament_export_dependencies(rosidl_default_runtime)

  if(BUILD_TESTING)
    find_package(ament_lint_auto REQUIRED)
    ament_lint_auto_find_test_dependencies()
  endif()

  ament_package()

endif()
