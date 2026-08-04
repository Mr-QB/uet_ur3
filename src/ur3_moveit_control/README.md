ros2 launch ur_moveit_config ur_moveit.launch.py ur_type:=ur3e launch_rviz:=true



# UR3e MoveIt Control (uet_ur3)

Package này cung cấp node điều khiển và cấu hình quỹ đạo chuyển động cho cánh tay robot UR3e sử dụng thư viện MoveIt 2 trên nền tảng ROS 2 Humble. Hệ thống hỗ trợ linh hoạt cả hai chế độ: Mô phỏng vật lý (Simulation) trong Gazebo và Điều khiển Robot vật lý (Real Robot).

---

## Cài đặt và Biên dịch

Yêu cầu thực hiện biên dịch package trong không gian làm việc (workspace) trước khi vận hành:

```bash
cd ~/uet_ws
colcon build --packages-select ur3_moveit_control
source src/ur3_moveit_control/scripts/activate_uet_ros.bash
```

Phải source `activate_uet_ros.bash` trong **mọi terminal** chạy Gazebo, MoveIt,
`send_goal_client.py` hoặc `random_pick_test.py`. Script đặt cùng một môi trường
ROS discovery (`ROS_DOMAIN_ID=10`, `ROS_LOCALHOST_ONLY=0`, Fast DDS local peer
discovery) rồi source workspace. Script chủ động unset `ROS_DISCOVERY_SERVER`
và `CYCLONEDDS_URI` để mô phỏng trên máy này không phụ thuộc một Fast DDS
Discovery Server chạy qua Tailscale. Nếu các terminal dùng domain hoặc discovery
mode khác nhau, process vẫn có thể đang chạy nhưng service/controller/action sẽ
không nhìn thấy nhau.

---

## 1. Vận hành trong môi trường Mô phỏng (Simulation)

Chế độ này sử dụng môi trường mô phỏng Gazebo (Ignition) phối hợp với cơ chế đồng bộ thời gian mô phỏng (`use_sim_time:=true`).

### Terminal 1: Khởi chạy môi trường Gazebo
Khởi tạo mô phỏng vật lý của robot UR3e trên Gazebo:
```bash
source ~/uet_ws/src/ur3_moveit_control/scripts/activate_uet_ros.bash
ros2 launch ur_simulation_gz ur_sim_control.launch.py ur_type:=ur3e
```

