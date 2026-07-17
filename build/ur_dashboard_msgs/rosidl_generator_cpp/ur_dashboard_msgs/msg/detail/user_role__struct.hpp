// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ur_dashboard_msgs:msg/UserRole.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__USER_ROLE__STRUCT_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__USER_ROLE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__msg__UserRole __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__msg__UserRole __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct UserRole_
{
  using Type = UserRole_<ContainerAllocator>;

  explicit UserRole_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->role = "";
    }
  }

  explicit UserRole_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : role(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->role = "";
    }
  }

  // field types and members
  using _role_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _role_type role;

  // setters for named parameter idiom
  Type & set__role(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->role = _arg;
    return *this;
  }

  // constant declarations
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> PROGRAMMER;
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> OPERATOR;
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> NONE;
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> LOCKED;
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> RESTRICTED;

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::msg::UserRole_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::msg::UserRole_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::msg::UserRole_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::msg::UserRole_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::msg::UserRole_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::msg::UserRole_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::msg::UserRole_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::msg::UserRole_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::msg::UserRole_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::msg::UserRole_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__msg__UserRole
    std::shared_ptr<ur_dashboard_msgs::msg::UserRole_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__msg__UserRole
    std::shared_ptr<ur_dashboard_msgs::msg::UserRole_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const UserRole_ & other) const
  {
    if (this->role != other.role) {
      return false;
    }
    return true;
  }
  bool operator!=(const UserRole_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct UserRole_

// alias to use template instance with default allocator
using UserRole =
  ur_dashboard_msgs::msg::UserRole_<std::allocator<void>>;

// constant definitions
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
UserRole_<ContainerAllocator>::PROGRAMMER = "PROGRAMMER";
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
UserRole_<ContainerAllocator>::OPERATOR = "OPERATOR";
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
UserRole_<ContainerAllocator>::NONE = "NONE";
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
UserRole_<ContainerAllocator>::LOCKED = "LOCKED";
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
UserRole_<ContainerAllocator>::RESTRICTED = "RESTRICTED";

}  // namespace msg

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__USER_ROLE__STRUCT_HPP_
