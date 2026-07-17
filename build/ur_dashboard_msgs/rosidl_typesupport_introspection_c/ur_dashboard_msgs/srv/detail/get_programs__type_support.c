// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from ur_dashboard_msgs:srv/GetPrograms.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "ur_dashboard_msgs/srv/detail/get_programs__rosidl_typesupport_introspection_c.h"
#include "ur_dashboard_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "ur_dashboard_msgs/srv/detail/get_programs__functions.h"
#include "ur_dashboard_msgs/srv/detail/get_programs__struct.h"


#ifdef __cplusplus
extern "C"
{
#endif

void ur_dashboard_msgs__srv__GetPrograms_Request__rosidl_typesupport_introspection_c__GetPrograms_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ur_dashboard_msgs__srv__GetPrograms_Request__init(message_memory);
}

void ur_dashboard_msgs__srv__GetPrograms_Request__rosidl_typesupport_introspection_c__GetPrograms_Request_fini_function(void * message_memory)
{
  ur_dashboard_msgs__srv__GetPrograms_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ur_dashboard_msgs__srv__GetPrograms_Request__rosidl_typesupport_introspection_c__GetPrograms_Request_message_member_array[1] = {
  {
    "structure_needs_at_least_one_member",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur_dashboard_msgs__srv__GetPrograms_Request, structure_needs_at_least_one_member),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ur_dashboard_msgs__srv__GetPrograms_Request__rosidl_typesupport_introspection_c__GetPrograms_Request_message_members = {
  "ur_dashboard_msgs__srv",  // message namespace
  "GetPrograms_Request",  // message name
  1,  // number of fields
  sizeof(ur_dashboard_msgs__srv__GetPrograms_Request),
  ur_dashboard_msgs__srv__GetPrograms_Request__rosidl_typesupport_introspection_c__GetPrograms_Request_message_member_array,  // message members
  ur_dashboard_msgs__srv__GetPrograms_Request__rosidl_typesupport_introspection_c__GetPrograms_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  ur_dashboard_msgs__srv__GetPrograms_Request__rosidl_typesupport_introspection_c__GetPrograms_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ur_dashboard_msgs__srv__GetPrograms_Request__rosidl_typesupport_introspection_c__GetPrograms_Request_message_type_support_handle = {
  0,
  &ur_dashboard_msgs__srv__GetPrograms_Request__rosidl_typesupport_introspection_c__GetPrograms_Request_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur_dashboard_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur_dashboard_msgs, srv, GetPrograms_Request)() {
  if (!ur_dashboard_msgs__srv__GetPrograms_Request__rosidl_typesupport_introspection_c__GetPrograms_Request_message_type_support_handle.typesupport_identifier) {
    ur_dashboard_msgs__srv__GetPrograms_Request__rosidl_typesupport_introspection_c__GetPrograms_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ur_dashboard_msgs__srv__GetPrograms_Request__rosidl_typesupport_introspection_c__GetPrograms_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "ur_dashboard_msgs/srv/detail/get_programs__rosidl_typesupport_introspection_c.h"
// already included above
// #include "ur_dashboard_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "ur_dashboard_msgs/srv/detail/get_programs__functions.h"
// already included above
// #include "ur_dashboard_msgs/srv/detail/get_programs__struct.h"


// Include directives for member types
// Member `programs`
#include "ur_dashboard_msgs/msg/program_information.h"
// Member `programs`
#include "ur_dashboard_msgs/msg/detail/program_information__rosidl_typesupport_introspection_c.h"
// Member `answer`
#include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ur_dashboard_msgs__srv__GetPrograms_Response__init(message_memory);
}

void ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_fini_function(void * message_memory)
{
  ur_dashboard_msgs__srv__GetPrograms_Response__fini(message_memory);
}

size_t ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__size_function__GetPrograms_Response__programs(
  const void * untyped_member)
{
  const ur_dashboard_msgs__msg__ProgramInformation__Sequence * member =
    (const ur_dashboard_msgs__msg__ProgramInformation__Sequence *)(untyped_member);
  return member->size;
}

const void * ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__get_const_function__GetPrograms_Response__programs(
  const void * untyped_member, size_t index)
{
  const ur_dashboard_msgs__msg__ProgramInformation__Sequence * member =
    (const ur_dashboard_msgs__msg__ProgramInformation__Sequence *)(untyped_member);
  return &member->data[index];
}

void * ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__get_function__GetPrograms_Response__programs(
  void * untyped_member, size_t index)
{
  ur_dashboard_msgs__msg__ProgramInformation__Sequence * member =
    (ur_dashboard_msgs__msg__ProgramInformation__Sequence *)(untyped_member);
  return &member->data[index];
}

void ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__fetch_function__GetPrograms_Response__programs(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const ur_dashboard_msgs__msg__ProgramInformation * item =
    ((const ur_dashboard_msgs__msg__ProgramInformation *)
    ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__get_const_function__GetPrograms_Response__programs(untyped_member, index));
  ur_dashboard_msgs__msg__ProgramInformation * value =
    (ur_dashboard_msgs__msg__ProgramInformation *)(untyped_value);
  *value = *item;
}

void ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__assign_function__GetPrograms_Response__programs(
  void * untyped_member, size_t index, const void * untyped_value)
{
  ur_dashboard_msgs__msg__ProgramInformation * item =
    ((ur_dashboard_msgs__msg__ProgramInformation *)
    ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__get_function__GetPrograms_Response__programs(untyped_member, index));
  const ur_dashboard_msgs__msg__ProgramInformation * value =
    (const ur_dashboard_msgs__msg__ProgramInformation *)(untyped_value);
  *item = *value;
}

bool ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__resize_function__GetPrograms_Response__programs(
  void * untyped_member, size_t size)
{
  ur_dashboard_msgs__msg__ProgramInformation__Sequence * member =
    (ur_dashboard_msgs__msg__ProgramInformation__Sequence *)(untyped_member);
  ur_dashboard_msgs__msg__ProgramInformation__Sequence__fini(member);
  return ur_dashboard_msgs__msg__ProgramInformation__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_message_member_array[3] = {
  {
    "programs",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur_dashboard_msgs__srv__GetPrograms_Response, programs),  // bytes offset in struct
    NULL,  // default value
    ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__size_function__GetPrograms_Response__programs,  // size() function pointer
    ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__get_const_function__GetPrograms_Response__programs,  // get_const(index) function pointer
    ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__get_function__GetPrograms_Response__programs,  // get(index) function pointer
    ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__fetch_function__GetPrograms_Response__programs,  // fetch(index, &value) function pointer
    ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__assign_function__GetPrograms_Response__programs,  // assign(index, value) function pointer
    ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__resize_function__GetPrograms_Response__programs  // resize(index) function pointer
  },
  {
    "answer",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur_dashboard_msgs__srv__GetPrograms_Response, answer),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "success",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ur_dashboard_msgs__srv__GetPrograms_Response, success),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_message_members = {
  "ur_dashboard_msgs__srv",  // message namespace
  "GetPrograms_Response",  // message name
  3,  // number of fields
  sizeof(ur_dashboard_msgs__srv__GetPrograms_Response),
  ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_message_member_array,  // message members
  ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_message_type_support_handle = {
  0,
  &ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur_dashboard_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur_dashboard_msgs, srv, GetPrograms_Response)() {
  ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur_dashboard_msgs, msg, ProgramInformation)();
  if (!ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_message_type_support_handle.typesupport_identifier) {
    ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ur_dashboard_msgs__srv__GetPrograms_Response__rosidl_typesupport_introspection_c__GetPrograms_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "ur_dashboard_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "ur_dashboard_msgs/srv/detail/get_programs__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers ur_dashboard_msgs__srv__detail__get_programs__rosidl_typesupport_introspection_c__GetPrograms_service_members = {
  "ur_dashboard_msgs__srv",  // service namespace
  "GetPrograms",  // service name
  // these two fields are initialized below on the first access
  NULL,  // request message
  // ur_dashboard_msgs__srv__detail__get_programs__rosidl_typesupport_introspection_c__GetPrograms_Request_message_type_support_handle,
  NULL  // response message
  // ur_dashboard_msgs__srv__detail__get_programs__rosidl_typesupport_introspection_c__GetPrograms_Response_message_type_support_handle
};

static rosidl_service_type_support_t ur_dashboard_msgs__srv__detail__get_programs__rosidl_typesupport_introspection_c__GetPrograms_service_type_support_handle = {
  0,
  &ur_dashboard_msgs__srv__detail__get_programs__rosidl_typesupport_introspection_c__GetPrograms_service_members,
  get_service_typesupport_handle_function,
};

// Forward declaration of request/response type support functions
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur_dashboard_msgs, srv, GetPrograms_Request)();

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur_dashboard_msgs, srv, GetPrograms_Response)();

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ur_dashboard_msgs
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur_dashboard_msgs, srv, GetPrograms)() {
  if (!ur_dashboard_msgs__srv__detail__get_programs__rosidl_typesupport_introspection_c__GetPrograms_service_type_support_handle.typesupport_identifier) {
    ur_dashboard_msgs__srv__detail__get_programs__rosidl_typesupport_introspection_c__GetPrograms_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)ur_dashboard_msgs__srv__detail__get_programs__rosidl_typesupport_introspection_c__GetPrograms_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur_dashboard_msgs, srv, GetPrograms_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ur_dashboard_msgs, srv, GetPrograms_Response)()->data;
  }

  return &ur_dashboard_msgs__srv__detail__get_programs__rosidl_typesupport_introspection_c__GetPrograms_service_type_support_handle;
}
