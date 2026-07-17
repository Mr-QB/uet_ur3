// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ur_dashboard_msgs:msg/VersionInformation.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__VERSION_INFORMATION__STRUCT_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__VERSION_INFORMATION__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__msg__VersionInformation __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__msg__VersionInformation __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct VersionInformation_
{
  using Type = VersionInformation_<ContainerAllocator>;

  explicit VersionInformation_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->major = 0ul;
      this->minor = 0ul;
      this->bugfix = 0ul;
      this->build = 0ul;
    }
  }

  explicit VersionInformation_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->major = 0ul;
      this->minor = 0ul;
      this->bugfix = 0ul;
      this->build = 0ul;
    }
  }

  // field types and members
  using _major_type =
    uint32_t;
  _major_type major;
  using _minor_type =
    uint32_t;
  _minor_type minor;
  using _bugfix_type =
    uint32_t;
  _bugfix_type bugfix;
  using _build_type =
    uint32_t;
  _build_type build;

  // setters for named parameter idiom
  Type & set__major(
    const uint32_t & _arg)
  {
    this->major = _arg;
    return *this;
  }
  Type & set__minor(
    const uint32_t & _arg)
  {
    this->minor = _arg;
    return *this;
  }
  Type & set__bugfix(
    const uint32_t & _arg)
  {
    this->bugfix = _arg;
    return *this;
  }
  Type & set__build(
    const uint32_t & _arg)
  {
    this->build = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::msg::VersionInformation_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::msg::VersionInformation_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::msg::VersionInformation_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::msg::VersionInformation_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::msg::VersionInformation_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::msg::VersionInformation_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::msg::VersionInformation_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::msg::VersionInformation_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::msg::VersionInformation_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::msg::VersionInformation_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__msg__VersionInformation
    std::shared_ptr<ur_dashboard_msgs::msg::VersionInformation_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__msg__VersionInformation
    std::shared_ptr<ur_dashboard_msgs::msg::VersionInformation_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const VersionInformation_ & other) const
  {
    if (this->major != other.major) {
      return false;
    }
    if (this->minor != other.minor) {
      return false;
    }
    if (this->bugfix != other.bugfix) {
      return false;
    }
    if (this->build != other.build) {
      return false;
    }
    return true;
  }
  bool operator!=(const VersionInformation_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct VersionInformation_

// alias to use template instance with default allocator
using VersionInformation =
  ur_dashboard_msgs::msg::VersionInformation_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__VERSION_INFORMATION__STRUCT_HPP_
