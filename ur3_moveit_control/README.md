# UR3e MoveIt Control (uet_ur3)

This package provides control nodes and trajectory motion configurations for the UR3e robot arm using MoveIt 2 on ROS 2 Humble. The system supports both physical simulation (Gazebo) and real robot control.

---

## Installation & Building

Build the package inside your workspace before running:

```bash
cd ~/ros_ws
colcon build --packages-select ur3_moveit_control
source install/setup.bash
```

---

## 1. Operating in Simulation Environment (Gazebo)

This mode uses the Gazebo simulation environment along with simulated time synchronization (`use_sim_time:=true`).

### Terminal 1: Launch Gazebo Environment
Initialize the UR3e physics simulation in Gazebo:
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur_simulation_gz ur_sim_control.launch.py ur_type:=ur3e
```

### Terminal 2: Launch MoveIt Server and Control Node
This launches the MoveIt Planning Server, RViz 2 visualization interface, and executes the predefined trajectory:
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur3_moveit_control ur3_demo.launch.py \
  ur_type:=ur3e \
  use_sim_time:=true
```

---

## 2. Operating with Real Robot Hardware

This mode sends control commands directly to UR3e robot hardware via Ethernet connection (`use_sim_time:=false`).

### Terminal 1: Launch Robot Hardware Driver
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur_robot_driver ur_control.launch.py \
  ur_type:=ur3e \
  robot_ip:=192.168.1.10 \
  launch_rviz:=false
```

> [!IMPORTANT]
> **Mandatory Procedure on Robot Teach Pendant:**
> 1. Set the robot state to Remote Control mode.
> 2. Open the control program containing the External Control node (ensure the PC IP address is configured correctly).
> 3. Press the Play button on the Teach Pendant to start the connection program.
> 4. Verify successful connection in the Driver terminal log:
>    `[UR_Client_Library:]: Robot connected to reverse interface. Ready to receive control commands.`

### Terminal 2: Launch MoveIt Server and Control Node
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur3_moveit_control ur3_demo.launch.py \
  ur_type:=ur3e \
  use_sim_time:=false
```

---

## 3. Concurrent Operation of UR3e Arm and SusGrip Hardware

To control both the robot arm and physical SusGrip gripper simultaneously (with real-time state visualization in RViz):

### Terminal 1: Launch UR3e Driver
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur_robot_driver ur_control.launch.py \
  ur_type:=ur3e \
  robot_ip:=192.168.1.10 \
  launch_rviz:=false
```
*(Remember to activate External Control on the Teach Pendant).*

### Terminal 2: Launch MoveIt Server and RViz
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur3_moveit_control ur3_demo.launch.py \
  ur_type:=ur3e \
  use_sim_time:=false
```

### Terminal 3: Connect SusGrip Hardware & Sync RViz
Ensure the USB-to-RS485 cable is connected to your computer. Grant USB access permission:
```bash
sudo chmod 666 /dev/ttyUSB0
```
Run the gripper driver with topic remapping to allow proper feedback for repeated operations:
```bash
source ~/ros_ws/install/setup.bash
ros2 run susgrip_2f_hardware hardware_interface --ros-args -p serial_port:=/dev/ttyUSB0 -r /susgrip/joint_states:=/joint_states
```

> [!TIP]
> **Controlling Gripper in RViz:**
> 1. In **MotionPlanning**, under **Planning Group**, select **`gripper`**.
> 2. Switch to **Joints** tab, drag the slider to adjust open/close posture.
> 3. Go back to **Planning** tab, click **Plan** then **Execute**. The physical gripper will execute identical motion to RViz.

---

## Configuration Arguments in `ur3_demo.launch.py`

Customize system behavior by passing arguments to the launch file:

| Argument | Default Value | Description |
| :--- | :--- | :--- |
| `ur_type` | `ur3e` | Universal Robots model series (e.g., `ur3`, `ur3e`, `ur5`, `ur5e`). |
| `use_sim_time` | `true` | Time source selection (`false` for real hardware). |
| `launch_rviz` | `true` | Option to launch RViz 2 visualization tool. |
| `launch_moveit` | `true` | Option to automatically start MoveIt Planning Server (`ur_moveit.launch.py`). |

