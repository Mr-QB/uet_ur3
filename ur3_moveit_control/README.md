# `ur3_moveit_control` Package Documentation

The `ur3_moveit_control` package provides MoveIt 2 motion planning interfaces, ROS 2 C++ action servers, specialized launch configurations, and an Eye-in-Hand camera calibration pipeline for the **Universal Robots UR3 / UR3e** manipulator.

---

## Package Architecture & Components

```
ur3_moveit_control/
├── action/
│   └── UR3Control.action       # Custom ROS 2 action definition
├── config/                     # MoveIt 2 kinematics and joint limit parameters
├── include/
│   └── ur3_moveit_control/     # C++ header files (ur3_motion_interface.hpp)
├── launch/
│   ├── ur3_demo.launch.py      # Primary MoveIt 2 demo launch script
│   ├── ur3_demo_gripper.launch.py # MoveIt 2 launch setup with SUSGrip 2F planning group
│   ├── ur3_susgrip_sim.launch.py  # Gazebo simulation launch setup for arm + gripper
│   └── eye_in_hand_calib.launch.py# ArUco marker detection launch script for calibration
├── rviz/                       # Pre-configured RViz 2 display configurations
├── scripts/
│   └── realsense_calib_eye_in_hand.py # Interactive OpenCV hand-eye calibration tool
└── src/
    ├── ur3_motion_interface.cpp# MoveIt 2 C++ Motion Planning API library wrapper
    ├── ur3_control_node.cpp    # ROS 2 Action Server execution node
    └── send_goal_client.py     # Python client script for action goal testing
```

---

## Build Instructions

Build this package individually within your ROS 2 workspace:

```bash
cd ~/ur3_ws
colcon build --packages-select ur3_moveit_control --symlink-install
source install/setup.bash
```

---

## Execution Guide

### 1. Motion Planning in Simulation (Gazebo)

#### Terminal 1: Launch Gazebo Physics Simulation
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur3_moveit_control ur3_susgrip_sim.launch.py ur_type:=ur3e
```

#### Terminal 2: Launch MoveIt 2 Server and Demo Node
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur3_moveit_control ur3_demo.launch.py ur_type:=ur3e use_sim_time:=true
```

---

### 2. Motion Execution on Physical Robot Hardware

#### Terminal 1: Launch Universal Robots Driver
Ensure External Control is active on the Teach Pendant before executing:
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur_robot_driver ur_control.launch.py ur_type:=ur3e robot_ip:=192.168.1.10 launch_rviz:=false
```

#### Terminal 2: Launch MoveIt 2 Execution Node
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur3_moveit_control ur3_demo.launch.py ur_type:=ur3e use_sim_time:=false
```

---

### 3. Combined Arm and SUSGrip Gripper Control

#### Terminal 1: Launch MoveIt Demo with Gripper Group
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur3_moveit_control ur3_demo_gripper.launch.py ur_type:=ur3e use_sim_time:=false
```

#### Terminal 2: Launch SUSGrip Hardware Interface
Grant serial port permissions and start hardware node:
```bash
sudo chmod 666 /dev/ttyUSB0
source ~/ur3_ws/install/setup.bash
ros2 run susgrip_2f_hardware hardware_interface --ros-args -p serial_port:=/dev/ttyUSB0 -r /susgrip/joint_states:=/joint_states
```

---

### 4. Eye-in-Hand Camera Calibration Workflow

This utility computes the 4x4 Homogeneous Transformation Matrix (`tool0` → `camera_link`) using OpenCV hand-eye calibration algorithms.

#### Terminal 1: Launch RealSense Camera Node
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch realsense2_camera rs_launch.py align_depth.enable:=true initial_reset:=true
```

#### Terminal 2: Launch ArUco Marker Detector
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur3_moveit_control eye_in_hand_calib.launch.py
```

#### Terminal 3: Run Interactive Calibration Script
```bash
source ~/ur3_ws/install/setup.bash
python3 ~/ur3_ws/src/uet_ur3/ur3_moveit_control/scripts/realsense_calib_eye_in_hand.py
```

- Press `Enter` to capture sample poses (minimum 5 distinct arm positions required).
- Press `c` + `Enter` to solve the calibration and generate static TF commands.
- Results are saved automatically into `~/ur3_ws/calib_results/`.

---

## Launch Arguments

Arguments for `ur3_demo.launch.py`:

| Argument | Default | Description |
| :--- | :--- | :--- |
| `ur_type` | `ur3e` | UR robot model variant (`ur3`, `ur3e`, `ur5`, `ur5e`). |
| `use_sim_time` | `true` | Toggles simulated clock (`true` for Gazebo, `false` for physical arm). |
| `launch_rviz` | `true` | Launches RViz 2 visualization interface. |
| `launch_moveit` | `true` | Starts the MoveIt Planning Pipeline (`ur_moveit.launch.py`). |

---

## ROS 2 Action API (`UR3Control.action`)

The `ur3_control_node` implements an action server interface defined in `action/UR3Control.action`.

### Action Interface Structure
- **Goal:** `geometry_msgs/Pose target_pose` or `float64[] joint_positions`.
- **Result:** `bool success`, `string message`.
- **Feedback:** `geometry_msgs/Pose current_pose`, `string status`.

### Running Test Action Client
```bash
ros2 run ur3_moveit_control send_goal_client.py
```
