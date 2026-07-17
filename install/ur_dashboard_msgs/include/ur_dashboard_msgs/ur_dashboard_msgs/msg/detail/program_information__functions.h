// generated from rosidl_generator_c/resource/idl__functions.h.em
// with input from ur_dashboard_msgs:msg/ProgramInformation.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__FUNCTIONS_H_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__FUNCTIONS_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stdlib.h>

#include "rosidl_runtime_c/visibility_control.h"
#include "ur_dashboard_msgs/msg/rosidl_generator_c__visibility_control.h"

#include "ur_dashboard_msgs/msg/detail/program_information__struct.h"

/// Initialize msg/ProgramInformation message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * ur_dashboard_msgs__msg__ProgramInformation
 * )) before or use
 * ur_dashboard_msgs__msg__ProgramInformation__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_ur_dashboard_msgs
bool
ur_dashboard_msgs__msg__ProgramInformation__init(ur_dashboard_msgs__msg__ProgramInformation * msg);

/// Finalize msg/ProgramInformation message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_ur_dashboard_msgs
void
ur_dashboard_msgs__msg__ProgramInformation__fini(ur_dashboard_msgs__msg__ProgramInformation * msg);

/// Create msg/ProgramInformation message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * ur_dashboard_msgs__msg__ProgramInformation__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_ur_dashboard_msgs
ur_dashboard_msgs__msg__ProgramInformation *
ur_dashboard_msgs__msg__ProgramInformation__create();

/// Destroy msg/ProgramInformation message.
/**
 * It calls
 * ur_dashboard_msgs__msg__ProgramInformation__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_ur_dashboard_msgs
void
ur_dashboard_msgs__msg__ProgramInformation__destroy(ur_dashboard_msgs__msg__ProgramInformation * msg);

/// Check for msg/ProgramInformation message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_ur_dashboard_msgs
bool
ur_dashboard_msgs__msg__ProgramInformation__are_equal(const ur_dashboard_msgs__msg__ProgramInformation * lhs, const ur_dashboard_msgs__msg__ProgramInformation * rhs);

/// Copy a msg/ProgramInformation message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_ur_dashboard_msgs
bool
ur_dashboard_msgs__msg__ProgramInformation__copy(
  const ur_dashboard_msgs__msg__ProgramInformation * input,
  ur_dashboard_msgs__msg__ProgramInformation * output);

/// Initialize array of msg/ProgramInformation messages.
/**
 * It allocates the memory for the number of elements and calls
 * ur_dashboard_msgs__msg__ProgramInformation__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_ur_dashboard_msgs
bool
ur_dashboard_msgs__msg__ProgramInformation__Sequence__init(ur_dashboard_msgs__msg__ProgramInformation__Sequence * array, size_t size);

/// Finalize array of msg/ProgramInformation messages.
/**
 * It calls
 * ur_dashboard_msgs__msg__ProgramInformation__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_ur_dashboard_msgs
void
ur_dashboard_msgs__msg__ProgramInformation__Sequence__fini(ur_dashboard_msgs__msg__ProgramInformation__Sequence * array);

/// Create array of msg/ProgramInformation messages.
/**
 * It allocates the memory for the array and calls
 * ur_dashboard_msgs__msg__ProgramInformation__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_ur_dashboard_msgs
ur_dashboard_msgs__msg__ProgramInformation__Sequence *
ur_dashboard_msgs__msg__ProgramInformation__Sequence__create(size_t size);

/// Destroy array of msg/ProgramInformation messages.
/**
 * It calls
 * ur_dashboard_msgs__msg__ProgramInformation__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_ur_dashboard_msgs
void
ur_dashboard_msgs__msg__ProgramInformation__Sequence__destroy(ur_dashboard_msgs__msg__ProgramInformation__Sequence * array);

/// Check for msg/ProgramInformation message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_ur_dashboard_msgs
bool
ur_dashboard_msgs__msg__ProgramInformation__Sequence__are_equal(const ur_dashboard_msgs__msg__ProgramInformation__Sequence * lhs, const ur_dashboard_msgs__msg__ProgramInformation__Sequence * rhs);

/// Copy an array of msg/ProgramInformation messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_ur_dashboard_msgs
bool
ur_dashboard_msgs__msg__ProgramInformation__Sequence__copy(
  const ur_dashboard_msgs__msg__ProgramInformation__Sequence * input,
  ur_dashboard_msgs__msg__ProgramInformation__Sequence * output);

#ifdef __cplusplus
}
#endif

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__FUNCTIONS_H_
