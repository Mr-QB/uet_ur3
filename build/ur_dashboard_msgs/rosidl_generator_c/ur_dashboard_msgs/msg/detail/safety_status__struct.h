// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ur_dashboard_msgs:msg/SafetyStatus.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__SAFETY_STATUS__STRUCT_H_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__SAFETY_STATUS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'NORMAL'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__NORMAL = 1
};

/// Constant 'REDUCED'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__REDUCED = 2
};

/// Constant 'PROTECTIVE_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__PROTECTIVE_STOP = 3
};

/// Constant 'RECOVERY'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__RECOVERY = 4
};

/// Constant 'SAFEGUARD_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__SAFEGUARD_STOP = 5
};

/// Constant 'SYSTEM_EMERGENCY_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__SYSTEM_EMERGENCY_STOP = 6
};

/// Constant 'ROBOT_EMERGENCY_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__ROBOT_EMERGENCY_STOP = 7
};

/// Constant 'VIOLATION'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__VIOLATION = 8
};

/// Constant 'FAULT'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__FAULT = 9
};

/// Constant 'VALIDATE_JOINT_ID'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__VALIDATE_JOINT_ID = 10
};

/// Constant 'UNDEFINED_SAFETY_MODE'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__UNDEFINED_SAFETY_MODE = 11
};

/// Constant 'AUTOMATIC_MODE_SAFEGUARD_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__AUTOMATIC_MODE_SAFEGUARD_STOP = 12
};

/// Constant 'SYSTEM_THREE_POSITION_ENABLING_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__SYSTEM_THREE_POSITION_ENABLING_STOP = 13
};

/// Constant 'TP_THREE_POSITION_ENABLING_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__TP_THREE_POSITION_ENABLING_STOP = 14
};

/// Constant 'IMMI_EMERGENCY_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__IMMI_EMERGENCY_STOP = 15
};

/// Constant 'IMMI_SAFEGUARD_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__IMMI_SAFEGUARD_STOP = 16
};

/// Constant 'PROFISAFE_WAITING_FOR_PARAMETERS'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__PROFISAFE_WAITING_FOR_PARAMETERS = 17
};

/// Constant 'PROFISAFE_AUTOMATIC_MODE_SAFEGUARD_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__PROFISAFE_AUTOMATIC_MODE_SAFEGUARD_STOP = 18
};

/// Constant 'PROFISAFE_SAFEGUARD_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__PROFISAFE_SAFEGUARD_STOP = 19
};

/// Constant 'PROFISAFE_EMERGENCY_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__PROFISAFE_EMERGENCY_STOP = 20
};

/// Constant 'SAFETY_API_SAFEGUARD_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__SAFETY_API_SAFEGUARD_STOP = 22
};

/// Constant 'IO_PLANE_STOP'.
enum
{
  ur_dashboard_msgs__msg__SafetyStatus__IO_PLANE_STOP = 23
};

/// Struct defined in msg/SafetyStatus in the package ur_dashboard_msgs.
typedef struct ur_dashboard_msgs__msg__SafetyStatus
{
  int8_t status;
} ur_dashboard_msgs__msg__SafetyStatus;

// Struct for a sequence of ur_dashboard_msgs__msg__SafetyStatus.
typedef struct ur_dashboard_msgs__msg__SafetyStatus__Sequence
{
  ur_dashboard_msgs__msg__SafetyStatus * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur_dashboard_msgs__msg__SafetyStatus__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__SAFETY_STATUS__STRUCT_H_
