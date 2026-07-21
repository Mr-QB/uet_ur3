from launch import LaunchDescription
from launch.actions import IncludeLaunchDescription, DeclareLaunchArgument
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration, PathJoinSubstitution
from launch_ros.substitutions import FindPackageShare

def generate_launch_description():
    declared_arguments = []
    
    # Declare arguments that we can pass to ur_sim_control
    declared_arguments.append(
        DeclareLaunchArgument("ur_type", default_value="ur3e", description="UR robot type")
    )
    declared_arguments.append(
        DeclareLaunchArgument("launch_rviz", default_value="true", description="Launch RViz?")
    )
    declared_arguments.append(
        DeclareLaunchArgument(
            "controllers_file",
            default_value="ur3_susgrip_gazebo_controllers.yaml",
            description="YAML file with the controllers configuration.",
        )
    )

    ur_type = LaunchConfiguration("ur_type")
    launch_rviz = LaunchConfiguration("launch_rviz")
    controllers_file = LaunchConfiguration("controllers_file")

    # Include the Gazebo Simulation from UR packages
    gz_sim_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource([
            PathJoinSubstitution([FindPackageShare("ur_simulation_gz"), "launch", "ur_sim_control.launch.py"])
        ]),
        launch_arguments={
            "ur_type": ur_type,
            "description_package": "ur3_moveit_control",
            "description_file": "ur3_with_susgrip.urdf.xacro",
            "runtime_config_package": "ur3_moveit_control",
            "controllers_file": controllers_file,
            "launch_rviz": "false",  # We launch RViz with MoveIt instead
            "gazebo_gui": "true",
        }.items()
    )

    # Include MoveIt launch
    moveit_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource([
            PathJoinSubstitution([FindPackageShare("ur_moveit_config"), "launch", "ur_moveit.launch.py"])
        ]),
        launch_arguments={
            "ur_type": ur_type,
            "description_package": "ur3_moveit_control",
            "description_file": "ur3_with_susgrip.urdf.xacro",
            "use_sim_time": "true",
            "launch_rviz": "true",
        }.items()
    )

    # Start the custom action server once, without launching a second MoveIt
    # instance. This lets this top-level launch provide the complete system.
    control_node_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource([
            PathJoinSubstitution([
                FindPackageShare("ur3_moveit_control"),
                "launch",
                "ur3_demo_gripper.launch.py",
            ])
        ]),
        launch_arguments={
            "ur_type": ur_type,
            "use_sim_time": "true",
            "launch_rviz": "false",
            "launch_moveit": "false",
        }.items(),
    )

    return LaunchDescription(declared_arguments + [
        gz_sim_launch,
        moveit_launch,
        control_node_launch,
    ])
