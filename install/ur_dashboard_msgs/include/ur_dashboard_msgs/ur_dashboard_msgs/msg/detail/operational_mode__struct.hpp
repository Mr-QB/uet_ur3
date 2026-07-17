// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ur_dashboard_msgs:msg/OperationalMode.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__OPERATIONAL_MODE__STRUCT_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__OPERATIONAL_MODE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__msg__OperationalMode __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__msg__OperationalMode __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct OperationalMode_
{
  using Type = OperationalMode_<ContainerAllocator>;

  explicit OperationalMode_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->mode = "";
    }
  }

  explicit OperationalMode_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : mode(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->mode = "";
    }
  }

  // field types and members
  using _mode_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _mode_type mode;

  // setters for named parameter idiom
  Type & set__mode(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->mode = _arg;
    return *this;
  }

  // constant declarations
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> MANUAL;
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> AUTOMATIC;

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::msg::OperationalMode_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::msg::OperationalMode_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::msg::OperationalMode_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::msg::OperationalMode_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::msg::OperationalMode_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::msg::OperationalMode_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::msg::OperationalMode_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::msg::OperationalMode_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::msg::OperationalMode_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::msg::OperationalMode_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__msg__OperationalMode
    std::shared_ptr<ur_dashboard_msgs::msg::OperationalMode_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__msg__OperationalMode
    std::shared_ptr<ur_dashboard_msgs::msg::OperationalMode_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const OperationalMode_ & other) const
  {
    if (this->mode != other.mode) {
      return false;
    }
    return true;
  }
  bool operator!=(const OperationalMode_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct OperationalMode_

// alias to use template instance with default allocator
using OperationalMode =
  ur_dashboard_msgs::msg::OperationalMode_<std::allocator<void>>;

// constant definitions
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
OperationalMode_<ContainerAllocator>::MANUAL = "MANUAL";
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
OperationalMode_<ContainerAllocator>::AUTOMATIC = "AUTOMATIC";

}  // namespace msg

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__OPERATIONAL_MODE__STRUCT_HPP_
