// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ur_dashboard_msgs:msg/UserRole.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__USER_ROLE__STRUCT_H_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__USER_ROLE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'PROGRAMMER'.
static const char * const ur_dashboard_msgs__msg__UserRole__PROGRAMMER = "PROGRAMMER";

/// Constant 'OPERATOR'.
static const char * const ur_dashboard_msgs__msg__UserRole__OPERATOR = "OPERATOR";

/// Constant 'NONE'.
static const char * const ur_dashboard_msgs__msg__UserRole__NONE = "NONE";

/// Constant 'LOCKED'.
static const char * const ur_dashboard_msgs__msg__UserRole__LOCKED = "LOCKED";

/// Constant 'RESTRICTED'.
static const char * const ur_dashboard_msgs__msg__UserRole__RESTRICTED = "RESTRICTED";

// Include directives for member types
// Member 'role'
#include "rosidl_runtime_c/string.h"

/// Struct defined in msg/UserRole in the package ur_dashboard_msgs.
typedef struct ur_dashboard_msgs__msg__UserRole
{
  rosidl_runtime_c__String role;
} ur_dashboard_msgs__msg__UserRole;

// Struct for a sequence of ur_dashboard_msgs__msg__UserRole.
typedef struct ur_dashboard_msgs__msg__UserRole__Sequence
{
  ur_dashboard_msgs__msg__UserRole * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur_dashboard_msgs__msg__UserRole__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__USER_ROLE__STRUCT_H_
