// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ur_dashboard_msgs:srv/GetPolyScopeVersion.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__GET_POLY_SCOPE_VERSION__STRUCT_H_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__GET_POLY_SCOPE_VERSION__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/GetPolyScopeVersion in the package ur_dashboard_msgs.
typedef struct ur_dashboard_msgs__srv__GetPolyScopeVersion_Request
{
  uint8_t structure_needs_at_least_one_member;
} ur_dashboard_msgs__srv__GetPolyScopeVersion_Request;

// Struct for a sequence of ur_dashboard_msgs__srv__GetPolyScopeVersion_Request.
typedef struct ur_dashboard_msgs__srv__GetPolyScopeVersion_Request__Sequence
{
  ur_dashboard_msgs__srv__GetPolyScopeVersion_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur_dashboard_msgs__srv__GetPolyScopeVersion_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'answer'
#include "rosidl_runtime_c/string.h"
// Member 'version'
#include "ur_dashboard_msgs/msg/detail/version_information__struct.h"

/// Struct defined in srv/GetPolyScopeVersion in the package ur_dashboard_msgs.
typedef struct ur_dashboard_msgs__srv__GetPolyScopeVersion_Response
{
  rosidl_runtime_c__String answer;
  bool success;
  ur_dashboard_msgs__msg__VersionInformation version;
} ur_dashboard_msgs__srv__GetPolyScopeVersion_Response;

// Struct for a sequence of ur_dashboard_msgs__srv__GetPolyScopeVersion_Response.
typedef struct ur_dashboard_msgs__srv__GetPolyScopeVersion_Response__Sequence
{
  ur_dashboard_msgs__srv__GetPolyScopeVersion_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur_dashboard_msgs__srv__GetPolyScopeVersion_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__GET_POLY_SCOPE_VERSION__STRUCT_H_
