// generated from rosidl_typesupport_fastrtps_cpp/resource/idl__type_support.cpp.em
// with input from ur_dashboard_msgs:msg/ProgramInformation.idl
// generated code does not contain a copyright notice
#include "ur_dashboard_msgs/msg/detail/program_information__rosidl_typesupport_fastrtps_cpp.hpp"
#include "ur_dashboard_msgs/msg/detail/program_information__struct.hpp"

#include <limits>
#include <stdexcept>
#include <string>
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_fastrtps_cpp/identifier.hpp"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support_decl.hpp"
#include "rosidl_typesupport_fastrtps_cpp/wstring_conversion.hpp"
#include "fastcdr/Cdr.h"


// forward declaration of message dependencies and their conversion functions

namespace ur_dashboard_msgs
{

namespace msg
{

namespace typesupport_fastrtps_cpp
{

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_ur_dashboard_msgs
cdr_serialize(
  const ur_dashboard_msgs::msg::ProgramInformation & ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  // Member: name
  cdr << ros_message.name;
  // Member: description
  cdr << ros_message.description;
  // Member: created_date
  cdr << ros_message.created_date;
  // Member: last_saved_date
  cdr << ros_message.last_saved_date;
  // Member: last_modified_date
  cdr << ros_message.last_modified_date;
  // Member: program_state
  cdr << ros_message.program_state;
  return true;
}

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_ur_dashboard_msgs
cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  ur_dashboard_msgs::msg::ProgramInformation & ros_message)
{
  // Member: name
  cdr >> ros_message.name;

  // Member: description
  cdr >> ros_message.description;

  // Member: created_date
  cdr >> ros_message.created_date;

  // Member: last_saved_date
  cdr >> ros_message.last_saved_date;

  // Member: last_modified_date
  cdr >> ros_message.last_modified_date;

  // Member: program_state
  cdr >> ros_message.program_state;

  return true;
}  // NOLINT(readability/fn_size)

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_ur_dashboard_msgs
get_serialized_size(
  const ur_dashboard_msgs::msg::ProgramInformation & ros_message,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // Member: name
  current_alignment += padding +
    eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
    (ros_message.name.size() + 1);
  // Member: description
  current_alignment += padding +
    eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
    (ros_message.description.size() + 1);
  // Member: created_date
  {
    size_t item_size = sizeof(ros_message.created_date);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: last_saved_date
  {
    size_t item_size = sizeof(ros_message.last_saved_date);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: last_modified_date
  {
    size_t item_size = sizeof(ros_message.last_modified_date);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: program_state
  current_alignment += padding +
    eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
    (ros_message.program_state.size() + 1);

  return current_alignment - initial_alignment;
}

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_ur_dashboard_msgs
max_serialized_size_ProgramInformation(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  size_t last_member_size = 0;
  (void)last_member_size;
  (void)padding;
  (void)wchar_size;

  full_bounded = true;
  is_plain = true;


  // Member: name
  {
    size_t array_size = 1;

    full_bounded = false;
    is_plain = false;
    for (size_t index = 0; index < array_size; ++index) {
      current_alignment += padding +
        eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
        1;
    }
  }

  // Member: description
  {
    size_t array_size = 1;

    full_bounded = false;
    is_plain = false;
    for (size_t index = 0; index < array_size; ++index) {
      current_alignment += padding +
        eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
        1;
    }
  }

  // Member: created_date
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint64_t);
    current_alignment += array_size * sizeof(uint64_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint64_t));
  }

  // Member: last_saved_date
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint64_t);
    current_alignment += array_size * sizeof(uint64_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint64_t));
  }

  // Member: last_modified_date
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint64_t);
    current_alignment += array_size * sizeof(uint64_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint64_t));
  }

  // Member: program_state
  {
    size_t array_size = 1;

    full_bounded = false;
    is_plain = false;
    for (size_t index = 0; index < array_size; ++index) {
      current_alignment += padding +
        eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
        1;
    }
  }

  size_t ret_val = current_alignment - initial_alignment;
  if (is_plain) {
    // All members are plain, and type is not empty.
    // We still need to check that the in-memory alignment
    // is the same as the CDR mandated alignment.
    using DataType = ur_dashboard_msgs::msg::ProgramInformation;
    is_plain =
      (
      offsetof(DataType, program_state) +
      last_member_size
      ) == ret_val;
  }

  return ret_val;
}

static bool _ProgramInformation__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  auto typed_message =
    static_cast<const ur_dashboard_msgs::msg::ProgramInformation *>(
    untyped_ros_message);
  return cdr_serialize(*typed_message, cdr);
}

static bool _ProgramInformation__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  auto typed_message =
    static_cast<ur_dashboard_msgs::msg::ProgramInformation *>(
    untyped_ros_message);
  return cdr_deserialize(cdr, *typed_message);
}

static uint32_t _ProgramInformation__get_serialized_size(
  const void * untyped_ros_message)
{
  auto typed_message =
    static_cast<const ur_dashboard_msgs::msg::ProgramInformation *>(
    untyped_ros_message);
  return static_cast<uint32_t>(get_serialized_size(*typed_message, 0));
}

static size_t _ProgramInformation__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_ProgramInformation(full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}

static message_type_support_callbacks_t _ProgramInformation__callbacks = {
  "ur_dashboard_msgs::msg",
  "ProgramInformation",
  _ProgramInformation__cdr_serialize,
  _ProgramInformation__cdr_deserialize,
  _ProgramInformation__get_serialized_size,
  _ProgramInformation__max_serialized_size
};

static rosidl_message_type_support_t _ProgramInformation__handle = {
  rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
  &_ProgramInformation__callbacks,
  get_message_typesupport_handle_function,
};

}  // namespace typesupport_fastrtps_cpp

}  // namespace msg

}  // namespace ur_dashboard_msgs

namespace rosidl_typesupport_fastrtps_cpp
{

template<>
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_EXPORT_ur_dashboard_msgs
const rosidl_message_type_support_t *
get_message_type_support_handle<ur_dashboard_msgs::msg::ProgramInformation>()
{
  return &ur_dashboard_msgs::msg::typesupport_fastrtps_cpp::_ProgramInformation__handle;
}

}  // namespace rosidl_typesupport_fastrtps_cpp

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, ur_dashboard_msgs, msg, ProgramInformation)() {
  return &ur_dashboard_msgs::msg::typesupport_fastrtps_cpp::_ProgramInformation__handle;
}

#ifdef __cplusplus
}
#endif