*Example command without RViz GUI:*
```bash
ros2 launch ur3_moveit_control ur3_demo.launch.py ur_type:=ur3e use_sim_time:=false launch_rviz:=false
```

---

## Troubleshooting

### 1. Action Goal Rejection: `Can't accept new action goals. Controller is not running.`
* **Cause:** `scaled_joint_trajectory_controller` on hardware driver is not active. Usually caused by interrupted connection or inactive External Control program on Teach Pendant.
* **Fix:** Check controller status using `ros2 control list_controllers`. Ensure External Control program is running and showing `active`.

### 2. Process Hangs on SIGINT (Ctrl+C)
* **Fix:** `ur3_demo_node` uses async thread execution for motion planning while main thread handles ROS 2 spin loop. On SIGINT, the node cleanly shuts down without hanging.

---

## 4. Eye-in-Hand Camera Calibration

This feature automatically computes the 4x4 Homogeneous Transformation Matrix from end-effector (`tool0`) to camera (`camera_link`) using OpenCV Hand-Eye Calibration algorithms (Tsai, Park, Horaud, Andreff, Daniilidis).

### Prerequisites

- RealSense camera connected (USB 3.0 Type-C recommended to prevent frame timeouts).
- Printed ArUco Marker placed on the table (Default ID: `26`, size: `0.1m`). **Do not move marker** during calibration.
- UR3e connected and TF `base_link` → `tool0` active.
- Package `aruco_ros` installed (`sudo apt-get install ros-humble-aruco-ros`).

### Terminal 1: Launch RealSense Camera
```bash
source ~/ros_ws/install/setup.bash
ros2 launch realsense2_camera rs_launch.py align_depth.enable:=true initial_reset:=true
```

> [!CAUTION]
> **DO NOT enable PointCloud2 color** during calibration. PointCloud2 consumes high CPU resources and can lock up the system. Standard RGB image stream is sufficient for ArUco detection.

### Terminal 2: Launch Robot Driver + MoveIt
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur_robot_driver ur_control.launch.py \
  ur_type:=ur3e \
  robot_ip:=192.168.1.10 \
  launch_rviz:=false
```

### Terminal 3: Launch ArUco Marker Detection
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur3_moveit_control eye_in_hand_calib.launch.py
```

### Terminal 4: Run Calibration Script
```bash
source ~/ros_ws/install/setup.bash
python3 ~/ros_ws/src/uet_ur3/ur3_moveit_control/scripts/realsense_calib_eye_in_hand.py
```

### Operational Keys

| Key | Function |
| :--- | :--- |
| `Enter` | Take 1 calibration sample (robot TF + marker pose) |
| `c` + `Enter` | Calculate calibration and print report |
| `q` + `Enter` | Exit program |

> [!IMPORTANT]
> **Sampling Procedure:**
> 1. Collect **at least 5 samples** at distinctly different robot arm postures.
> 2. Ensure camera clearly detects the ArUco Marker before pressing Enter.
> 3. After gathering sufficient samples, press `c` to compute. The script prints:
>    - Comparison table across 5 algorithms (X, Y, Z, Roll, Pitch, Yaw)
>    - 4x4 Transformation Matrix of the best method
>    - `static_transform_publisher` command ready to copy-paste.

### Sample Output

```
═══════════════════════════════════════════════════
  🏆 BEST METHOD RESULT: PARK
═══════════════════════════════════════════════════
--- HOMOGENEOUS TRANSFORMATION MATRIX 4x4 (tool0 → camera_link) ---
  [ -0.012950  -0.999833  -0.012911  -0.043398 ]
  [ +0.032546  +0.012483  -0.999392  -0.107180 ]
  [ +0.999386  -0.013363  +0.032379  +0.041341 ]
  [ +0.000000  +0.000000  +0.000000  +1.000000 ]
```

### Saving Results

Calibration saves files automatically into `~/ros_ws/calib_results/`:
1. **`.txt` Report File:** Summary table, transformation matrix, and static TF command.
2. **`.npz` Raw Data File:** Numpy binary containing matrix data and raw pose samples.
