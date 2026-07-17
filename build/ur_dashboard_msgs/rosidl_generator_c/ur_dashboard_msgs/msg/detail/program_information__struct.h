// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ur_dashboard_msgs:msg/ProgramInformation.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__STRUCT_H_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'PROGRAM_STATE_DRAFT'.
static const char * const ur_dashboard_msgs__msg__ProgramInformation__PROGRAM_STATE_DRAFT = "DRAFT";

/// Constant 'PROGRAM_STATE_FINAL'.
static const char * const ur_dashboard_msgs__msg__ProgramInformation__PROGRAM_STATE_FINAL = "FINAL";

// Include directives for member types
// Member 'name'
// Member 'description'
// Member 'program_state'
#include "rosidl_runtime_c/string.h"

/// Struct defined in msg/ProgramInformation in the package ur_dashboard_msgs.
typedef struct ur_dashboard_msgs__msg__ProgramInformation
{
  rosidl_runtime_c__String name;
  rosidl_runtime_c__String description;
  uint64_t created_date;
  uint64_t last_saved_date;
  uint64_t last_modified_date;
  rosidl_runtime_c__String program_state;
} ur_dashboard_msgs__msg__ProgramInformation;

// Struct for a sequence of ur_dashboard_msgs__msg__ProgramInformation.
typedef struct ur_dashboard_msgs__msg__ProgramInformation__Sequence
{
  ur_dashboard_msgs__msg__ProgramInformation * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur_dashboard_msgs__msg__ProgramInformation__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__STRUCT_H_
