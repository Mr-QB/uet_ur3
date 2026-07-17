// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ur_dashboard_msgs:srv/UploadProgram.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__UPLOAD_PROGRAM__STRUCT_H_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__UPLOAD_PROGRAM__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'file_path'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/UploadProgram in the package ur_dashboard_msgs.
typedef struct ur_dashboard_msgs__srv__UploadProgram_Request
{
  rosidl_runtime_c__String file_path;
} ur_dashboard_msgs__srv__UploadProgram_Request;

// Struct for a sequence of ur_dashboard_msgs__srv__UploadProgram_Request.
typedef struct ur_dashboard_msgs__srv__UploadProgram_Request__Sequence
{
  ur_dashboard_msgs__srv__UploadProgram_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur_dashboard_msgs__srv__UploadProgram_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'answer'
// Member 'program_name'
// already included above
// #include "rosidl_runtime_c/string.h"

/// Struct defined in srv/UploadProgram in the package ur_dashboard_msgs.
typedef struct ur_dashboard_msgs__srv__UploadProgram_Response
{
  rosidl_runtime_c__String answer;
  rosidl_runtime_c__String program_name;
  bool success;
} ur_dashboard_msgs__srv__UploadProgram_Response;

// Struct for a sequence of ur_dashboard_msgs__srv__UploadProgram_Response.
typedef struct ur_dashboard_msgs__srv__UploadProgram_Response__Sequence
{
  ur_dashboard_msgs__srv__UploadProgram_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur_dashboard_msgs__srv__UploadProgram_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__UPLOAD_PROGRAM__STRUCT_H_