### Terminal 2: Khởi chạy MoveIt Server và Node điều khiển
Tiến trình này sẽ tích hợp khởi động MoveIt Planning Server, giao diện trực quan hóa RViz 2, và thực thi tuần tự quỹ đạo di chuyển đã lập trình sẵn sau thời gian trễ 5 giây:
```bash
source ~/uet_ws/src/ur3_moveit_control/scripts/activate_uet_ros.bash
 ros2 launch ur3_moveit_control ur3_demo_gripper.launch.py \
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

## 3. Test planning nhiều lần với một chai ngẫu nhiên

`random_pick_test.py` tái sử dụng entity `pick_box`, hiện có hình dạng chai cao
0.26 m và đường kính thân 0.10 m. Mỗi trial sẽ:

`ur3_control_node` đồng thời publish mô hình trực quan của chai cố định lên
`/fixed_bottle_marker`. Cấu hình RViz nạp sẵn display `Fixed Bottle`, vì vậy chai
ở `base_link (0.62, 0.00, 0.05)` xuất hiện làm mốc đặt gripper nhưng marker này
không tham gia collision checking và không chặn đường Cartesian đi vào chai.

1. Xóa fixed joint và attached collision object còn sót từ trial trước.
2. Đưa `pick_box` ra vị trí đỗ ngoài vùng làm việc.
3. Đưa robot về `home` khi vùng làm việc chưa có chai.
4. Đặt `pick_box` tới một tọa độ ngẫu nhiên bằng Gazebo Transport, mở
   gripper và chạy đủ chuỗi pick-and-place.
5. Sau khi attach và lift, mang chai sang trái hoặc phải bằng strict Cartesian
   với `dy = ±0.06 .. ±0.10 m`, đồng thời cho phép `dx = -0.02 .. 0.02 m`.
6. Giữ chai attached và đóng gripper, sau đó lập kế hoạch chỉ xoay
   `wrist_3_joint` mặc định `+100°` để tạo tư thế rót.
7. Random test không có bước mở càng hay thả chai xuống bàn. Sau tư
   thế rót, chương trình detach nội bộ, chuyển ngay entity tới vị trí
   parking và xóa collision object cũ để trial sau dùng lại chai.

`send_goal_client.py` dừng khi chai vẫn attached ở tư thế rót. Tham số
`--pour-angle-deg` đổi độ nghiêng; dùng giá trị âm, ví dụ `-100`, nếu
cần rót theo hướng ngược lại. Chuyển động này dùng joint-space
planning có collision checking; các cấu hình PRM, IK và Cartesian cũ không
thay đổi.

Chuỗi gắp dùng kiểu **radial side grasp** giống ảnh tham chiếu. Với mỗi tọa độ
chai, hướng tiếp cận được tính bằng `yaw = atan2(object_y, object_x)`, vì vậy TCP
luôn quay mặt từ robot về phía chai thay vì dùng một quaternion cố định cho cả
vùng làm việc. TCP mặc định gắp tại tâm chiều cao chai với
`tcp_grasp_offset = 0.06 m`; `gripper_tcp` cũng được đặt tại giữa chiều dài hai
ngón kẹp. Cả test chai cố định và random test dùng pre-grasp cách
grasp pose `0.12 m` theo hướng radial. Khoảng này vẫn tạo đủ khe hở
ngoài bán kính chai `0.05 m`, nhưng không kéo dài đoạn Cartesian
thẳng để tránh tăng nguy cơ gặp joint limit hoặc singularity.

Bước tiến cuối dùng `MOVE_CARTESIAN_STRICT`: nếu MoveIt không tạo được ít nhất
99% đường thẳng thì trial dừng, không fallback sang PRM và không cho wrist quay
đường vòng ngay trước khi đóng càng. TCP dừng trước tâm chai `0.015 m` theo hướng
tiếp cận (`grasp_depth_offset`) để mặt trước thân gripper không chạm chai. Lệnh
đóng mặc định cho bài test cố định là `0.042 m`, đủ để hai càng tiếp xúc với
thân chai đường kính `0.100 m`.

Thứ tự này ngăn gripper va vào chai mới trong lúc quay từ tư thế cuối của trial
trước về `home`. Vị trí đỗ mặc định trong world là `(1.50, 1.50, 0.10) m`.
Có thể đổi bằng `--parking-world-x`, `--parking-world-y` và
`--parking-world-z` nếu vị trí đó không phù hợp với world đang dùng.

Không cần restart Gazebo giữa các trial. Cần giữ launch mô phỏng và action
server đang chạy, sau đó mở terminal mới:

```bash
cd ~/uet_ws
source src/ur3_moveit_control/scripts/activate_uet_ros.bash
ros2 run ur3_moveit_control random_pick_test.py \
  --trials 20 \
  --seed 23 \
  --results-file data/random_pick_results.csv
```

`seed` giúp sinh lại đúng cùng một tập tọa độ và độ dịch chuyển để so sánh các
planner công bằng. Mặc định vùng spawn chai nằm trong:

```text
x = 0.58 .. 0.64 m
y = -0.06 .. 0.06 m
```

Ví dụ thay đổi vùng test:

```bash
ros2 run ur3_moveit_control random_pick_test.py \
  --trials 50 \
  --seed 23 \
  --pick-x-min 0.58 \
  --pick-x-max 0.64 \
  --pick-y-min -0.06 \
  --pick-y-max 0.06 \
  --transport-x-max 0.02 \
  --transport-y-min 0.06 \
  --transport-y-max 0.10
