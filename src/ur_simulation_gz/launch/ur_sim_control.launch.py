# Copyright (c) 2021 Stogl Robotics Consulting UG (haftungsbeschränkt)
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions are met:
#
#    * Redistributions of source code must retain the above copyright
#      notice, this list of conditions and the following disclaimer.
#
#    * Redistributions in binary form must reproduce the above copyright
#      notice, this list of conditions and the following disclaimer in the
#      documentation and/or other materials provided with the distribution.
#
#    * Neither the name of the {copyright_holder} nor the names of its
#      contributors may be used to endorse or promote products derived from
#      this software without specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
# AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
# IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
# ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
# LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
# CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
# SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
# INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
# CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
# ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
# POSSIBILITY OF SUCH DAMAGE.
#
# Author: Denis Stogl

from launch import LaunchDescription
from launch.actions import (
    DeclareLaunchArgument,
    ExecuteProcess,
    IncludeLaunchDescription,
    OpaqueFunction,
    RegisterEventHandler,
    TimerAction,
)
from launch.conditions import IfCondition, UnlessCondition
from launch.event_handlers import OnProcessExit
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import (
    Command,
    FindExecutable,
    LaunchConfiguration,
    PathJoinSubstitution,
)
from launch_ros.actions import Node
from launch_ros.parameter_descriptions import ParameterValue
from launch_ros.substitutions import FindPackageShare


