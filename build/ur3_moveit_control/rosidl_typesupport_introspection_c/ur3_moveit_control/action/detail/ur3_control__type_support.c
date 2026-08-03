// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from ur3_moveit_control:action/UR3Control.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"
#include "ur3_moveit_control/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "ur3_moveit_control/action/detail/ur3_control__functions.h"
#include "ur3_moveit_control/action/detail/ur3_control__struct.h"


// Include directives for member types
// Member `joint_goal`
#include "sensor_msgs/msg/joint_state.h"
// Member `joint_goal`
#include "sensor_msgs/msg/detail/joint_state__rosidl_typesupport_introspection_c.h"
// Member `pose_goal`
#include "geometry_msgs/msg/pose_stamped.h"
// Member `pose_goal`
#include "geometry_msgs/msg/detail/pose_stamped__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ur3_moveit_control__action__UR3Control_Goal__init(message_memory);
}

void ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_fini_function(void * message_memory)
{
  ur3_moveit_control__action__UR3Control_Goal__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_message_member_array[11] = {
  {
    "command_type",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Goal, command_type),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "joint_goal",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Goal, joint_goal),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "pose_goal",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Goal, pose_goal),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "cartesian_x_offset",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Goal, cartesian_x_offset),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "cartesian_y_offset",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Goal, cartesian_y_offset),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "cartesian_z_offset",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Goal, cartesian_z_offset),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "target_x",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Goal, target_x),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "target_y",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Goal, target_y),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "object_x",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Goal, object_x),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "object_y",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Goal, object_y),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "object_z",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Goal, object_z),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_message_members = {
  "ur3_moveit_control__action",  // message namespace
  "UR3Control_Goal",  // message name
  11,  // number of fields
  sizeof(ur3_moveit_control__action__UR3Control_Goal),
  ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_message_member_array,  // message members
  ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_init_function,  // function to initialize message memory (memory has to be allocated)
  ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_message_type_support_handle = {
  0,
  &ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur3_moveit_control
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_Goal)() {
  ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, sensor_msgs, msg, JointState)();
  ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_message_member_array[2].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, geometry_msgs, msg, PoseStamped)();
  if (!ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_message_type_support_handle.typesupport_identifier) {
    ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ur3_moveit_control__action__UR3Control_Goal__rosidl_typesupport_introspection_c__UR3Control_Goal_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"
// already included above
// #include "ur3_moveit_control/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__functions.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__struct.h"


// Include directives for member types
// Member `message`
#include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ur3_moveit_control__action__UR3Control_Result__rosidl_typesupport_introspection_c__UR3Control_Result_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ur3_moveit_control__action__UR3Control_Result__init(message_memory);
}

