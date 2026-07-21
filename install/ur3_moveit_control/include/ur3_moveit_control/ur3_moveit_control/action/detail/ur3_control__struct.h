// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ur3_moveit_control:action/UR3Control.idl
// generated code does not contain a copyright notice

#ifndef UR3_MOVEIT_CONTROL__ACTION__DETAIL__UR3_CONTROL__STRUCT_H_
#define UR3_MOVEIT_CONTROL__ACTION__DETAIL__UR3_CONTROL__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'MOVE_HOME'.
enum
{
  ur3_moveit_control__action__UR3Control_Goal__MOVE_HOME = 0
};

/// Constant 'MOVE_JOINT'.
enum
{
  ur3_moveit_control__action__UR3Control_Goal__MOVE_JOINT = 1
};

/// Constant 'MOVE_POSE'.
enum
{
  ur3_moveit_control__action__UR3Control_Goal__MOVE_POSE = 2
};

/// Constant 'ATTACH_AND_LIFT'.
enum
{
  ur3_moveit_control__action__UR3Control_Goal__ATTACH_AND_LIFT = 3
};

// Include directives for member types
// Member 'joint_goal'
#include "sensor_msgs/msg/detail/joint_state__struct.h"
// Member 'pose_goal'
#include "geometry_msgs/msg/detail/pose_stamped__struct.h"

/// Struct defined in action/UR3Control in the package ur3_moveit_control.
typedef struct ur3_moveit_control__action__UR3Control_Goal
{
  uint8_t command_type;
  sensor_msgs__msg__JointState joint_goal;
  geometry_msgs__msg__PoseStamped pose_goal;
  double cartesian_z_offset;
} ur3_moveit_control__action__UR3Control_Goal;

// Struct for a sequence of ur3_moveit_control__action__UR3Control_Goal.
typedef struct ur3_moveit_control__action__UR3Control_Goal__Sequence
{
  ur3_moveit_control__action__UR3Control_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur3_moveit_control__action__UR3Control_Goal__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
#include "rosidl_runtime_c/string.h"

/// Struct defined in action/UR3Control in the package ur3_moveit_control.
typedef struct ur3_moveit_control__action__UR3Control_Result
{
  bool success;
  rosidl_runtime_c__String message;
} ur3_moveit_control__action__UR3Control_Result;

// Struct for a sequence of ur3_moveit_control__action__UR3Control_Result.
typedef struct ur3_moveit_control__action__UR3Control_Result__Sequence
{
  ur3_moveit_control__action__UR3Control_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur3_moveit_control__action__UR3Control_Result__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'state'
// already included above
// #include "rosidl_runtime_c/string.h"

/// Struct defined in action/UR3Control in the package ur3_moveit_control.
typedef struct ur3_moveit_control__action__UR3Control_Feedback
{
  rosidl_runtime_c__String state;
} ur3_moveit_control__action__UR3Control_Feedback;

// Struct for a sequence of ur3_moveit_control__action__UR3Control_Feedback.
typedef struct ur3_moveit_control__action__UR3Control_Feedback__Sequence
{
  ur3_moveit_control__action__UR3Control_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur3_moveit_control__action__UR3Control_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "ur3_moveit_control/action/detail/ur3_control__struct.h"

/// Struct defined in action/UR3Control in the package ur3_moveit_control.
typedef struct ur3_moveit_control__action__UR3Control_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  ur3_moveit_control__action__UR3Control_Goal goal;
} ur3_moveit_control__action__UR3Control_SendGoal_Request;

// Struct for a sequence of ur3_moveit_control__action__UR3Control_SendGoal_Request.
typedef struct ur3_moveit_control__action__UR3Control_SendGoal_Request__Sequence
{
  ur3_moveit_control__action__UR3Control_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur3_moveit_control__action__UR3Control_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

/// Struct defined in action/UR3Control in the package ur3_moveit_control.
typedef struct ur3_moveit_control__action__UR3Control_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} ur3_moveit_control__action__UR3Control_SendGoal_Response;

// Struct for a sequence of ur3_moveit_control__action__UR3Control_SendGoal_Response.
typedef struct ur3_moveit_control__action__UR3Control_SendGoal_Response__Sequence
{
  ur3_moveit_control__action__UR3Control_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur3_moveit_control__action__UR3Control_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

/// Struct defined in action/UR3Control in the package ur3_moveit_control.
typedef struct ur3_moveit_control__action__UR3Control_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} ur3_moveit_control__action__UR3Control_GetResult_Request;

// Struct for a sequence of ur3_moveit_control__action__UR3Control_GetResult_Request.
typedef struct ur3_moveit_control__action__UR3Control_GetResult_Request__Sequence
{
  ur3_moveit_control__action__UR3Control_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur3_moveit_control__action__UR3Control_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__struct.h"

/// Struct defined in action/UR3Control in the package ur3_moveit_control.
typedef struct ur3_moveit_control__action__UR3Control_GetResult_Response
{
  int8_t status;
  ur3_moveit_control__action__UR3Control_Result result;
} ur3_moveit_control__action__UR3Control_GetResult_Response;

// Struct for a sequence of ur3_moveit_control__action__UR3Control_GetResult_Response.
typedef struct ur3_moveit_control__action__UR3Control_GetResult_Response__Sequence
{
  ur3_moveit_control__action__UR3Control_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur3_moveit_control__action__UR3Control_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__struct.h"

/// Struct defined in action/UR3Control in the package ur3_moveit_control.
typedef struct ur3_moveit_control__action__UR3Control_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  ur3_moveit_control__action__UR3Control_Feedback feedback;
} ur3_moveit_control__action__UR3Control_FeedbackMessage;

// Struct for a sequence of ur3_moveit_control__action__UR3Control_FeedbackMessage.
typedef struct ur3_moveit_control__action__UR3Control_FeedbackMessage__Sequence
{
  ur3_moveit_control__action__UR3Control_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ur3_moveit_control__action__UR3Control_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // UR3_MOVEIT_CONTROL__ACTION__DETAIL__UR3_CONTROL__STRUCT_H_
