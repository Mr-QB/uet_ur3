// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from ur_dashboard_msgs:msg/VersionInformation.idl
// generated code does not contain a copyright notice
#include "ur_dashboard_msgs/msg/detail/version_information__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


bool
ur_dashboard_msgs__msg__VersionInformation__init(ur_dashboard_msgs__msg__VersionInformation * msg)
{
  if (!msg) {
    return false;
  }
  // major
  // minor
  // bugfix
  // build
  return true;
}

void
ur_dashboard_msgs__msg__VersionInformation__fini(ur_dashboard_msgs__msg__VersionInformation * msg)
{
  if (!msg) {
    return;
  }
  // major
  // minor
  // bugfix
  // build
}

bool
ur_dashboard_msgs__msg__VersionInformation__are_equal(const ur_dashboard_msgs__msg__VersionInformation * lhs, const ur_dashboard_msgs__msg__VersionInformation * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // major
  if (lhs->major != rhs->major) {
    return false;
  }
  // minor
  if (lhs->minor != rhs->minor) {
    return false;
  }
  // bugfix
  if (lhs->bugfix != rhs->bugfix) {
    return false;
  }
  // build
  if (lhs->build != rhs->build) {
    return false;
  }
  return true;
}

bool
ur_dashboard_msgs__msg__VersionInformation__copy(
  const ur_dashboard_msgs__msg__VersionInformation * input,
  ur_dashboard_msgs__msg__VersionInformation * output)
{
  if (!input || !output) {
    return false;
  }
  // major
  output->major = input->major;
  // minor
  output->minor = input->minor;
  // bugfix
  output->bugfix = input->bugfix;
  // build
  output->build = input->build;
  return true;
}

ur_dashboard_msgs__msg__VersionInformation *
ur_dashboard_msgs__msg__VersionInformation__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__msg__VersionInformation * msg = (ur_dashboard_msgs__msg__VersionInformation *)allocator.allocate(sizeof(ur_dashboard_msgs__msg__VersionInformation), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(ur_dashboard_msgs__msg__VersionInformation));
  bool success = ur_dashboard_msgs__msg__VersionInformation__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
ur_dashboard_msgs__msg__VersionInformation__destroy(ur_dashboard_msgs__msg__VersionInformation * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    ur_dashboard_msgs__msg__VersionInformation__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
ur_dashboard_msgs__msg__VersionInformation__Sequence__init(ur_dashboard_msgs__msg__VersionInformation__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__msg__VersionInformation * data = NULL;

  if (size) {
    data = (ur_dashboard_msgs__msg__VersionInformation *)allocator.zero_allocate(size, sizeof(ur_dashboard_msgs__msg__VersionInformation), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = ur_dashboard_msgs__msg__VersionInformation__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        ur_dashboard_msgs__msg__VersionInformation__fini(&data[i - 1]);
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
ur_dashboard_msgs__msg__VersionInformation__Sequence__fini(ur_dashboard_msgs__msg__VersionInformation__Sequence * array)
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
      ur_dashboard_msgs__msg__VersionInformation__fini(&array->data[i]);
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

ur_dashboard_msgs__msg__VersionInformation__Sequence *
ur_dashboard_msgs__msg__VersionInformation__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__msg__VersionInformation__Sequence * array = (ur_dashboard_msgs__msg__VersionInformation__Sequence *)allocator.allocate(sizeof(ur_dashboard_msgs__msg__VersionInformation__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = ur_dashboard_msgs__msg__VersionInformation__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
ur_dashboard_msgs__msg__VersionInformation__Sequence__destroy(ur_dashboard_msgs__msg__VersionInformation__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    ur_dashboard_msgs__msg__VersionInformation__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
ur_dashboard_msgs__msg__VersionInformation__Sequence__are_equal(const ur_dashboard_msgs__msg__VersionInformation__Sequence * lhs, const ur_dashboard_msgs__msg__VersionInformation__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!ur_dashboard_msgs__msg__VersionInformation__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
ur_dashboard_msgs__msg__VersionInformation__Sequence__copy(
  const ur_dashboard_msgs__msg__VersionInformation__Sequence * input,
  ur_dashboard_msgs__msg__VersionInformation__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(ur_dashboard_msgs__msg__VersionInformation);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    ur_dashboard_msgs__msg__VersionInformation * data =
      (ur_dashboard_msgs__msg__VersionInformation *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!ur_dashboard_msgs__msg__VersionInformation__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          ur_dashboard_msgs__msg__VersionInformation__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!ur_dashboard_msgs__msg__VersionInformation__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
