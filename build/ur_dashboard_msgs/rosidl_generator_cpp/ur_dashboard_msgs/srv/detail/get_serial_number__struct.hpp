// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ur_dashboard_msgs:srv/GetSerialNumber.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__GET_SERIAL_NUMBER__STRUCT_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__GET_SERIAL_NUMBER__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__srv__GetSerialNumber_Request __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__srv__GetSerialNumber_Request __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct GetSerialNumber_Request_
{
  using Type = GetSerialNumber_Request_<ContainerAllocator>;

  explicit GetSerialNumber_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->structure_needs_at_least_one_member = 0;
    }
  }

  explicit GetSerialNumber_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->structure_needs_at_least_one_member = 0;
    }
  }

  // field types and members
  using _structure_needs_at_least_one_member_type =
    uint8_t;
  _structure_needs_at_least_one_member_type structure_needs_at_least_one_member;


  // constant declarations

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::srv::GetSerialNumber_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::srv::GetSerialNumber_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::GetSerialNumber_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::GetSerialNumber_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__srv__GetSerialNumber_Request
    std::shared_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__srv__GetSerialNumber_Request
    std::shared_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const GetSerialNumber_Request_ & other) const
  {
    if (this->structure_needs_at_least_one_member != other.structure_needs_at_least_one_member) {
      return false;
    }
    return true;
  }
  bool operator!=(const GetSerialNumber_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct GetSerialNumber_Request_

// alias to use template instance with default allocator
using GetSerialNumber_Request =
  ur_dashboard_msgs::srv::GetSerialNumber_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ur_dashboard_msgs


#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__srv__GetSerialNumber_Response __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__srv__GetSerialNumber_Response __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct GetSerialNumber_Response_
{
  using Type = GetSerialNumber_Response_<ContainerAllocator>;

  explicit GetSerialNumber_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->serial_number = 0ull;
      this->answer = "";
      this->success = false;
    }
  }

  explicit GetSerialNumber_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : answer(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->serial_number = 0ull;
      this->answer = "";
      this->success = false;
    }
  }

  // field types and members
  using _serial_number_type =
    uint64_t;
  _serial_number_type serial_number;
  using _answer_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _answer_type answer;
  using _success_type =
    bool;
  _success_type success;

  // setters for named parameter idiom
  Type & set__serial_number(
    const uint64_t & _arg)
  {
    this->serial_number = _arg;
    return *this;
  }
  Type & set__answer(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->answer = _arg;
    return *this;
  }
  Type & set__success(
    const bool & _arg)
  {
    this->success = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::srv::GetSerialNumber_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::srv::GetSerialNumber_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::GetSerialNumber_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::GetSerialNumber_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__srv__GetSerialNumber_Response
    std::shared_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__srv__GetSerialNumber_Response
    std::shared_ptr<ur_dashboard_msgs::srv::GetSerialNumber_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const GetSerialNumber_Response_ & other) const
  {
    if (this->serial_number != other.serial_number) {
      return false;
    }
    if (this->answer != other.answer) {
      return false;
    }
    if (this->success != other.success) {
      return false;
    }
    return true;
  }
  bool operator!=(const GetSerialNumber_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct GetSerialNumber_Response_

// alias to use template instance with default allocator
using GetSerialNumber_Response =
  ur_dashboard_msgs::srv::GetSerialNumber_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ur_dashboard_msgs

namespace ur_dashboard_msgs
{

namespace srv
{

struct GetSerialNumber
{
  using Request = ur_dashboard_msgs::srv::GetSerialNumber_Request;
  using Response = ur_dashboard_msgs::srv::GetSerialNumber_Response;
};

}  // namespace srv

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__GET_SERIAL_NUMBER__STRUCT_HPP_