void ur3_moveit_control__action__UR3Control_Result__rosidl_typesupport_introspection_c__UR3Control_Result_fini_function(void * message_memory)
{
  ur3_moveit_control__action__UR3Control_Result__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ur3_moveit_control__action__UR3Control_Result__rosidl_typesupport_introspection_c__UR3Control_Result_message_member_array[2] = {
  {
    "success",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Result, success),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "message",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Result, message),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ur3_moveit_control__action__UR3Control_Result__rosidl_typesupport_introspection_c__UR3Control_Result_message_members = {
  "ur3_moveit_control__action",  // message namespace
  "UR3Control_Result",  // message name
  2,  // number of fields
  sizeof(ur3_moveit_control__action__UR3Control_Result),
  ur3_moveit_control__action__UR3Control_Result__rosidl_typesupport_introspection_c__UR3Control_Result_message_member_array,  // message members
  ur3_moveit_control__action__UR3Control_Result__rosidl_typesupport_introspection_c__UR3Control_Result_init_function,  // function to initialize message memory (memory has to be allocated)
  ur3_moveit_control__action__UR3Control_Result__rosidl_typesupport_introspection_c__UR3Control_Result_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ur3_moveit_control__action__UR3Control_Result__rosidl_typesupport_introspection_c__UR3Control_Result_message_type_support_handle = {
  0,
  &ur3_moveit_control__action__UR3Control_Result__rosidl_typesupport_introspection_c__UR3Control_Result_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur3_moveit_control
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_Result)() {
  if (!ur3_moveit_control__action__UR3Control_Result__rosidl_typesupport_introspection_c__UR3Control_Result_message_type_support_handle.typesupport_identifier) {
    ur3_moveit_control__action__UR3Control_Result__rosidl_typesupport_introspection_c__UR3Control_Result_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ur3_moveit_control__action__UR3Control_Result__rosidl_typesupport_introspection_c__UR3Control_Result_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"
// already included above
// #include "ur3_moveit_control/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__functions.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__struct.h"


// Include directives for member types
// Member `state`
// already included above
// #include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ur3_moveit_control__action__UR3Control_Feedback__rosidl_typesupport_introspection_c__UR3Control_Feedback_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ur3_moveit_control__action__UR3Control_Feedback__init(message_memory);
}

void ur3_moveit_control__action__UR3Control_Feedback__rosidl_typesupport_introspection_c__UR3Control_Feedback_fini_function(void * message_memory)
{
  ur3_moveit_control__action__UR3Control_Feedback__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ur3_moveit_control__action__UR3Control_Feedback__rosidl_typesupport_introspection_c__UR3Control_Feedback_message_member_array[1] = {
  {
    "state",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_Feedback, state),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ur3_moveit_control__action__UR3Control_Feedback__rosidl_typesupport_introspection_c__UR3Control_Feedback_message_members = {
  "ur3_moveit_control__action",  // message namespace
  "UR3Control_Feedback",  // message name
  1,  // number of fields
  sizeof(ur3_moveit_control__action__UR3Control_Feedback),
  ur3_moveit_control__action__UR3Control_Feedback__rosidl_typesupport_introspection_c__UR3Control_Feedback_message_member_array,  // message members
  ur3_moveit_control__action__UR3Control_Feedback__rosidl_typesupport_introspection_c__UR3Control_Feedback_init_function,  // function to initialize message memory (memory has to be allocated)
  ur3_moveit_control__action__UR3Control_Feedback__rosidl_typesupport_introspection_c__UR3Control_Feedback_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ur3_moveit_control__action__UR3Control_Feedback__rosidl_typesupport_introspection_c__UR3Control_Feedback_message_type_support_handle = {
  0,
  &ur3_moveit_control__action__UR3Control_Feedback__rosidl_typesupport_introspection_c__UR3Control_Feedback_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur3_moveit_control
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_Feedback)() {
  if (!ur3_moveit_control__action__UR3Control_Feedback__rosidl_typesupport_introspection_c__UR3Control_Feedback_message_type_support_handle.typesupport_identifier) {
    ur3_moveit_control__action__UR3Control_Feedback__rosidl_typesupport_introspection_c__UR3Control_Feedback_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ur3_moveit_control__action__UR3Control_Feedback__rosidl_typesupport_introspection_c__UR3Control_Feedback_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"
// already included above
// #include "ur3_moveit_control/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__functions.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__struct.h"


// Include directives for member types
// Member `goal_id`
#include "unique_identifier_msgs/msg/uuid.h"
// Member `goal_id`
#include "unique_identifier_msgs/msg/detail/uuid__rosidl_typesupport_introspection_c.h"
// Member `goal`
#include "ur3_moveit_control/action/ur3_control.h"
// Member `goal`
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ur3_moveit_control__action__UR3Control_SendGoal_Request__init(message_memory);
}

void ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_fini_function(void * message_memory)
{
  ur3_moveit_control__action__UR3Control_SendGoal_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_message_member_array[2] = {
  {
    "goal_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_SendGoal_Request, goal_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "goal",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_SendGoal_Request, goal),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_message_members = {
  "ur3_moveit_control__action",  // message namespace
  "UR3Control_SendGoal_Request",  // message name
  2,  // number of fields
  sizeof(ur3_moveit_control__action__UR3Control_SendGoal_Request),
  ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_message_member_array,  // message members
  ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_message_type_support_handle = {
  0,
  &ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur3_moveit_control
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_SendGoal_Request)() {
  ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, unique_identifier_msgs, msg, UUID)();
  ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_Goal)();
  if (!ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_message_type_support_handle.typesupport_identifier) {
    ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ur3_moveit_control__action__UR3Control_SendGoal_Request__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"
// already included above
// #include "ur3_moveit_control/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__functions.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__struct.h"


// Include directives for member types
// Member `stamp`
#include "builtin_interfaces/msg/time.h"
// Member `stamp`
#include "builtin_interfaces/msg/detail/time__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ur3_moveit_control__action__UR3Control_SendGoal_Response__init(message_memory);
}

void ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_fini_function(void * message_memory)
{
  ur3_moveit_control__action__UR3Control_SendGoal_Response__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_message_member_array[2] = {
  {
    "accepted",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_SendGoal_Response, accepted),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "stamp",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_SendGoal_Response, stamp),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_message_members = {
  "ur3_moveit_control__action",  // message namespace
  "UR3Control_SendGoal_Response",  // message name
  2,  // number of fields
  sizeof(ur3_moveit_control__action__UR3Control_SendGoal_Response),
  ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_message_member_array,  // message members
  ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_message_type_support_handle = {
  0,
  &ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur3_moveit_control
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_SendGoal_Response)() {
  ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, builtin_interfaces, msg, Time)();
  if (!ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_message_type_support_handle.typesupport_identifier) {
    ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ur3_moveit_control__action__UR3Control_SendGoal_Response__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "ur3_moveit_control/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_SendGoal_service_members = {
  "ur3_moveit_control__action",  // service namespace
  "UR3Control_SendGoal",  // service name
  // these two fields are initialized below on the first access
  NULL,  // request message
  // ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Request_message_type_support_handle,
  NULL  // response message
  // ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_SendGoal_Response_message_type_support_handle
};

static rosidl_service_type_support_t ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_SendGoal_service_type_support_handle = {
  0,
  &ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_SendGoal_service_members,
  get_service_typesupport_handle_function,
};

// Forward declaration of request/response type support functions
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_SendGoal_Request)();

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_SendGoal_Response)();

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur3_moveit_control
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_SendGoal)() {
  if (!ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_SendGoal_service_type_support_handle.typesupport_identifier) {
    ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_SendGoal_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_SendGoal_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_SendGoal_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_SendGoal_Response)()->data;
  }

  return &ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_SendGoal_service_type_support_handle;
}

// already included above
// #include <stddef.h>
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"
// already included above
// #include "ur3_moveit_control/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__functions.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__struct.h"


// Include directives for member types
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/uuid.h"
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ur3_moveit_control__action__UR3Control_GetResult_Request__init(message_memory);
}

void ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_fini_function(void * message_memory)
{
  ur3_moveit_control__action__UR3Control_GetResult_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_message_member_array[1] = {
  {
    "goal_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_GetResult_Request, goal_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_message_members = {
  "ur3_moveit_control__action",  // message namespace
  "UR3Control_GetResult_Request",  // message name
  1,  // number of fields
  sizeof(ur3_moveit_control__action__UR3Control_GetResult_Request),
  ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_message_member_array,  // message members
  ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_message_type_support_handle = {
  0,
  &ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur3_moveit_control
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_GetResult_Request)() {
  ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, unique_identifier_msgs, msg, UUID)();
  if (!ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_message_type_support_handle.typesupport_identifier) {
    ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ur3_moveit_control__action__UR3Control_GetResult_Request__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"
// already included above
// #include "ur3_moveit_control/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__functions.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__struct.h"


// Include directives for member types
// Member `result`
// already included above
// #include "ur3_moveit_control/action/ur3_control.h"
// Member `result`
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ur3_moveit_control__action__UR3Control_GetResult_Response__init(message_memory);
}

void ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_fini_function(void * message_memory)
{
  ur3_moveit_control__action__UR3Control_GetResult_Response__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_message_member_array[2] = {
  {
    "status",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_INT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_GetResult_Response, status),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "result",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_GetResult_Response, result),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_message_members = {
  "ur3_moveit_control__action",  // message namespace
  "UR3Control_GetResult_Response",  // message name
  2,  // number of fields
  sizeof(ur3_moveit_control__action__UR3Control_GetResult_Response),
  ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_message_member_array,  // message members
  ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_message_type_support_handle = {
  0,
  &ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur3_moveit_control
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_GetResult_Response)() {
  ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_Result)();
  if (!ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_message_type_support_handle.typesupport_identifier) {
    ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ur3_moveit_control__action__UR3Control_GetResult_Response__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "ur3_moveit_control/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_GetResult_service_members = {
  "ur3_moveit_control__action",  // service namespace
  "UR3Control_GetResult",  // service name
  // these two fields are initialized below on the first access
  NULL,  // request message
  // ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_GetResult_Request_message_type_support_handle,
  NULL  // response message
  // ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_GetResult_Response_message_type_support_handle
};

static rosidl_service_type_support_t ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_GetResult_service_type_support_handle = {
  0,
  &ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_GetResult_service_members,
  get_service_typesupport_handle_function,
};

// Forward declaration of request/response type support functions
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_GetResult_Request)();

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_GetResult_Response)();

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur3_moveit_control
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_GetResult)() {
  if (!ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_GetResult_service_type_support_handle.typesupport_identifier) {
    ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_GetResult_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_GetResult_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_GetResult_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_GetResult_Response)()->data;
  }

  return &ur3_moveit_control__action__detail__ur3_control__rosidl_typesupport_introspection_c__UR3Control_GetResult_service_type_support_handle;
}

// already included above
// #include <stddef.h>
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"
// already included above
// #include "ur3_moveit_control/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__functions.h"
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__struct.h"


// Include directives for member types
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/uuid.h"
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__rosidl_typesupport_introspection_c.h"
// Member `feedback`
// already included above
// #include "ur3_moveit_control/action/ur3_control.h"
// Member `feedback`
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ur3_moveit_control__action__UR3Control_FeedbackMessage__init(message_memory);
}

void ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_fini_function(void * message_memory)
{
  ur3_moveit_control__action__UR3Control_FeedbackMessage__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_message_member_array[2] = {
  {
    "goal_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_FeedbackMessage, goal_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "feedback",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur3_moveit_control__action__UR3Control_FeedbackMessage, feedback),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_message_members = {
  "ur3_moveit_control__action",  // message namespace
  "UR3Control_FeedbackMessage",  // message name
  2,  // number of fields
  sizeof(ur3_moveit_control__action__UR3Control_FeedbackMessage),
  ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_message_member_array,  // message members
  ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_init_function,  // function to initialize message memory (memory has to be allocated)
  ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_message_type_support_handle = {
  0,
  &ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur3_moveit_control
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_FeedbackMessage)() {
  ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, unique_identifier_msgs, msg, UUID)();
  ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur3_moveit_control, action, UR3Control_Feedback)();
  if (!ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_message_type_support_handle.typesupport_identifier) {
    ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ur3_moveit_control__action__UR3Control_FeedbackMessage__rosidl_typesupport_introspection_c__UR3Control_FeedbackMessage_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
