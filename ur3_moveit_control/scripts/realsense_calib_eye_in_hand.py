#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
import tf2_ros
import cv2
import numpy as np
from scipy.spatial.transform import Rotation as R
import threading
import sys
import os
from datetime import datetime

class HandEyeCalibrator(Node):
    def __init__(self):
        super().__init__('hand_eye_calibrator')
        
        self.tf_buffer = tf2_ros.Buffer()
        self.tf_listener = tf2_ros.TransformListener(self.tf_buffer, self)
        
        self.R_gripper2base_list = []
        self.t_gripper2base_list = []
        self.R_target2cam_list = []
        self.t_target2cam_list = []
        
        self.get_logger().info('=============================================')
        self.get_logger().info('  HAND-EYE CALIBRATION SCRIPT (MULTI-METHOD) ')
        self.get_logger().info('=============================================')
        self.get_logger().info('HƯỚNG DẪN SỬ DỤNG:')
        self.get_logger().info('1. Di chuyển tay máy tới một góc nhìn thấy rõ Marker.')
        self.get_logger().info('2. Gõ [Enter] để lấy mẫu (Cần ít nhất 5 mẫu ở các góc khác nhau).')
        self.get_logger().info('3. Gõ chữ "c" rồi [Enter] để tự động tính toán Offset.')
        self.get_logger().info('4. Gõ chữ "q" rồi [Enter] để thoát.')
        self.get_logger().info('=============================================')
        
        # Chạy luồng đọc phím ngầm
        self.input_thread = threading.Thread(target=self.input_loop)
        self.input_thread.daemon = True
        self.input_thread.start()

    def get_transform_matrix(self, parent_frame, child_frame):
        try:
            # Chờ tối đa 1.0s để lấy TF mới nhất
            trans = self.tf_buffer.lookup_transform(
                parent_frame, 
                child_frame, 
                rclpy.time.Time(), 
                rclpy.duration.Duration(seconds=1.0)
            )
            
            t = np.array([[trans.transform.translation.x],
                          [trans.transform.translation.y],
                          [trans.transform.translation.z]])
                          
            q = [trans.transform.rotation.x,
                 trans.transform.rotation.y,
                 trans.transform.rotation.z,
                 trans.transform.rotation.w]
                 
            rot = R.from_quat(q).as_matrix()
            
            return rot, t
        except Exception as e:
            self.get_logger().error(f"Lỗi: Không tìm thấy TF từ {parent_frame} đến {child_frame}: {e}")
            return None, None

    def take_sample(self):
        self.get_logger().info('Đang lấy mẫu toạ độ...')
        
        R_g2b, t_g2b = self.get_transform_matrix('base_link', 'tool0')
        if R_g2b is None:
            self.get_logger().warn('Lấy mẫu thất bại! Hãy kiểm tra lại tay máy.')
            return
            
        R_t2c, t_t2c = self.get_transform_matrix('camera_link', 'aruco_marker_frame')
        if R_t2c is None:
            self.get_logger().warn('Lấy mẫu thất bại! Hãy chắc chắn camera đang nhìn thấy tấm Marker.')
            return
            
        self.R_gripper2base_list.append(R_g2b)
        self.t_gripper2base_list.append(t_g2b)
        self.R_target2cam_list.append(R_t2c)
        self.t_target2cam_list.append(t_t2c)
        
        n = len(self.R_gripper2base_list)
        self.get_logger().info(f'👉 Đã lưu thành công mẫu thứ {n}!')
        self.get_logger().info(f'   Tool0 pos: [{t_g2b[0][0]:.4f}, {t_g2b[1][0]:.4f}, {t_g2b[2][0]:.4f}]')
        self.get_logger().info(f'   Marker pos: [{t_t2c[0][0]:.4f}, {t_t2c[1][0]:.4f}, {t_t2c[2][0]:.4f}]')

    def build_homogeneous_matrix(self, R_mat, t_vec):
        """Xây dựng ma trận chuyển vị thuần nhất 4×4 từ R (3×3) và t (3×1)."""
        T = np.eye(4)
        T[:3, :3] = R_mat
        T[:3, 3] = t_vec.flatten()
        return T

    def print_matrix_4x4(self, T, label=''):
        """In đẹp ma trận 4×4."""
        self.get_logger().info(f'--- {label} ---')
        for i in range(4):
            row = '  '.join([f'{T[i, j]:+.6f}' for j in range(4)])
            self.get_logger().info(f'  [ {row} ]')

    def compute_calibration(self):
        n = len(self.R_gripper2base_list)
        if n < 3:
            self.get_logger().error(f'Cần ít nhất 3 mẫu để tính toán! Bạn mới có {n} mẫu.')
            return
        
        self.get_logger().info('')
        self.get_logger().info('═' * 60)
        self.get_logger().info('  ĐANG TÍNH TOÁN HAND-EYE CALIBRATION...')
        self.get_logger().info(f'  Số lượng mẫu: {n}')
        self.get_logger().info('═' * 60)
        
        # ── Danh sách 5 thuật toán OpenCV ──
        methods = {
            'TSAI':       cv2.CALIB_HAND_EYE_TSAI,
            'PARK':       cv2.CALIB_HAND_EYE_PARK,
            'HORAUD':     cv2.CALIB_HAND_EYE_HORAUD,
            'ANDREFF':    cv2.CALIB_HAND_EYE_ANDREFF,
            'DANIILIDIS': cv2.CALIB_HAND_EYE_DANIILIDIS,
        }
        
        results = {}
        
        for name, method in methods.items():
            try:
                R_cam2gripper, t_cam2gripper = cv2.calibrateHandEye(
                    self.R_gripper2base_list,
                    self.t_gripper2base_list,
                    self.R_target2cam_list,
                    self.t_target2cam_list,
                    method=method
                )
                
                T_cam2gripper = self.build_homogeneous_matrix(R_cam2gripper, t_cam2gripper)
                rot = R.from_matrix(R_cam2gripper)
                rpy = rot.as_euler('xyz', degrees=False)
                rpy_deg = rot.as_euler('xyz', degrees=True)
                quat = rot.as_quat()  # x, y, z, w
                
                results[name] = {
                    'R': R_cam2gripper,
                    't': t_cam2gripper,
                    'T': T_cam2gripper,
                    'rpy': rpy,
                    'rpy_deg': rpy_deg,
                    'quat': quat,
                }
                
            except Exception as e:
                self.get_logger().error(f'  Thuật toán {name} lỗi: {e}')
        
        if not results:
            self.get_logger().error('Tất cả thuật toán đều lỗi! Kiểm tra lại dữ liệu mẫu.')
            return
        
        # ── In kết quả từng thuật toán ──
        self.get_logger().info('')
        self.get_logger().info('═' * 60)
        self.get_logger().info('  SO SÁNH KẾT QUẢ 5 THUẬT TOÁN')
        self.get_logger().info('═' * 60)
        
        self.get_logger().info('')
        self.get_logger().info(f'  {"Method":<12} {"X(m)":>10} {"Y(m)":>10} {"Z(m)":>10} │ {"Roll°":>8} {"Pitch°":>8} {"Yaw°":>8}')
        self.get_logger().info('  ' + '─' * 72)
        
        for name, res in results.items():
            t = res['t']
            rpy_d = res['rpy_deg']
            self.get_logger().info(
                f'  {name:<12} {t[0][0]:>+10.5f} {t[1][0]:>+10.5f} {t[2][0]:>+10.5f} │ '
                f'{rpy_d[0]:>+8.3f} {rpy_d[1]:>+8.3f} {rpy_d[2]:>+8.3f}'
            )
        
        # ── Tính trung vị (median) để chọn kết quả robust nhất ──
        all_t = np.array([res['t'].flatten() for res in results.values()])
        median_t = np.median(all_t, axis=0)
        
        # Chọn thuật toán có translation gần median nhất
        best_name = None
        best_dist = float('inf')
        for name, res in results.items():
            dist = np.linalg.norm(res['t'].flatten() - median_t)
            if dist < best_dist:
                best_dist = dist
                best_name = name
        
        best = results[best_name]
        
        # ── In kết quả chính (Best Method) ──
        self.get_logger().info('')
        self.get_logger().info('═' * 60)
        self.get_logger().info(f'  🏆 KẾT QUẢ TỐT NHẤT: {best_name}')
        self.get_logger().info(f'     (Gần trung vị nhất, distance = {best_dist:.6f}m)')
        self.get_logger().info('═' * 60)
        
        t = best['t']
        rpy = best['rpy']
        rpy_deg = best['rpy_deg']
        quat = best['quat']
        T = best['T']
        
        # In ma trận chuyển vị 4×4
        self.get_logger().info('')
        self.print_matrix_4x4(T, 'MA TRẬN CHUYỂN VỊ 4×4 (tool0 → camera_link)')
        
        self.get_logger().info('')
        self.get_logger().info(f'  Tịnh tiến (X, Y, Z):')
        self.get_logger().info(f'    X = {t[0][0]:+.6f} m')
        self.get_logger().info(f'    Y = {t[1][0]:+.6f} m')
        self.get_logger().info(f'    Z = {t[2][0]:+.6f} m')
        
        self.get_logger().info(f'  Góc xoay RPY (radian):')
        self.get_logger().info(f'    Roll  = {rpy[0]:+.6f} rad  ({rpy_deg[0]:+.3f}°)')
        self.get_logger().info(f'    Pitch = {rpy[1]:+.6f} rad  ({rpy_deg[1]:+.3f}°)')
        self.get_logger().info(f'    Yaw   = {rpy[2]:+.6f} rad  ({rpy_deg[2]:+.3f}°)')
        
        self.get_logger().info(f'  Quaternion (x, y, z, w):')
        self.get_logger().info(f'    [{quat[0]:+.6f}, {quat[1]:+.6f}, {quat[2]:+.6f}, {quat[3]:+.6f}]')
        
        # ── Lệnh static_transform_publisher ──
        self.get_logger().info('')
        self.get_logger().info('═' * 60)
        self.get_logger().info('  LỆNH PUBLISH TF (copy để dùng)')
        self.get_logger().info('═' * 60)
        
        # Format mới (quaternion - chính xác hơn)
        self.get_logger().info('')
        self.get_logger().info('  📌 Dùng Quaternion (khuyên dùng):')
        self.get_logger().info(
            f'  ros2 run tf2_ros static_transform_publisher '
            f'--x {t[0][0]:.6f} --y {t[1][0]:.6f} --z {t[2][0]:.6f} '
            f'--qx {quat[0]:.6f} --qy {quat[1]:.6f} --qz {quat[2]:.6f} --qw {quat[3]:.6f} '
            f'--frame-id tool0 --child-frame-id camera_link'
        )
        
        # Format cũ (euler)
        self.get_logger().info('')
        self.get_logger().info('  📌 Dùng Euler RPY (tương thích cũ):')
        self.get_logger().info(
            f'  ros2 run tf2_ros static_transform_publisher '
            f'{t[0][0]:.6f} {t[1][0]:.6f} {t[2][0]:.6f} '
            f'{rpy[0]:.6f} {rpy[1]:.6f} {rpy[2]:.6f} '
            f'tool0 camera_link'
        )
        
        # ── Lưu kết quả ra file .npz ──
        self.save_results(results, best_name)
        
        self.get_logger().info('')
        self.get_logger().info('═' * 60)
        self.get_logger().info('  ✅ CALIBRATION HOÀN TẤT!')
        self.get_logger().info('═' * 60)

    def save_results(self, results, best_name):
        """Lưu kết quả calibration ra file .npz và file văn bản .txt dễ đọc"""
        try:
            save_dir = os.path.expanduser('~/ros_ws/calib_results')
            os.makedirs(save_dir, exist_ok=True)
            
            timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
            npz_filepath = os.path.join(save_dir, f'hand_eye_calib_{timestamp}.npz')
            txt_filepath = os.path.join(save_dir, f'hand_eye_calib_{timestamp}.txt')
            
            save_data = {
                'best_method': best_name,
                'n_samples': len(self.R_gripper2base_list),
            }
            
            # Lưu dữ liệu npz
            for name, res in results.items():
                save_data[f'{name}_T'] = res['T']
                save_data[f'{name}_R'] = res['R']
                save_data[f'{name}_t'] = res['t']
                save_data[f'{name}_quat'] = res['quat']
                save_data[f'{name}_rpy'] = res['rpy']
            
            save_data['R_gripper2base'] = np.array(self.R_gripper2base_list)
            save_data['t_gripper2base'] = np.array(self.t_gripper2base_list)
            save_data['R_target2cam'] = np.array(self.R_target2cam_list)
            save_data['t_target2cam'] = np.array(self.t_target2cam_list)
            
            np.savez(npz_filepath, **save_data)
            
            # Lưu dữ liệu txt
            best = results[best_name]
            with open(txt_filepath, 'w', encoding='utf-8') as f:
                f.write("=============================================\n")
                f.write("      KẾT QUẢ HAND-EYE CALIBRATION\n")
                f.write("=============================================\n")
                f.write(f"Thời gian lấy mẫu: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n")
                f.write(f"Số lượng mẫu: {len(self.R_gripper2base_list)}\n")
                f.write(f"Thuật toán tốt nhất (gần trung vị nhất): {best_name}\n\n")
                
                f.write("--- MA TRẬN CHUYỂN VỊ 4x4 (tool0 -> camera_link) ---\n")
                T = best['T']
                for i in range(4):
                    row = '  '.join([f'{T[i, j]:+.6f}' for j in range(4)])
                    f.write(f"  [ {row} ]\n")
                
                f.write("\n--- THÔNG SỐ CHI TIẾT ---\n")
                t = best['t']
                rpy = best['rpy']
                rpy_deg = best['rpy_deg']
                quat = best['quat']
                
                f.write("1. Tịnh tiến (X, Y, Z - mét):\n")
                f.write(f"   X = {t[0][0]:+.6f}\n")
                f.write(f"   Y = {t[1][0]:+.6f}\n")
                f.write(f"   Z = {t[2][0]:+.6f}\n\n")
                
                f.write("2. Góc xoay Euler RPY (radian & độ):\n")
                f.write(f"   Roll  = {rpy[0]:+.6f} rad  ({rpy_deg[0]:+.3f}°)\n")
                f.write(f"   Pitch = {rpy[1]:+.6f} rad  ({rpy_deg[1]:+.3f}°)\n")
                f.write(f"   Yaw   = {rpy[2]:+.6f} rad  ({rpy_deg[2]:+.3f}°)\n\n")
                
                f.write("3. Quaternion (x, y, z, w):\n")
                f.write(f"   [ {quat[0]:+.6f}, {quat[1]:+.6f}, {quat[2]:+.6f}, {quat[3]:+.6f} ]\n\n")
                
                f.write("--- LỆNH ROS2 PUBLISH TF (Sử dụng Quaternion) ---\n")
                f.write(f"ros2 run tf2_ros static_transform_publisher \\\n"
                        f"  --x {t[0][0]:.6f} --y {t[1][0]:.6f} --z {t[2][0]:.6f} \\\n"
                        f"  --qx {quat[0]:.6f} --qy {quat[1]:.6f} --qz {quat[2]:.6f} --qw {quat[3]:.6f} \\\n"
                        f"  --frame-id tool0 --child-frame-id camera_link\n")
            
            self.get_logger().info('')
            self.get_logger().info(f'  💾 Đã lưu dữ liệu gốc vào: {npz_filepath}')
            self.get_logger().info(f'  📄 Đã lưu file đọc dễ nhìn vào: {txt_filepath}')
            
        except Exception as e:
            self.get_logger().error(f'Không thể lưu file: {e}')

    def input_loop(self):
        while True:
            try:
                cmd = sys.stdin.readline().strip()
                if cmd.lower() == 'c':
                    self.compute_calibration()
                elif cmd.lower() == 'q':
                    self.get_logger().info('Đang thoát...')
                    rclpy.shutdown()
                    break
                else:
                    self.take_sample()
            except EOFError:
                break

def main(args=None):
    rclpy.init(args=args)
    node = HandEyeCalibrator()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()
