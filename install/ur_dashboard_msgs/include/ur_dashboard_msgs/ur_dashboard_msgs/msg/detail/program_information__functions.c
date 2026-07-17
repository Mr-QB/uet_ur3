// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from ur_dashboard_msgs:msg/ProgramInformation.idl
// generated code does not contain a copyright notice
#include "ur_dashboard_msgs/msg/detail/program_information__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `name`
// Member `description`
// Member `program_state`
#include "rosidl_runtime_c/string_functions.h"

bool
ur_dashboard_msgs__msg__ProgramInformation__init(ur_dashboard_msgs__msg__ProgramInformation * msg)
{
  if (!msg) {
    return false;
  }
  // name
  if (!rosidl_runtime_c__String__init(&msg->name)) {
    ur_dashboard_msgs__msg__ProgramInformation__fini(msg);
    return false;
  }
  // description
  if (!rosidl_runtime_c__String__init(&msg->description)) {
    ur_dashboard_msgs__msg__ProgramInformation__fini(msg);
    return false;
  }
  // created_date
  // last_saved_date
  // last_modified_date
  // program_state
  if (!rosidl_runtime_c__String__init(&msg->program_state)) {
    ur_dashboard_msgs__msg__ProgramInformation__fini(msg);
    return false;
  }
  return true;
}

void
ur_dashboard_msgs__msg__ProgramInformation__fini(ur_dashboard_msgs__msg__ProgramInformation * msg)
{
  if (!msg) {
    return;
  }
  // name
  rosidl_runtime_c__String__fini(&msg->name);
  // description
  rosidl_runtime_c__String__fini(&msg->description);
  // created_date
  // last_saved_date
  // last_modified_date
  // program_state
  rosidl_runtime_c__String__fini(&msg->program_state);
}

bool
ur_dashboard_msgs__msg__ProgramInformation__are_equal(const ur_dashboard_msgs__msg__ProgramInformation * lhs, const ur_dashboard_msgs__msg__ProgramInformation * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // name
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->name), &(rhs->name)))
  {
    return false;
  }
  // description
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->description), &(rhs->description)))
  {
    return false;
  }
  // created_date
  if (lhs->created_date != rhs->created_date) {
    return false;
  }
  // last_saved_date
  if (lhs->last_saved_date != rhs->last_saved_date) {
    return false;
  }
  // last_modified_date
  if (lhs->last_modified_date != rhs->last_modified_date) {
    return false;
  }
  // program_state
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->program_state), &(rhs->program_state)))
  {
    return false;
  }
  return true;
}

bool
ur_dashboard_msgs__msg__ProgramInformation__copy(
  const ur_dashboard_msgs__msg__ProgramInformation * input,
  ur_dashboard_msgs__msg__ProgramInformation * output)
{
  if (!input || !output) {
    return false;
  }
  // name
  if (!rosidl_runtime_c__String__copy(
      &(input->name), &(output->name)))
  {
    return false;
  }
  // description
  if (!rosidl_runtime_c__String__copy(
      &(input->description), &(output->description)))
  {
    return false;
  }
  // created_date
  output->created_date = input->created_date;
  // last_saved_date
  output->last_saved_date = input->last_saved_date;
  // last_modified_date
  output->last_modified_date = input->last_modified_date;
  // program_state
  if (!rosidl_runtime_c__String__copy(
      &(input->program_state), &(output->program_state)))
  {
    return false;
  }
  return true;
}

ur_dashboard_msgs__msg__ProgramInformation *
ur_dashboard_msgs__msg__ProgramInformation__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__msg__ProgramInformation * msg = (ur_dashboard_msgs__msg__ProgramInformation *)allocator.allocate(sizeof(ur_dashboard_msgs__msg__ProgramInformation), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(ur_dashboard_msgs__msg__ProgramInformation));
  bool success = ur_dashboard_msgs__msg__ProgramInformation__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
ur_dashboard_msgs__msg__ProgramInformation__destroy(ur_dashboard_msgs__msg__ProgramInformation * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    ur_dashboard_msgs__msg__ProgramInformation__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
ur_dashboard_msgs__msg__ProgramInformation__Sequence__init(ur_dashboard_msgs__msg__ProgramInformation__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__msg__ProgramInformation * data = NULL;

  if (size) {
    data = (ur_dashboard_msgs__msg__ProgramInformation *)allocator.zero_allocate(size, sizeof(ur_dashboard_msgs__msg__ProgramInformation), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = ur_dashboard_msgs__msg__ProgramInformation__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        ur_dashboard_msgs__msg__ProgramInformation__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
ur_dashboard_msgs__msg__ProgramInformation__Sequence__fini(ur_dashboard_msgs__msg__ProgramInformation__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      ur_dashboard_msgs__msg__ProgramInformation__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

ur_dashboard_msgs__msg__ProgramInformation__Sequence *
ur_dashboard_msgs__msg__ProgramInformation__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__msg__ProgramInformation__Sequence * array = (ur_dashboard_msgs__msg__ProgramInformation__Sequence *)allocator.allocate(sizeof(ur_dashboard_msgs__msg__ProgramInformation__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = ur_dashboard_msgs__msg__ProgramInformation__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
ur_dashboard_msgs__msg__ProgramInformation__Sequence__destroy(ur_dashboard_msgs__msg__ProgramInformation__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    ur_dashboard_msgs__msg__ProgramInformation__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
ur_dashboard_msgs__msg__ProgramInformation__Sequence__are_equal(const ur_dashboard_msgs__msg__ProgramInformation__Sequence * lhs, const ur_dashboard_msgs__msg__ProgramInformation__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!ur_dashboard_msgs__msg__ProgramInformation__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
ur_dashboard_msgs__msg__ProgramInformation__Sequence__copy(
  const ur_dashboard_msgs__msg__ProgramInformation__Sequence * input,
  ur_dashboard_msgs__msg__ProgramInformation__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(ur_dashboard_msgs__msg__ProgramInformation);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    ur_dashboard_msgs__msg__ProgramInformation * data =
      (ur_dashboard_msgs__msg__ProgramInformation *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!ur_dashboard_msgs__msg__ProgramInformation__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          ur_dashboard_msgs__msg__ProgramInformation__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!ur_dashboard_msgs__msg__ProgramInformation__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
