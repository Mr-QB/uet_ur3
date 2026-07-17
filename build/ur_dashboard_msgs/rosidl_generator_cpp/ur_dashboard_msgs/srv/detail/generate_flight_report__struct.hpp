// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ur_dashboard_msgs:srv/GenerateFlightReport.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_FLIGHT_REPORT__STRUCT_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_FLIGHT_REPORT__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__srv__GenerateFlightReport_Request __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__srv__GenerateFlightReport_Request __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct GenerateFlightReport_Request_
{
  using Type = GenerateFlightReport_Request_<ContainerAllocator>;

  explicit GenerateFlightReport_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::DEFAULTS_ONLY == _init)
    {
      this->report_type = "SYSTEM";
    } else if (rosidl_runtime_cpp::MessageInitialization::ZERO == _init) {
      this->report_type = "";
    }
  }

  explicit GenerateFlightReport_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : report_type(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::DEFAULTS_ONLY == _init)
    {
      this->report_type = "SYSTEM";
    } else if (rosidl_runtime_cpp::MessageInitialization::ZERO == _init) {
      this->report_type = "";
    }
  }

  // field types and members
  using _report_type_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _report_type_type report_type;

  // setters for named parameter idiom
  Type & set__report_type(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->report_type = _arg;
    return *this;
  }

  // constant declarations
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> CONTROLLER;
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> SOFTWARE;
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> SYSTEM;

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::srv::GenerateFlightReport_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::srv::GenerateFlightReport_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::GenerateFlightReport_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::GenerateFlightReport_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__srv__GenerateFlightReport_Request
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__srv__GenerateFlightReport_Request
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const GenerateFlightReport_Request_ & other) const
  {
    if (this->report_type != other.report_type) {
      return false;
    }
    return true;
  }
  bool operator!=(const GenerateFlightReport_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct GenerateFlightReport_Request_

// alias to use template instance with default allocator
using GenerateFlightReport_Request =
  ur_dashboard_msgs::srv::GenerateFlightReport_Request_<std::allocator<void>>;

// constant definitions
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
GenerateFlightReport_Request_<ContainerAllocator>::CONTROLLER = "controller";
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
GenerateFlightReport_Request_<ContainerAllocator>::SOFTWARE = "software";
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
GenerateFlightReport_Request_<ContainerAllocator>::SYSTEM = "system";

}  // namespace srv

}  // namespace ur_dashboard_msgs


#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__srv__GenerateFlightReport_Response __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__srv__GenerateFlightReport_Response __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct GenerateFlightReport_Response_
{
  using Type = GenerateFlightReport_Response_<ContainerAllocator>;

  explicit GenerateFlightReport_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->answer = "";
      this->report_id = "";
    }
  }

  explicit GenerateFlightReport_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : answer(_alloc),
    report_id(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->answer = "";
      this->report_id = "";
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;
  using _answer_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _answer_type answer;
  using _report_id_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _report_id_type report_id;

  // setters for named parameter idiom
  Type & set__success(
    const bool & _arg)
  {
    this->success = _arg;
    return *this;
  }
  Type & set__answer(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->answer = _arg;
    return *this;
  }
  Type & set__report_id(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->report_id = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::srv::GenerateFlightReport_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::srv::GenerateFlightReport_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::GenerateFlightReport_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::GenerateFlightReport_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__srv__GenerateFlightReport_Response
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__srv__GenerateFlightReport_Response
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateFlightReport_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const GenerateFlightReport_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->answer != other.answer) {
      return false;
    }
    if (this->report_id != other.report_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const GenerateFlightReport_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct GenerateFlightReport_Response_

// alias to use template instance with default allocator
using GenerateFlightReport_Response =
  ur_dashboard_msgs::srv::GenerateFlightReport_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ur_dashboard_msgs

namespace ur_dashboard_msgs
{

namespace srv
{

struct GenerateFlightReport
{
  using Request = ur_dashboard_msgs::srv::GenerateFlightReport_Request;
  using Response = ur_dashboard_msgs::srv::GenerateFlightReport_Response;
};

}  // namespace srv

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_FLIGHT_REPORT__STRUCT_HPP_