def launch_setup(context, *args, **kwargs):
    # Gazebo layout (metres, expressed in the Gazebo world frame).
    # The table top is centred at z=0.75 and is 0.05 m thick, therefore its
    # upper surface is at z=0.775.  The robot is mounted at the middle of the
    # table's -X edge. The bottle is placed at world X=0.62 m: far enough for
    # a clean horizontal side grasp while retaining reach and IK margin. Its
    # model origin stays 0.05 m above the bottom for compatibility with the
    # existing pick coordinates.
    table_x = 0.5
    table_y = 0.0
    table_z = 0.0
    table_surface_z = 0.775

    robot_x = 0.0
    robot_y = 0.0
    robot_z = table_surface_z

    bottle_origin_height = 0.05
    bottle_x = 0.62
    bottle_y = 0.0
    bottle_z = table_surface_z + bottle_origin_height

    # Initialize Arguments
    ur_type = LaunchConfiguration("ur_type")
    safety_limits = LaunchConfiguration("safety_limits")
    safety_pos_margin = LaunchConfiguration("safety_pos_margin")
    safety_k_position = LaunchConfiguration("safety_k_position")
    # General arguments
    runtime_config_package = LaunchConfiguration("runtime_config_package")
    controllers_file = LaunchConfiguration("controllers_file")
    description_package = LaunchConfiguration("description_package")
    description_file = LaunchConfiguration("description_file")
    prefix = LaunchConfiguration("prefix")
    start_joint_controller = LaunchConfiguration("start_joint_controller")
    initial_joint_controller = LaunchConfiguration("initial_joint_controller")
    launch_rviz = LaunchConfiguration("launch_rviz")
    gazebo_gui = LaunchConfiguration("gazebo_gui")
    world_file = LaunchConfiguration("world_file")

    initial_joint_controllers = PathJoinSubstitution(
        [FindPackageShare(runtime_config_package), "config", controllers_file]
    )

    rviz_config_file = PathJoinSubstitution(
        [FindPackageShare(description_package), "rviz", "view_robot.rviz"]
    )

    robot_description_content = Command(
        [
            PathJoinSubstitution([FindExecutable(name="xacro")]),
            " ",
            PathJoinSubstitution(
                [FindPackageShare(description_package), "urdf", description_file]
            ),
            " ",
            "safety_limits:=",
            safety_limits,
            " ",
            "safety_pos_margin:=",
            safety_pos_margin,
            " ",
            "safety_k_position:=",
            safety_k_position,
            " ",
            "name:=",
            "ur",
            " ",
            "ur_type:=",
            ur_type,
            " ",
            "prefix:=",
            prefix,
            " ",
            "sim_ignition:=true",
            " ",
            "simulation_controllers:=",
            initial_joint_controllers,
        ]
    )
    table_file = PathJoinSubstitution(
    [
        FindPackageShare("ur3_moveit_control"),
        "models",
        "table.sdf",
    ]
)

    bottle_file = PathJoinSubstitution(
    [
        FindPackageShare("ur3_moveit_control"),
        "models",
        "bottle.sdf",
    ]
)
    robot_description = {
        "robot_description": ParameterValue(robot_description_content, value_type=str)
    }

    robot_state_publisher_node = Node(
        package="robot_state_publisher",
        executable="robot_state_publisher",
        output="both",
        parameters=[{"use_sim_time": True}, robot_description],
    )

    rviz_node = Node(
        package="rviz2",
        executable="rviz2",
        name="rviz2",
        output="log",
        arguments=["-d", rviz_config_file],
        condition=IfCondition(launch_rviz),
    )

    joint_state_broadcaster_spawner = Node(
        package="controller_manager",
        executable="spawner",
        arguments=["joint_state_broadcaster", "--controller-manager", "/controller_manager"],
    )

    # Delay rviz start after `joint_state_broadcaster`
    delay_rviz_after_joint_state_broadcaster_spawner = RegisterEventHandler(
        event_handler=OnProcessExit(
            target_action=joint_state_broadcaster_spawner,
            on_exit=[rviz_node],
        ),
        condition=IfCondition(launch_rviz),
    )

    gripper_controller_spawner = Node(
    package="controller_manager",
    executable="spawner",
    arguments=[
        "gripper_controller",
        "--controller-manager",
        "/controller_manager",
        "--controller-manager-timeout",
        "60",
    ],
    output="screen",
    )
    # There may be other controllers of the joints, but this is the initially-started one
    initial_joint_controller_spawner_started = Node(
        package="controller_manager",
        executable="spawner",
        arguments=[initial_joint_controller, "-c", "/controller_manager"],
        condition=IfCondition(start_joint_controller),
    )
    initial_joint_controller_spawner_stopped = Node(
        package="controller_manager",
        executable="spawner",
        arguments=[initial_joint_controller, "-c", "/controller_manager", "--stopped"],
        condition=UnlessCondition(start_joint_controller),
    )

    # Spawn topic-based controllers in stopped state
    forward_position_controller_spawner = Node(
        package="controller_manager",
        executable="spawner",
        arguments=["forward_position_controller", "-c", "/controller_manager", "--stopped"],
    )

    forward_velocity_controller_spawner = Node(
        package="controller_manager",
        executable="spawner",
        arguments=["forward_velocity_controller", "-c", "/controller_manager", "--stopped"],
    )

    # GZ nodes
    gz_spawn_entity = Node(
    package="ros_gz_sim",
    executable="create",
    output="screen",
    arguments=[
        "-string", robot_description_content,
        "-name", "ur",
        "-allow_renaming", "true",
        "-x", str(robot_x),
        "-y", str(robot_y),
        "-z", str(robot_z),
    ],
)
    gz_spawn_table = Node(
    package="ros_gz_sim",
    executable="create",
    output="screen",
    arguments=[
        "-file",
        table_file,
        "-name",
        "table",
        "-x",
        str(table_x),
        "-y",
        str(table_y),
        "-z",
        str(table_z),
    ],
)
    # Always run the simulation server on its own. Starting server and GUI in
    # the same ign process occasionally leaves MinimalScene connected before
    # the world scene service is ready, producing a white window even though
    # physics and ros2_control are healthy.
    gz_server_launch_description = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            [FindPackageShare("ros_gz_sim"), "/launch/gz_sim.launch.py"]
        ),
        launch_arguments={"gz_args": [" -s -r -v 2 ", world_file]}.items(),
    )

    gz_gui_process = ExecuteProcess(
        cmd=["ign", "gazebo", "-g", "-v", "2"],
        output="screen",
        condition=IfCondition(gazebo_gui),
    )

    start_gz_gui_after_server = TimerAction(
        period=2.0,
        actions=[gz_gui_process],
    )

    gz_spawn_box = Node(
    package="ros_gz_sim",
    executable="create",
    output="screen",
    arguments=[
        "-file",
        bottle_file,
        "-name",
        "pick_box",
        "-x",
        str(bottle_x),
        "-y",
        str(bottle_y),
        "-z",
        str(bottle_z),
    ],
)

    # Insert world entities one at a time. The create service acknowledges a
    # queued request before Gazebo's update loop has necessarily applied it;
    # firing table and bottle requests concurrently made startup intermittent.
    spawn_box_after_table = RegisterEventHandler(
        event_handler=OnProcessExit(
            target_action=gz_spawn_table,
            on_exit=[TimerAction(period=1.5, actions=[gz_spawn_box])],
        )
    )

    # DetachableJoint is configured when the robot model is inserted. Its
    # child model must already exist at that moment, otherwise the plugin
    # cannot resolve pick_box::box_link and will not retry later. Give Gazebo
    # one update interval after the bottle create process before inserting UR.
    spawn_robot_after_box = RegisterEventHandler(
        event_handler=OnProcessExit(
            target_action=gz_spawn_box,
            on_exit=[TimerAction(period=2.0, actions=[gz_spawn_entity])],
        )
    )

    # ign_ros2_control creates /controller_manager only after the robot entity
    # and its Gazebo system plugin have finished loading. Start only the joint
    # state broadcaster after that point, then chain the remaining controller
    # spawners. Calling five controller-manager services concurrently can starve
    # the Gazebo update thread on a slow startup and make every spawner time out.
    spawn_controllers_after_robot = RegisterEventHandler(
        event_handler=OnProcessExit(
            target_action=gz_spawn_entity,
            on_exit=[
                TimerAction(
                    period=3.0,
                    actions=[joint_state_broadcaster_spawner],
                )
            ],
        )
    )

    spawn_joint_controller_after_joint_state = RegisterEventHandler(
        event_handler=OnProcessExit(
            target_action=joint_state_broadcaster_spawner,
            on_exit=[
                initial_joint_controller_spawner_stopped,
                initial_joint_controller_spawner_started,
            ],
        )
    )

    spawn_gripper_after_started_joint_controller = RegisterEventHandler(
        event_handler=OnProcessExit(
            target_action=initial_joint_controller_spawner_started,
            on_exit=[gripper_controller_spawner],
        ),
        condition=IfCondition(start_joint_controller),
    )

    spawn_gripper_after_stopped_joint_controller = RegisterEventHandler(
        event_handler=OnProcessExit(
            target_action=initial_joint_controller_spawner_stopped,
            on_exit=[gripper_controller_spawner],
        ),
        condition=UnlessCondition(start_joint_controller),
    )

    spawn_forward_position_after_gripper = RegisterEventHandler(
        event_handler=OnProcessExit(
            target_action=gripper_controller_spawner,
            on_exit=[forward_position_controller_spawner],
        )
    )

    spawn_forward_velocity_after_forward_position = RegisterEventHandler(
        event_handler=OnProcessExit(
            target_action=forward_position_controller_spawner,
            on_exit=[forward_velocity_controller_spawner],
        )
    )

    # Make the /clock topic available in ROS
    gz_sim_bridge = Node(
        package="ros_gz_bridge",
        executable="parameter_bridge",
        arguments=[
            "/clock@rosgraph_msgs/msg/Clock[ignition.msgs.Clock",
            # ROS publishes Empty; Gazebo's DetachableJoint subscribes to it.
            "/pick_box/attach@std_msgs/msg/Empty]ignition.msgs.Empty",
            "/pick_box/detach@std_msgs/msg/Empty]ignition.msgs.Empty",
            # Ignition Gazebo 6 publishes the words "attached" / "detached"
            # from DetachableJoint.  Keep the transport type exact; bridging
            # this as Boolean silently prevents the control node from ever
            # receiving attachment confirmation.
            "/pick_box/attachment_state@std_msgs/msg/String[ignition.msgs.StringMsg",
        ],
        output="screen",
    )

    nodes_to_start = [
        robot_state_publisher_node,
        delay_rviz_after_joint_state_broadcaster_spawner,
        gz_server_launch_description,
        start_gz_gui_after_server,
        gz_sim_bridge,
        gz_spawn_table,
        spawn_box_after_table,
        spawn_robot_after_box,
        spawn_controllers_after_robot,
        spawn_joint_controller_after_joint_state,
        spawn_gripper_after_started_joint_controller,
        spawn_gripper_after_stopped_joint_controller,
        spawn_forward_position_after_gripper,
        spawn_forward_velocity_after_forward_position,
    ]

    return nodes_to_start


