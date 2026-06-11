import os
from launch import LaunchDescription
from launch.actions import OpaqueFunction, DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration, PathJoinSubstitution, Command, FindExecutable
from launch_ros.actions import Node
from launch_ros.substitutions import FindPackageShare
from launch_ros.parameter_descriptions import ParameterValue
import yaml
from ament_index_python.packages import get_package_share_directory

def load_yaml(package_name, file_path):
    try:
        package_path = get_package_share_directory(package_name)
        absolute_path = os.path.join(package_path, file_path)
        with open(absolute_path, "r") as file:
            return yaml.safe_load(file)
    except Exception:
        return None

def launch_setup(context, *args, **kwargs):
    ur_type = LaunchConfiguration("ur_type")
    
    # 1. Load Robot Model (URDF/Xacro)
    robot_description_content = Command([
        PathJoinSubstitution([FindExecutable(name="xacro")]), " ",
        PathJoinSubstitution([FindPackageShare("ur_description"), "urdf", "ur.urdf.xacro"]), " ",
        "ur_type:=", ur_type, " ",
        "sim_ignition:=true", " ",
        "name:=", "ur",
    ])
    robot_description = {"robot_description": ParameterValue(robot_description_content, value_type=str)}

    # 2. Load Semantic Model (SRDF)
    robot_description_semantic_content = Command([
        PathJoinSubstitution([FindExecutable(name="xacro")]), " ",
        PathJoinSubstitution([FindPackageShare("ur_moveit_config"), "srdf", "ur.srdf.xacro"]), " ",
        "name:=", "ur",
    ])
    robot_description_semantic = {"robot_description_semantic": robot_description_semantic_content}

    # 3. Load Kinematics and Joint Limits configurations
    kinematics_yaml = {"robot_description_kinematics": load_yaml("ur3_moveit_control", "config/kinematics.yaml")}
    joint_limits_yaml = {"robot_description_planning": load_yaml("ur3_moveit_control", "config/joint_limits.yaml")}
    custom_config_yaml = load_yaml("ur3_moveit_control", "config/moveit_custom_config.yaml")

    # 4. Load controllers configuration
    controllers_yaml = load_yaml("ur3_moveit_control", "config/moveit_controllers.yaml")

    # 5. Load planning pipelines and merge with OMPL planner configs
    planning_pipelines_yaml = load_yaml("ur3_moveit_control", "config/planning_pipelines.yaml")
    ompl_planning_yaml = load_yaml("ur3_moveit_control", "config/ompl_planning.yaml")
    if planning_pipelines_yaml and ompl_planning_yaml:
        planning_pipelines_yaml["planning_pipelines"]["ompl"].update(ompl_planning_yaml)

    # 6. Load planning scene monitor configurations
    planning_scene_monitor_yaml = load_yaml("ur3_moveit_control", "config/planning_scene_monitor.yaml")

    # 7. Launch our custom demo C++ node
    ur3_demo_node = Node(
        package="ur3_moveit_control",
        executable="ur3_demo_node",
        output="screen",
        parameters=[
            robot_description,
            robot_description_semantic,
            kinematics_yaml,
            joint_limits_yaml,
            planning_pipelines_yaml,
            controllers_yaml,
            planning_scene_monitor_yaml,
            custom_config_yaml,
            {"use_sim_time": True}
        ],
    )

    return [ur3_demo_node]

def generate_launch_description():
    declared_arguments = [
        DeclareLaunchArgument("ur_type", default_value="ur3e", description="Type of UR robot")
    ]
    return LaunchDescription(declared_arguments + [OpaqueFunction(function=launch_setup)])