```

File CSV lưu tọa độ pick, `transport_dx/dy`, bước bị lỗi và thời gian wall-clock của từng
trial. Nếu world Gazebo không tên `empty`, truyền thêm `--world <tên_world>`.
Mỗi trial random thành công cũng lưu pose và sáu joint pre-grasp vào file dùng
chung `data/successful_waypoints.csv`.

Khi chạy riêng `send_goal_client.py`, sáu joint tại pre-grasp được chụp tạm
trong RAM. Chỉ sau khi advance, đóng càng, attach/lift và Cartesian transport
đều thành công, waypoint mới được thêm vào
`data/successful_waypoints.csv`, tính từ nơi chạy lệnh. Trial thất bại
không được ghi vào file này.

Khi planning tới pre-grasp, action server đọc file waypoint thành công dùng chung,
chọn các hàng có pose gần mục tiêu làm seed cho KDL IK, chuẩn hóa các góc tương
đương về nhánh gần trạng thái hiện tại và xếp hạng nhiều nghiệm. Ba wrist joint
được phạt cao hơn shoulder/elbow; tối đa ba plan thành công được so sánh theo
độ dài chuyển động khớp trước khi execute. Cartesian path bật cả phát hiện
relative jump và giới hạn tuyệt đối `0.20 rad` giữa hai mẫu. Trước khi lift,
server còn kiểm tra vị trí TCP, trạng thái đóng/lực của gripper và chờ chuỗi
`/pick_box/attachment_state="attached"` từ Gazebo 6.

`send_goal_client.py` và `random_pick_test.py` dùng chung toàn bộ thông số và
công thức side-grasp trong `src/grasp_profile.py`. Random test chỉ teleport một
entity `pick_box` duy nhất giữa các trial; không được xóa rồi spawn lại chai vì
Gazebo `DetachableJoint` giữ ID của entity được tạo lúc robot khởi động. Nếu
chai bị mất, hãy restart launch Gazebo để chai được tạo trước plugin robot.
Mỗi lần gắp, action server thử tối đa ba request attach trong tổng thời gian
`3 s`; giữa các lần thử nó gửi detach để buộc Gazebo tạo một chuyển trạng thái
`"attached"` mới thay vì chờ mãi một confirmation đã bị mất.

`send_goal_client.py` nhận tọa độ vật động trong frame `base_link`. Ví dụ:

```bash
ros2 run ur3_moveit_control send_goal_client.py \
  --object-x 0.60 --object-y 0.04 --object-z 0.05 \
  --pour-angle-deg 100
```

Có thể đặt cả vị trí dịch sau lift bằng `--transport-dx` và `--transport-dy`.
Nếu camera trả pose trong `camera_link`, phải TF-transform pose đó sang
`base_link`; không truyền trực tiếp tọa độ camera vào ba tham số trên.

Trước khi tới pre-grasp, one-shot client chạy cùng bước khởi tạo ổn
định như random test: xóa attachment cũ bằng `PREPARE_NEXT_TRIAL`, đưa
tay về `home`, rồi mới mở gripper. Như vậy trajectory tới pre-grasp
được lập và thực thi từ một trạng thái khớp xác định, giảm lỗi
controller path tolerance sau những lần chạy trước.

Trong model mô phỏng hiện tại, `object_z` là cao độ mốc model/đáy chai
so với `base_link`; TCP thực tế sẽ nhắm tới
`grasp_z = object_z + tcp_grasp_offset`. Nếu thuật toán camera đã trả về
thẳng **tâm điểm cần kẹp**, hãy truyền thêm
`--tcp-grasp-offset 0.0`; nếu camera trả về mốc đáy chai thì giữ giá trị
mặc định `0.06 m`.

Simulation mặc định dùng `ur_simulation_gz/worlds/fast_empty.sdf`: physics step
`0.002 s`, RTF mục tiêu `2.0`. Đây là giá trị mục tiêu; RTF thực vẫn có thể thấp
hơn hoặc dao động nếu CPU/GPU không xử lý kịp. Có thể quay về world chuẩn bằng
launch argument `world_file:=empty.sdf`. Gazebo chạy ở verbosity `2` thay vì
debug `4`, và random test chờ `0.25 s` wall-clock sau mỗi lần teleport (xấp xỉ
`0.5 s` simulation tại RTF 2).

---

## 4. Hiệu chuẩn Camera Eye-in-Hand (Hand-Eye Calibration)

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