def generate_launch_description():
    declared_arguments = []
    # UR specific arguments
    declared_arguments.append(
        DeclareLaunchArgument(
            "ur_type",
            description="Type/series of used UR robot.",
            choices=[
                "ur3",
                "ur5",
                "ur10",
                "ur3e",
                "ur5e",
                "ur7e",
                "ur10e",
                "ur12e",
                "ur16e",
                "ur8long",
                "ur15",
                "ur18",
                "ur20",
                "ur30",
            ],
            default_value="ur5e",
        )
    )
    declared_arguments.append(
        DeclareLaunchArgument(
            "safety_limits",
            default_value="true",
            description="Enables the safety limits controller if true.",
        )
    )
    declared_arguments.append(
        DeclareLaunchArgument(
            "safety_pos_margin",
            default_value="0.15",
            description="The margin to lower and upper limits in the safety controller.",
        )
    )
    declared_arguments.append(
        DeclareLaunchArgument(
            "safety_k_position",
            default_value="20",
            description="k-position factor in the safety controller.",
        )
    )
    # General arguments
    declared_arguments.append(
        DeclareLaunchArgument(
            "runtime_config_package",
            default_value="ur_simulation_gz",
            description='Package with the controller\'s configuration in "config" folder. \
        Usually the argument is not set, it enables use of a custom setup.',
        )
    )
    declared_arguments.append(
        DeclareLaunchArgument(
            "controllers_file",
            default_value="ur_controllers.yaml",
            description="YAML file with the controllers configuration.",
        )
    )
    declared_arguments.append(
        DeclareLaunchArgument(
            "description_package",
            default_value="ur3_moveit_control",
            description="Description package with robot URDF/XACRO files. Usually the argument \
        is not set, it enables use of a custom description.",
        )
    )
    declared_arguments.append(
        DeclareLaunchArgument(
            "description_file",
            default_value="ur3_with_susgrip.urdf.xacro",
            description="URDF/XACRO description file with the robot.",
        )
    )
    declared_arguments.append(
        DeclareLaunchArgument(
            "prefix",
            default_value='""',
            description="Prefix of the joint names, useful for \
        multi-robot setup. If changed than also joint names in the controllers' configuration \
        have to be updated.",
        )
    )
    declared_arguments.append(
        DeclareLaunchArgument(
            "start_joint_controller",
            default_value="true",
            description="Enable headless mode for robot control",
        )
    )
    declared_arguments.append(
        DeclareLaunchArgument(
            "initial_joint_controller",
            default_value="joint_trajectory_controller",
            description="Robot controller to start.",
        )
    )
    declared_arguments.append(
        DeclareLaunchArgument("launch_rviz", default_value="false", description="Launch RViz?")
    )
    declared_arguments.append(
        DeclareLaunchArgument(
            "gazebo_gui", default_value="true", description="Start gazebo with GUI?"
        )
    )
    declared_arguments.append(
        DeclareLaunchArgument(
            "world_file",
            default_value=PathJoinSubstitution(
                [FindPackageShare("ur_simulation_gz"), "worlds", "fast_empty.sdf"]
            ),
            description=(
                "Gazebo world file. The default uses a 2 ms physics step and "
                "targets RTF 2.0; pass empty.sdf to restore the stock RTF 1 world."
            ),
        )
    )

    return LaunchDescription(declared_arguments + [OpaqueFunction(function=launch_setup)])
