// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from ur_dashboard_msgs:msg/OperationalMode.idl
// generated code does not contain a copyright notice
#include "ur_dashboard_msgs/msg/detail/operational_mode__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `mode`
#include "rosidl_runtime_c/string_functions.h"

bool
ur_dashboard_msgs__msg__OperationalMode__init(ur_dashboard_msgs__msg__OperationalMode * msg)
{
  if (!msg) {
    return false;
  }
  // mode
  if (!rosidl_runtime_c__String__init(&msg->mode)) {
    ur_dashboard_msgs__msg__OperationalMode__fini(msg);
    return false;
  }
  return true;
}

void
ur_dashboard_msgs__msg__OperationalMode__fini(ur_dashboard_msgs__msg__OperationalMode * msg)
{
  if (!msg) {
    return;
  }
  // mode
  rosidl_runtime_c__String__fini(&msg->mode);
}

bool
ur_dashboard_msgs__msg__OperationalMode__are_equal(const ur_dashboard_msgs__msg__OperationalMode * lhs, const ur_dashboard_msgs__msg__OperationalMode * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // mode
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->mode), &(rhs->mode)))
  {
    return false;
  }
  return true;
}

bool
ur_dashboard_msgs__msg__OperationalMode__copy(
  const ur_dashboard_msgs__msg__OperationalMode * input,
  ur_dashboard_msgs__msg__OperationalMode * output)
{
  if (!input || !output) {
    return false;
  }
  // mode
  if (!rosidl_runtime_c__String__copy(
      &(input->mode), &(output->mode)))
  {
    return false;
  }
  return true;
}

ur_dashboard_msgs__msg__OperationalMode *
ur_dashboard_msgs__msg__OperationalMode__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__msg__OperationalMode * msg = (ur_dashboard_msgs__msg__OperationalMode *)allocator.allocate(sizeof(ur_dashboard_msgs__msg__OperationalMode), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(ur_dashboard_msgs__msg__OperationalMode));
  bool success = ur_dashboard_msgs__msg__OperationalMode__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
ur_dashboard_msgs__msg__OperationalMode__destroy(ur_dashboard_msgs__msg__OperationalMode * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    ur_dashboard_msgs__msg__OperationalMode__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
ur_dashboard_msgs__msg__OperationalMode__Sequence__init(ur_dashboard_msgs__msg__OperationalMode__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__msg__OperationalMode * data = NULL;

  if (size) {
    data = (ur_dashboard_msgs__msg__OperationalMode *)allocator.zero_allocate(size, sizeof(ur_dashboard_msgs__msg__OperationalMode), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = ur_dashboard_msgs__msg__OperationalMode__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        ur_dashboard_msgs__msg__OperationalMode__fini(&data[i - 1]);
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
ur_dashboard_msgs__msg__OperationalMode__Sequence__fini(ur_dashboard_msgs__msg__OperationalMode__Sequence * array)
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
      ur_dashboard_msgs__msg__OperationalMode__fini(&array->data[i]);
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

ur_dashboard_msgs__msg__OperationalMode__Sequence *
ur_dashboard_msgs__msg__OperationalMode__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__msg__OperationalMode__Sequence * array = (ur_dashboard_msgs__msg__OperationalMode__Sequence *)allocator.allocate(sizeof(ur_dashboard_msgs__msg__OperationalMode__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = ur_dashboard_msgs__msg__OperationalMode__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
ur_dashboard_msgs__msg__OperationalMode__Sequence__destroy(ur_dashboard_msgs__msg__OperationalMode__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    ur_dashboard_msgs__msg__OperationalMode__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
ur_dashboard_msgs__msg__OperationalMode__Sequence__are_equal(const ur_dashboard_msgs__msg__OperationalMode__Sequence * lhs, const ur_dashboard_msgs__msg__OperationalMode__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!ur_dashboard_msgs__msg__OperationalMode__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
ur_dashboard_msgs__msg__OperationalMode__Sequence__copy(
  const ur_dashboard_msgs__msg__OperationalMode__Sequence * input,
  ur_dashboard_msgs__msg__OperationalMode__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(ur_dashboard_msgs__msg__OperationalMode);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    ur_dashboard_msgs__msg__OperationalMode * data =
      (ur_dashboard_msgs__msg__OperationalMode *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!ur_dashboard_msgs__msg__OperationalMode__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          ur_dashboard_msgs__msg__OperationalMode__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!ur_dashboard_msgs__msg__OperationalMode__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
