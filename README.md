# UET UR3 Robotic Manipulation System

A ROS 2 Humble software framework for trajectory planning, simulation, and hardware control of the **Universal Robots UR3 / UR3e** robotic arm integrated with the **SUSGrip 2F** adaptive two-finger gripper.

---

## System Overview

This repository provides an end-to-end integration for the UR3 robotic manipulator. It supports both high-fidelity physics simulation via Gazebo (Ignition) and physical hardware control using the official Universal Robots ROS 2 Driver and MoveIt 2 framework.

### Package Architecture

| Package / Module | Description |
| :--- | :--- |
| **`Universal_Robots_ROS2_Driver`** | Official ROS 2 hardware driver for Universal Robots arms. Manages low-level communication, Dashboard Server protocol, and state streaming. |
| **`susgrip_2f`** | Meta-package for the SUSGrip 2-finger adaptive gripper containing:<br>- `susgrip_2f_description`: URDF/Xacro kinematic models and 3D mesh files.<br>- `susgrip_2f_control`: Controller configurations for gripper actuation.<br>- `susgrip_2f_hardware`: Hardware interface over RS-485/USB serial protocol.<br>- `susgrip_2f_gazebo`: Gazebo physics plugin configurations. |
| **`ur3_moveit_control`** | Trajectory planning nodes, RViz 2 visualization setups, and Eye-in-Hand camera calibration scripts utilizing MoveIt 2. |
| **`ur_simulation_gz`** | Ignition Gazebo simulation launch scripts and world configurations for UR arms. |

---

## System Requirements

- **Operating System:** Ubuntu 22.04 LTS (Jammy Jellyfish)
- **ROS Distribution:** ROS 2 Humble Hawksbill
- **Build System:** `colcon-core`, `cmake` (>= 3.16), `ament_cmake`

---

## Dependencies & Prerequisites

Before building the workspace, ensure ROS 2 Humble base and all required control and motion planning dependencies are installed.

### System Package Installation

Run the following command to install required ROS 2 dependencies:

```bash
sudo apt update
sudo apt install -y \
  ros-humble-desktop \
  ros-humble-moveit \
  ros-humble-moveit-servo \
  ros-humble-ros2-control \
  ros-humble-ros2-controllers \
  ros-humble-controller-interface \
  ros-humble-gripper-controllers \
  ros-humble-joint-state-broadcaster \
  ros-humble-position-controllers \
  ros-humble-aruco-ros \
  ros-humble-ur-msgs \
  ros-humble-ur-client-library \
  ros-humble-ur-description \
  ros-humble-ros-gz \
  ros-humble-ign-ros2-control
```

### Automatic Dependency Resolution via `rosdep`

Navigate to your workspace root and run `rosdep` to resolve missing dependencies declared in `package.xml` files:

```bash
sudo rosdep init 2>/dev/null || true
rosdep update
cd ~/ur3_ws
rosdep install --from-paths src --ignore-src -y
```

---

## Build and Sourcing Instructions

### 1. Environment Setup

Always source the main ROS 2 installation prior to building:

```bash
source /opt/ros/humble/setup.bash
```

To automatically source ROS 2 upon opening new terminal sessions:

```bash
echo "source /opt/ros/humble/setup.bash" >> ~/.bashrc
source ~/.bashrc
```

### 2. Workspace Compilation

Navigate to the workspace root directory and build using `colcon`:

```bash
cd ~/ur3_ws
colcon build --symlink-install
```

### 3. Sourcing Workspace Overlay

After a successful compilation, source the local overlay setup script:

```bash
source install/setup.bash
```

---

## Execution Guide

### 1. Simulation Environment (Gazebo)

This mode runs the physics simulation in Gazebo with simulated time (`use_sim_time:=true`).

#### Terminal 1: Launch Gazebo Simulation
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur_simulation_gz ur_sim_control.launch.py ur_type:=ur3e
```

#### Terminal 2: Launch MoveIt Server and Control Demo Node
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur3_moveit_control ur3_demo.launch.py ur_type:=ur3e use_sim_time:=true
```

---

### 2. Real Robot Hardware Execution

This mode executes motion trajectories on physical UR3 / UR3e hardware over Ethernet (`use_sim_time:=false`).

#### Mandatory Teach Pendant Setup
1. Set the robot state to **Remote Control** mode on the Teach Pendant.
2. Open the control program containing the **External Control** node.
3. Verify that the PC IP address is configured correctly in the External Control settings.
4. Press the **Play** button on the Teach Pendant to initiate the control loop.

#### Terminal 1: Launch Universal Robots Hardware Driver
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur_robot_driver ur_control.launch.py \
  ur_type:=ur3e \
  robot_ip:=192.168.1.10 \
  launch_rviz:=false
```

#### Terminal 2: Launch MoveIt Server and Execution Node
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur3_moveit_control ur3_demo.launch.py \
  ur_type:=ur3e \
  use_sim_time:=false
```

---

### 3. Concurrent Arm and SUSGrip Gripper Hardware Control

To operate both the physical UR3 arm and the physical SUSGrip gripper simultaneously:

#### Terminal 1: Launch UR Driver
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur_robot_driver ur_control.launch.py \
  ur_type:=ur3e \
  robot_ip:=192.168.1.10 \
  launch_rviz:=false
```

#### Terminal 2: Launch MoveIt Server and RViz 2
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur3_moveit_control ur3_demo.launch.py \
  ur_type:=ur3e \
  use_sim_time:=false
```

#### Terminal 3: Launch SUSGrip Hardware Interface
Connect the USB-to-RS485 adapter and set serial permissions:
```bash
sudo chmod 666 /dev/ttyUSB0
source ~/ur3_ws/install/setup.bash
ros2 run susgrip_2f_hardware hardware_interface --ros-args -p serial_port:=/dev/ttyUSB0 -r /susgrip/joint_states:=/joint_states
```

---

### 4. Eye-in-Hand Camera Calibration

Computes the homogeneous transformation matrix between the end-effector (`tool0`) and the camera (`camera_link`) using OpenCV hand-eye calibration algorithms.

#### Terminal 1: Launch RealSense Camera
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch realsense2_camera rs_launch.py align_depth.enable:=true initial_reset:=true
```

#### Terminal 2: Launch UR Robot Driver
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur_robot_driver ur_control.launch.py \
  ur_type:=ur3e \
  robot_ip:=192.168.1.10 \
  launch_rviz:=false
```

#### Terminal 3: Launch ArUco Marker Detection
```bash
source ~/ur3_ws/install/setup.bash
ros2 launch ur3_moveit_control eye_in_hand_calib.launch.py
```

#### Terminal 4: Run Calibration Script
```bash
source ~/ur3_ws/install/setup.bash
python3 ~/ur3_ws/src/uet_ur3/ur3_moveit_control/scripts/realsense_calib_eye_in_hand.py
```

- Press `Enter` to collect samples at different arm poses (minimum 5 poses required).
- Press `c` + `Enter` to compute the hand-eye transformation matrix.
- Press `q` + `Enter` to exit.

---

## Configuration Launch Arguments

Arguments available for `ur3_demo.launch.py`:

| Argument | Default | Description |
| :--- | :--- | :--- |
| `ur_type` | `ur3e` | Robot model series (`ur3`, `ur3e`, `ur5`, `ur5e`). |
| `use_sim_time` | `true` | Set to `false` when executing on physical hardware. |
| `launch_rviz` | `true` | Toggle RViz 2 visualization GUI. |
| `launch_moveit` | `true` | Toggle automatic launch of MoveIt Planning Server. |

---
