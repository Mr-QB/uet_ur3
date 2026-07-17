// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ur_dashboard_msgs:srv/SetUserRole.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__SET_USER_ROLE__STRUCT_H_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__SET_USER_ROLE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'user_role'
#include "ur_dashboard_msgs/msg/detail/user_role__struct.h"

/// Struct defined in srv/SetUserRole in the package ur_dashboard_msgs.
typedef struct ur_dashboard_msgs__srv__SetUserRole_Request
{
  ur_dashboard_msgs__msg__UserRole user_role;
} ur_dashboard_msgs__srv__SetUserRole_Request;

// Struct for a sequence of ur_dashboard_msgs__srv__SetUserRole_Request.
typedef struct ur_dashboard_msgs__srv__SetUserRole_Request__Sequence
{
  ur_dashboard_msgs__srv__SetUserRole_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur_dashboard_msgs__srv__SetUserRole_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'answer'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/SetUserRole in the package ur_dashboard_msgs.
typedef struct ur_dashboard_msgs__srv__SetUserRole_Response
{
  bool success;
  rosidl_runtime_c__String answer;
} ur_dashboard_msgs__srv__SetUserRole_Response;

// Struct for a sequence of ur_dashboard_msgs__srv__SetUserRole_Response.
typedef struct ur_dashboard_msgs__srv__SetUserRole_Response__Sequence
{
  ur_dashboard_msgs__srv__SetUserRole_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur_dashboard_msgs__srv__SetUserRole_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__SET_USER_ROLE__STRUCT_H_
