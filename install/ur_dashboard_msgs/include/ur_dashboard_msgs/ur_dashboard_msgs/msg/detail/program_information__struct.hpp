// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ur_dashboard_msgs:msg/ProgramInformation.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__STRUCT_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ur_dashboard_msgs__msg__ProgramInformation __attribute__((deprecated))
#else
# define DEPRECATED__ur_dashboard_msgs__msg__ProgramInformation __declspec(deprecated)
#endif

namespace ur_dashboard_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct ProgramInformation_
{
  using Type = ProgramInformation_<ContainerAllocator>;

  explicit ProgramInformation_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->name = "";
      this->description = "";
      this->created_date = 0ull;
      this->last_saved_date = 0ull;
      this->last_modified_date = 0ull;
      this->program_state = "";
    }
  }

  explicit ProgramInformation_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : name(_alloc),
    description(_alloc),
    program_state(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->name = "";
      this->description = "";
      this->created_date = 0ull;
      this->last_saved_date = 0ull;
      this->last_modified_date = 0ull;
      this->program_state = "";
    }
  }

  // field types and members
  using _name_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _name_type name;
  using _description_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _description_type description;
  using _created_date_type =
    uint64_t;
  _created_date_type created_date;
  using _last_saved_date_type =
    uint64_t;
  _last_saved_date_type last_saved_date;
  using _last_modified_date_type =
    uint64_t;
  _last_modified_date_type last_modified_date;
  using _program_state_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _program_state_type program_state;

  // setters for named parameter idiom
  Type & set__name(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->name = _arg;
    return *this;
  }
  Type & set__description(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->description = _arg;
    return *this;
  }
  Type & set__created_date(
    const uint64_t & _arg)
  {
    this->created_date = _arg;
    return *this;
  }
  Type & set__last_saved_date(
    const uint64_t & _arg)
  {
    this->last_saved_date = _arg;
    return *this;
  }
  Type & set__last_modified_date(
    const uint64_t & _arg)
  {
    this->last_modified_date = _arg;
    return *this;
  }
  Type & set__program_state(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->program_state = _arg;
    return *this;
  }

  // constant declarations
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> PROGRAM_STATE_DRAFT;
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> PROGRAM_STATE_FINAL;

  // pointer types
  using RawPtr =
    ur_dashboard_msgs::msg::ProgramInformation_<ContainerAllocator> *;
  using ConstRawPtr =
    const ur_dashboard_msgs::msg::ProgramInformation_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ur_dashboard_msgs::msg::ProgramInformation_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ur_dashboard_msgs::msg::ProgramInformation_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::msg::ProgramInformation_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::msg::ProgramInformation_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ur_dashboard_msgs::msg::ProgramInformation_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ur_dashboard_msgs::msg::ProgramInformation_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ur_dashboard_msgs::msg::ProgramInformation_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ur_dashboard_msgs::msg::ProgramInformation_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ur_dashboard_msgs__msg__ProgramInformation
    std::shared_ptr<ur_dashboard_msgs::msg::ProgramInformation_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ur_dashboard_msgs__msg__ProgramInformation
    std::shared_ptr<ur_dashboard_msgs::msg::ProgramInformation_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ProgramInformation_ & other) const
  {
    if (this->name != other.name) {
      return false;
    }
    if (this->description != other.description) {
      return false;
    }
    if (this->created_date != other.created_date) {
      return false;
    }
    if (this->last_saved_date != other.last_saved_date) {
      return false;
    }
    if (this->last_modified_date != other.last_modified_date) {
      return false;
    }
    if (this->program_state != other.program_state) {
      return false;
    }
    return true;
  }
  bool operator!=(const ProgramInformation_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ProgramInformation_

// alias to use template instance with default allocator
using ProgramInformation =
  ur_dashboard_msgs::msg::ProgramInformation_<std::allocator<void>>;

// constant definitions
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
ProgramInformation_<ContainerAllocator>::PROGRAM_STATE_DRAFT = "DRAFT";
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
ProgramInformation_<ContainerAllocator>::PROGRAM_STATE_FINAL = "FINAL";

}  // namespace msg

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__STRUCT_HPP_
