ros2 launch ur_moveit_config ur_moveit.launch.py ur_type:=ur3e launch_rviz:=true



# UR3e MoveIt Control (uet_ur3)

Package này cung cấp node điều khiển và cấu hình quỹ đạo chuyển động cho cánh tay robot UR3e sử dụng thư viện MoveIt 2 trên nền tảng ROS 2 Humble. Hệ thống hỗ trợ linh hoạt cả hai chế độ: Mô phỏng vật lý (Simulation) trong Gazebo và Điều khiển Robot vật lý (Real Robot).

---

## Cài đặt và Biên dịch

Yêu cầu thực hiện biên dịch package trong không gian làm việc (workspace) trước khi vận hành:

```bash
cd ~/ros_ws
colcon build --packages-select ur3_moveit_control
source install/setup.bash
```

---

## 1. Vận hành trong môi trường Mô phỏng (Simulation)

Chế độ này sử dụng môi trường mô phỏng Gazebo (Ignition) phối hợp với cơ chế đồng bộ thời gian mô phỏng (`use_sim_time:=true`).

### Terminal 1: Khởi chạy môi trường Gazebo
Khởi tạo mô phỏng vật lý của robot UR3e trên Gazebo:
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur_simulation_gz ur_sim_control.launch.py ur_type:=ur3e
```

### Terminal 2: Khởi chạy MoveIt Server và Node điều khiển
Tiến trình này sẽ tích hợp khởi động MoveIt Planning Server, giao diện trực quan hóa RViz 2, và thực thi tuần tự quỹ đạo di chuyển đã lập trình sẵn sau thời gian trễ 5 giây:
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur3_moveit_control ur3_demo.launch.py \
  ur_type:=ur3e \
  use_sim_time:=true
```

---

## 2. Vận hành với Robot vật lý (Real Robot)

Chế độ này thực thi các lệnh điều khiển trực tiếp tới phần cứng robot UR3e thông qua kết nối Ethernet (`use_sim_time:=false`).

### Terminal 1: Khởi chạy trình điều khiển phần cứng (Driver)
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur_robot_driver ur_control.launch.py \
  ur_type:=ur3e \
  robot_ip:=192.168.1.10 \
  launch_rviz:=false
```

> [!IMPORTANT]
> **Quy trình bắt buộc trên thiết bị Teach Pendant của Robot:**
> 1. Thiết lập trạng thái robot sang chế độ Remote Control (Điều khiển từ xa).
> 2. Mở chương trình điều khiển có chứa node External Control (đảm bảo cấu hình đúng địa chỉ IP của máy tính gửi lệnh).
> 3. Nhấn nút Play trên Teach Pendant để bắt đầu thực thi chương trình kết nối.
> 4. Xác nhận kết nối thành công tại Terminal Driver thông qua log:
>    `[UR_Client_Library:]: Robot connected to reverse interface. Ready to receive control commands.`

### Terminal 2: Khởi chạy MoveIt Server và Node điều khiển
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur3_moveit_control ur3_demo.launch.py \
  ur_type:=ur3e \
  use_sim_time:=false
```

---

## Các tham số cấu hình trong `ur3_demo.launch.py`

Người dùng có thể tùy biến hành vi hệ thống thông qua việc truyền các đối số (arguments) khi thực thi file launch:

| Tham số | Giá trị mặc định | Mô tả chi tiết |
| :--- | :--- | :--- |
| `ur_type` | `ur3e` | Dòng robot Universal Robots tương ứng (e.g. `ur3`, `ur3e`, `ur5`, `ur5e`). |
| `use_sim_time` | `true` | Xác định nguồn thời gian sử dụng (đặt `false` đối với robot vật lý). |
| `launch_rviz` | `true` | Tùy chọn hiển thị công cụ trực quan hóa RViz 2. |
| `launch_moveit` | `true` | Tùy chọn tự động gọi MoveIt Planning Server (`ur_moveit.launch.py`). |

*Ví dụ cấu hình vận hành thực tế không sử dụng giao diện đồ họa RViz 2:*
```bash
ros2 launch ur3_moveit_control ur3_demo.launch.py ur_type:=ur3e use_sim_time:=false launch_rviz:=false
```

---

## Xử lý lỗi hệ thống (Troubleshooting)

### 1. Lỗi từ chối nhận lệnh điều khiển: `Can't accept new action goals. Controller is not running.`
* **Nguyên nhân:** Bộ điều khiển `scaled_joint_trajectory_controller` trên driver phần cứng chưa được kích hoạt. Lỗi này thường do kết nối giữa máy tính điều khiển và UR Controller Box bị gián đoạn (chương trình External Control trên Teach Pendant chưa được chạy).
* **Khắc phục:** Thực hiện kiểm tra danh sách bộ điều khiển đang hoạt động bằng lệnh `ros2 control list_controllers`. Đảm bảo chương trình External Control đã chạy và hiển thị trạng thái `active`.

### 2. Tiến trình bị treo khi nhận tín hiệu kết thúc (Ctrl+C)
* **Khắc phục:** Node điều khiển `ur3_demo_node` đã được tái cấu trúc luồng xử lý. Tác vụ di chuyển và lập kế hoạch quỹ đạo được đẩy xuống chạy bất đồng bộ ở luồng phụ (background thread), trong khi luồng chính đảm nhiệm việc lắng nghe sự kiện spin của ROS 2. Khi nhận tín hiệu SIGINT (`Ctrl+C`), toàn bộ hệ thống sẽ thoát lập tức mà không gặp hiện tượng treo tiến trình chờ giải phóng tài nguyên.

