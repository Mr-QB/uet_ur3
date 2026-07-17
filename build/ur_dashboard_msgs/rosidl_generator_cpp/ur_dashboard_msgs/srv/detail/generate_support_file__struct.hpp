// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ur_dashboard_msgs:srv/GenerateSupportFile.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_SUPPORT_FILE__STRUCT_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_SUPPORT_FILE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__srv__GenerateSupportFile_Request __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__srv__GenerateSupportFile_Request __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct GenerateSupportFile_Request_
{
  using Type = GenerateSupportFile_Request_<ContainerAllocator>;

  explicit GenerateSupportFile_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::DEFAULTS_ONLY == _init)
    {
      this->dir_path = ".";
    } else if (rosidl_runtime_cpp::MessageInitialization::ZERO == _init) {
      this->dir_path = "";
    }
  }

  explicit GenerateSupportFile_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : dir_path(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::DEFAULTS_ONLY == _init)
    {
      this->dir_path = ".";
    } else if (rosidl_runtime_cpp::MessageInitialization::ZERO == _init) {
      this->dir_path = "";
    }
  }

  // field types and members
  using _dir_path_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _dir_path_type dir_path;

  // setters for named parameter idiom
  Type & set__dir_path(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->dir_path = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::srv::GenerateSupportFile_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::srv::GenerateSupportFile_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::GenerateSupportFile_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::GenerateSupportFile_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__srv__GenerateSupportFile_Request
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__srv__GenerateSupportFile_Request
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const GenerateSupportFile_Request_ & other) const
  {
    if (this->dir_path != other.dir_path) {
      return false;
    }
    return true;
  }
  bool operator!=(const GenerateSupportFile_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct GenerateSupportFile_Request_

// alias to use template instance with default allocator
using GenerateSupportFile_Request =
  ur_dashboard_msgs::srv::GenerateSupportFile_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ur_dashboard_msgs


#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__srv__GenerateSupportFile_Response __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__srv__GenerateSupportFile_Response __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct GenerateSupportFile_Response_
{
  using Type = GenerateSupportFile_Response_<ContainerAllocator>;

  explicit GenerateSupportFile_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->answer = "";
      this->generated_file_name = "";
    }
  }

  explicit GenerateSupportFile_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : answer(_alloc),
    generated_file_name(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->answer = "";
      this->generated_file_name = "";
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;
  using _answer_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _answer_type answer;
  using _generated_file_name_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _generated_file_name_type generated_file_name;

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
  Type & set__generated_file_name(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->generated_file_name = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::srv::GenerateSupportFile_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::srv::GenerateSupportFile_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::GenerateSupportFile_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::srv::GenerateSupportFile_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__srv__GenerateSupportFile_Response
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__srv__GenerateSupportFile_Response
    std::shared_ptr<ur_dashboard_msgs::srv::GenerateSupportFile_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const GenerateSupportFile_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->answer != other.answer) {
      return false;
    }
    if (this->generated_file_name != other.generated_file_name) {
      return false;
    }
    return true;
  }
  bool operator!=(const GenerateSupportFile_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct GenerateSupportFile_Response_

// alias to use template instance with default allocator
using GenerateSupportFile_Response =
  ur_dashboard_msgs::srv::GenerateSupportFile_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ur_dashboard_msgs

namespace ur_dashboard_msgs
{

namespace srv
{

struct GenerateSupportFile
{
  using Request = ur_dashboard_msgs::srv::GenerateSupportFile_Request;
  using Response = ur_dashboard_msgs::srv::GenerateSupportFile_Response;
};

}  // namespace srv

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_SUPPORT_FILE__STRUCT_HPP_
