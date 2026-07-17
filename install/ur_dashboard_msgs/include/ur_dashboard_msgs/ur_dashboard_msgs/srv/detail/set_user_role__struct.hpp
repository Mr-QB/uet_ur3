// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ur_dashboard_msgs:srv/SetUserRole.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__SET_USER_ROLE__STRUCT_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__SET_USER_ROLE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'user_role'
#include "ur_dashboard_msgs/msg/detail/user_role__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__srv__SetUserRole_Request __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__srv__SetUserRole_Request __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SetUserRole_Request_
{
  using Type = SetUserRole_Request_<ContainerAllocator>;

  explicit SetUserRole_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : user_role(_init)
  {
    (void)_init;
  }

  explicit SetUserRole_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : user_role(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _user_role_type =
    ur_dashboard_msgs::msg::UserRole_<ContainerAllocator>;
  _user_role_type user_role;

  // setters for named parameter idiom
  Type & set__user_role(
    const ur_dashboard_msgs::msg::UserRole_<ContainerAllocator> & _arg)
  {
    this->user_role = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::srv::SetUserRole_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::srv::SetUserRole_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::SetUserRole_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::SetUserRole_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::SetUserRole_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::SetUserRole_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::SetUserRole_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::SetUserRole_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::SetUserRole_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::SetUserRole_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__srv__SetUserRole_Request
    std::shared_ptr<ur_dashboard_msgs::srv::SetUserRole_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__srv__SetUserRole_Request
    std::shared_ptr<ur_dashboard_msgs::srv::SetUserRole_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SetUserRole_Request_ & other) const
  {
    if (this->user_role != other.user_role) {
      return false;
    }
    return true;
  }
  bool operator!=(const SetUserRole_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SetUserRole_Request_

// alias to use template instance with default allocator
using SetUserRole_Request =
  ur_dashboard_msgs::srv::SetUserRole_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ur_dashboard_msgs


#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__srv__SetUserRole_Response __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__srv__SetUserRole_Response __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SetUserRole_Response_
{
  using Type = SetUserRole_Response_<ContainerAllocator>;

  explicit SetUserRole_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->answer = "";
    }
  }

  explicit SetUserRole_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : answer(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->answer = "";
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;
  using _answer_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _answer_type answer;

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

  // constant declarations

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::srv::SetUserRole_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::srv::SetUserRole_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::SetUserRole_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::SetUserRole_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::SetUserRole_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::SetUserRole_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::SetUserRole_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::SetUserRole_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::SetUserRole_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::SetUserRole_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__srv__SetUserRole_Response
    std::shared_ptr<ur_dashboard_msgs::srv::SetUserRole_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__srv__SetUserRole_Response
    std::shared_ptr<ur_dashboard_msgs::srv::SetUserRole_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SetUserRole_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->answer != other.answer) {
      return false;
    }
    return true;
  }
  bool operator!=(const SetUserRole_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SetUserRole_Response_

// alias to use template instance with default allocator
using SetUserRole_Response =
  ur_dashboard_msgs::srv::SetUserRole_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ur_dashboard_msgs

namespace ur_dashboard_msgs
{

namespace srv
{

struct SetUserRole
{
  using Request = ur_dashboard_msgs::srv::SetUserRole_Request;
  using Response = ur_dashboard_msgs::srv::SetUserRole_Response;
};

}  // namespace srv

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__SET_USER_ROLE__STRUCT_HPP_