---

## 3. Hiệu chuẩn Camera Eye-in-Hand (Hand-Eye Calibration)

Tính năng này tự động tính toán **ma trận chuyển vị 4×4** (Homogeneous Transformation Matrix) từ end-effector (`tool0`) tới camera (`camera_link`) bằng thuật toán Hand-Eye Calibration của OpenCV. Script hỗ trợ chạy đồng thời **5 thuật toán** (Tsai, Park, Horaud, Andreff, Daniilidis) và tự động chọn kết quả tốt nhất.

### Yêu cầu trước khi chạy

- Camera RealSense đã cắm và hoạt động
- ArUco Marker đã in và đặt cố định trên mặt bàn (ID mặc định: `26`, kích thước: `0.1m`)
- Robot UR3e đã kết nối và TF `base_link` → `tool0` đang publish
- Package `aruco_ros` đã cài đặt

### Terminal 1: Khởi chạy Camera RealSense
```bash
source ~/ros_ws/install/setup.bash
ros2 launch realsense2_camera rs_launch.py align_depth.enable:=true
```
*Đợi đến khi log báo `RealSense Node Is Up!`*

> [!CAUTION]
> **KHÔNG bật PointCloud2 color** khi calibrate. PointCloud2 tiêu tốn rất nhiều tài nguyên (~9 triệu điểm 3D/giây) và có thể gây **freeze toàn bộ hệ thống**. Chỉ cần image RGB là đủ cho ArUco detection.

### Terminal 2: Khởi chạy Robot Driver + MoveIt
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur_robot_driver ur_control.launch.py \
  ur_type:=ur3e \
  robot_ip:=192.168.1.10 \
  launch_rviz:=false
```
*(Nhớ bật External Control trên Teach Pendant)*

### Terminal 3: Khởi chạy ArUco Marker Detection
```bash
source ~/ros_ws/install/setup.bash
ros2 launch ur3_moveit_control eye_in_hand_calib.launch.py
```

### Terminal 4: Chạy Script Calibration
```bash
source ~/ros_ws/install/setup.bash
python3 ~/ros_ws/src/uet_ur3/ur3_moveit_control/scripts/realsense_calib_eye_in_hand.py
```

### Quy trình thao tác

| Phím | Chức năng |
| :--- | :--- |
| `Enter` | Lấy 1 mẫu calibration (TF robot + pose marker) |
| `c` + `Enter` | Tính toán calibration và in kết quả |
| `q` + `Enter` | Thoát chương trình |

> [!IMPORTANT]
> **Quy trình lấy mẫu chuẩn xác:**
> 1. Cần **ít nhất 5 mẫu** ở các tư thế (pose) tay máy **khác nhau rõ rệt** — thay đổi cả vị trí lẫn góc xoay.
> 2. Tại mỗi tư thế, đảm bảo camera **nhìn thấy rõ toàn bộ ArUco Marker** trước khi nhấn Enter.
> 3. Tránh các tư thế quá gần nhau hoặc chỉ thay đổi 1 trục — điều này gây ra kết quả không chính xác.
> 4. Sau khi lấy đủ mẫu, gõ `c` để tính toán. Script sẽ in ra:
>    - **Bảng so sánh** kết quả 5 thuật toán (X, Y, Z, Roll, Pitch, Yaw)
>    - **Ma trận chuyển vị 4×4** của thuật toán tốt nhất
>    - **Lệnh `static_transform_publisher`** sẵn sàng copy-paste (cả Quaternion và Euler)

### Output mẫu

```
═══════════════════════════════════════════════════
  🏆 KẾT QUẢ TỐT NHẤT: PARK
═══════════════════════════════════════════════════
--- MA TRẬN CHUYỂN VỊ 4×4 (tool0 → camera_link) ---
  [ +0.999123  -0.012345  +0.034567  +0.045678 ]
  [ +0.011234  +0.998765  +0.023456  -0.023456 ]
  [ -0.035678  -0.022345  +0.999234  +0.067890 ]
  [ +0.000000  +0.000000  +0.000000  +1.000000 ]
```

### Lưu và tái sử dụng kết quả

Hệ thống sẽ tự động lưu lại **2 file** cho mỗi lần calib tại thư mục `~/ros_ws/calib_results/`:
1. **File `.txt` (Báo cáo tổng hợp):** Chứa kết quả ma trận, nhận xét, và lệnh publish TF có thể đọc và copy trực tiếp (phù hợp để làm báo cáo).
2. **File `.npz` (Dữ liệu gốc):** Chứa toàn bộ ma trận và dữ liệu mẫu. Để load lại bằng code:
```python
import numpy as np
data = np.load('~/ros_ws/calib_results/hand_eye_calib_20260625_170000.npz', allow_pickle=True)
T_best = data['HORAUD_T']  # Ma trận 4x4 của thuật toán HORAUD
print(T_best)
```

### Áp dụng kết quả vào hệ thống

Sau khi calibration xong, copy lệnh `static_transform_publisher` từ output và thêm vào launch file hoặc chạy trực tiếp:
```bash
# Ví dụ (thay số thực tế từ output):
ros2 run tf2_ros static_transform_publisher \
  --x 0.045678 --y -0.023456 --z 0.067890 \
  --qx 0.012345 --qy -0.017890 --qz 0.005678 --qw 0.999750 \
  --frame-id tool0 --child-frame-id camera_link
```
