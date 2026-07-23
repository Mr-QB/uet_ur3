Luôn tóm tắt những gì đã sửa kèm đường dẫn file sửa
Luôn giải thích những thứ mới mà codex cho rằng người dùng chưa biết
Luôn giải thích luồng khi của dự án khi cập nhật thêm code
Luôn tự động build lại bằng lệnh sau: 
"CMAKE_BUILD_PARALLEL_LEVEL=1 \
MAKEFLAGS="-j1" \
colcon build \
  --executor sequential \
  --event-handlers console_direct+"

