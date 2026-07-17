// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from ur_dashboard_msgs:msg/ProgramInformation.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "ur_dashboard_msgs/msg/detail/program_information__rosidl_typesupport_introspection_c.h"
#include "ur_dashboard_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "ur_dashboard_msgs/msg/detail/program_information__functions.h"
#include "ur_dashboard_msgs/msg/detail/program_information__struct.h"


// Include directives for member types
// Member `name`
// Member `description`
// Member `program_state`
#include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ur_dashboard_msgs__msg__ProgramInformation__rosidl_typesupport_introspection_c__ProgramInformation_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ur_dashboard_msgs__msg__ProgramInformation__init(message_memory);
}

void ur_dashboard_msgs__msg__ProgramInformation__rosidl_typesupport_introspection_c__ProgramInformation_fini_function(void * message_memory)
{
  ur_dashboard_msgs__msg__ProgramInformation__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ur_dashboard_msgs__msg__ProgramInformation__rosidl_typesupport_introspection_c__ProgramInformation_message_member_array[6] = {
  {
    "name",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur_dashboard_msgs__msg__ProgramInformation, name),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "description",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur_dashboard_msgs__msg__ProgramInformation, description),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "created_date",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT64,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur_dashboard_msgs__msg__ProgramInformation, created_date),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "last_saved_date",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT64,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur_dashboard_msgs__msg__ProgramInformation, last_saved_date),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "last_modified_date",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT64,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur_dashboard_msgs__msg__ProgramInformation, last_modified_date),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "program_state",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur_dashboard_msgs__msg__ProgramInformation, program_state),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ur_dashboard_msgs__msg__ProgramInformation__rosidl_typesupport_introspection_c__ProgramInformation_message_members = {
  "ur_dashboard_msgs__msg",  // message namespace
  "ProgramInformation",  // message name
  6,  // number of fields
  sizeof(ur_dashboard_msgs__msg__ProgramInformation),
  ur_dashboard_msgs__msg__ProgramInformation__rosidl_typesupport_introspection_c__ProgramInformation_message_member_array,  // message members
  ur_dashboard_msgs__msg__ProgramInformation__rosidl_typesupport_introspection_c__ProgramInformation_init_function,  // function to initialize message memory (memory has to be allocated)
  ur_dashboard_msgs__msg__ProgramInformation__rosidl_typesupport_introspection_c__ProgramInformation_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ur_dashboard_msgs__msg__ProgramInformation__rosidl_typesupport_introspection_c__ProgramInformation_message_type_support_handle = {
  0,
  &ur_dashboard_msgs__msg__ProgramInformation__rosidl_typesupport_introspection_c__ProgramInformation_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur_dashboard_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur_dashboard_msgs, msg, ProgramInformation)() {
  if (!ur_dashboard_msgs__msg__ProgramInformation__rosidl_typesupport_introspection_c__ProgramInformation_message_type_support_handle.typesupport_identifier) {
    ur_dashboard_msgs__msg__ProgramInformation__rosidl_typesupport_introspection_c__ProgramInformation_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ur_dashboard_msgs__msg__ProgramInformation__rosidl_typesupport_introspection_c__ProgramInformation_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
