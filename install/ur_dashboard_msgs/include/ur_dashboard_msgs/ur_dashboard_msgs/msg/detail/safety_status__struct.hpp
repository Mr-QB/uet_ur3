// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ur_dashboard_msgs:msg/SafetyStatus.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__SAFETY_STATUS__STRUCT_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__SAFETY_STATUS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__msg__SafetyStatus __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__msg__SafetyStatus __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct SafetyStatus_
{
  using Type = SafetyStatus_<ContainerAllocator>;

  explicit SafetyStatus_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->status = 0;
    }
  }

  explicit SafetyStatus_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->status = 0;
    }
  }

  // field types and members
  using _status_type =
    int8_t;
  _status_type status;

  // setters for named parameter idiom
  Type & set__status(
    const int8_t & _arg)
  {
    this->status = _arg;
    return *this;
  }

  // constant declarations
  static constexpr int8_t NORMAL =
    1;
  static constexpr int8_t REDUCED =
    2;
  static constexpr int8_t PROTECTIVE_STOP =
    3;
  static constexpr int8_t RECOVERY =
    4;
  static constexpr int8_t SAFEGUARD_STOP =
    5;
  static constexpr int8_t SYSTEM_EMERGENCY_STOP =
    6;
  static constexpr int8_t ROBOT_EMERGENCY_STOP =
    7;
  static constexpr int8_t VIOLATION =
    8;
  static constexpr int8_t FAULT =
    9;
  static constexpr int8_t VALIDATE_JOINT_ID =
    10;
  static constexpr int8_t UNDEFINED_SAFETY_MODE =
    11;
  static constexpr int8_t AUTOMATIC_MODE_SAFEGUARD_STOP =
    12;
  static constexpr int8_t SYSTEM_THREE_POSITION_ENABLING_STOP =
    13;
  static constexpr int8_t TP_THREE_POSITION_ENABLING_STOP =
    14;
  static constexpr int8_t IMMI_EMERGENCY_STOP =
    15;
  static constexpr int8_t IMMI_SAFEGUARD_STOP =
    16;
  static constexpr int8_t PROFISAFE_WAITING_FOR_PARAMETERS =
    17;
  static constexpr int8_t PROFISAFE_AUTOMATIC_MODE_SAFEGUARD_STOP =
    18;
  static constexpr int8_t PROFISAFE_SAFEGUARD_STOP =
    19;
  static constexpr int8_t PROFISAFE_EMERGENCY_STOP =
    20;
  static constexpr int8_t SAFETY_API_SAFEGUARD_STOP =
    22;
  static constexpr int8_t IO_PLANE_STOP =
    23;

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::msg::SafetyStatus_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::msg::SafetyStatus_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::msg::SafetyStatus_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::msg::SafetyStatus_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::msg::SafetyStatus_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::msg::SafetyStatus_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::msg::SafetyStatus_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::msg::SafetyStatus_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::msg::SafetyStatus_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::msg::SafetyStatus_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__msg__SafetyStatus
    std::shared_ptr<ur_dashboard_msgs::msg::SafetyStatus_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__msg__SafetyStatus
    std::shared_ptr<ur_dashboard_msgs::msg::SafetyStatus_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SafetyStatus_ & other) const
  {
    if (this->status != other.status) {
      return false;
    }
    return true;
  }
  bool operator!=(const SafetyStatus_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SafetyStatus_

// alias to use template instance with default allocator
using SafetyStatus =
  ur_dashboard_msgs::msg::SafetyStatus_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::NORMAL;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::REDUCED;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::PROTECTIVE_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::RECOVERY;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::SAFEGUARD_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::SYSTEM_EMERGENCY_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::ROBOT_EMERGENCY_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::VIOLATION;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::FAULT;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::VALIDATE_JOINT_ID;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::UNDEFINED_SAFETY_MODE;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::AUTOMATIC_MODE_SAFEGUARD_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::SYSTEM_THREE_POSITION_ENABLING_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::TP_THREE_POSITION_ENABLING_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::IMMI_EMERGENCY_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::IMMI_SAFEGUARD_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::PROFISAFE_WAITING_FOR_PARAMETERS;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::PROFISAFE_AUTOMATIC_MODE_SAFEGUARD_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::PROFISAFE_SAFEGUARD_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::PROFISAFE_EMERGENCY_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::SAFETY_API_SAFEGUARD_STOP;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t SafetyStatus_<ContainerAllocator>::IO_PLANE_STOP;
#endif  // __cplusplus < 201703L

}  // namespace msg

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__SAFETY_STATUS__STRUCT_HPP_
