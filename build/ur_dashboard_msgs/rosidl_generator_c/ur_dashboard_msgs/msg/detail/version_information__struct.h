// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ur_dashboard_msgs:msg/VersionInformation.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__VERSION_INFORMATION__STRUCT_H_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__VERSION_INFORMATION__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in msg/VersionInformation in the package ur_dashboard_msgs.
typedef struct ur_dashboard_msgs__msg__VersionInformation
{
  uint32_t major;
  uint32_t minor;
  uint32_t bugfix;
  uint32_t build;
} ur_dashboard_msgs__msg__VersionInformation;

// Struct for a sequence of ur_dashboard_msgs__msg__VersionInformation.
typedef struct ur_dashboard_msgs__msg__VersionInformation__Sequence
{
  ur_dashboard_msgs__msg__VersionInformation * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur_dashboard_msgs__msg__VersionInformation__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__VERSION_INFORMATION__STRUCT_H_
