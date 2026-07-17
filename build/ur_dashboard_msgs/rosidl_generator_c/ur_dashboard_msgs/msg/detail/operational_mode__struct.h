// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ur_dashboard_msgs:msg/OperationalMode.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__OPERATIONAL_MODE__STRUCT_H_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__OPERATIONAL_MODE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'MANUAL'.
static const char * const ur_dashboard_msgs__msg__OperationalMode__MANUAL = "MANUAL";

/// Constant 'AUTOMATIC'.
static const char * const ur_dashboard_msgs__msg__OperationalMode__AUTOMATIC = "AUTOMATIC";

// Include directives for member types
// Member 'mode'
#include "rosidl_runtime_c/string.h"

/// Struct defined in msg/OperationalMode in the package ur_dashboard_msgs.
typedef struct ur_dashboard_msgs__msg__OperationalMode
{
  rosidl_runtime_c__String mode;
} ur_dashboard_msgs__msg__OperationalMode;

// Struct for a sequence of ur_dashboard_msgs__msg__OperationalMode.
typedef struct ur_dashboard_msgs__msg__OperationalMode__Sequence
{
  ur_dashboard_msgs__msg__OperationalMode * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur_dashboard_msgs__msg__OperationalMode__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__OPERATIONAL_MODE__STRUCT_H_
