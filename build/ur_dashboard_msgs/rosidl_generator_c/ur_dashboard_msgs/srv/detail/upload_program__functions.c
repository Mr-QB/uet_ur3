// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from ur_dashboard_msgs:srv/UploadProgram.idl
// generated code does not contain a copyright notice
#include "ur_dashboard_msgs/srv/detail/upload_program__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"

// Include directives for member types
// Member `file_path`
#include "rosidl_runtime_c/string_functions.h"

bool
ur_dashboard_msgs__srv__UploadProgram_Request__init(ur_dashboard_msgs__srv__UploadProgram_Request * msg)
{
  if (!msg) {
    return false;
  }
  // file_path
  if (!rosidl_runtime_c__String__init(&msg->file_path)) {
    ur_dashboard_msgs__srv__UploadProgram_Request__fini(msg);
    return false;
  }
  return true;
}

void
ur_dashboard_msgs__srv__UploadProgram_Request__fini(ur_dashboard_msgs__srv__UploadProgram_Request * msg)
{
  if (!msg) {
    return;
  }
  // file_path
  rosidl_runtime_c__String__fini(&msg->file_path);
}

bool
ur_dashboard_msgs__srv__UploadProgram_Request__are_equal(const ur_dashboard_msgs__srv__UploadProgram_Request * lhs, const ur_dashboard_msgs__srv__UploadProgram_Request * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // file_path
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->file_path), &(rhs->file_path)))
  {
    return false;
  }
  return true;
}

bool
ur_dashboard_msgs__srv__UploadProgram_Request__copy(
  const ur_dashboard_msgs__srv__UploadProgram_Request * input,
  ur_dashboard_msgs__srv__UploadProgram_Request * output)
{
  if (!input || !output) {
    return false;
  }
  // file_path
  if (!rosidl_runtime_c__String__copy(
      &(input->file_path), &(output->file_path)))
  {
    return false;
  }
  return true;
}

ur_dashboard_msgs__srv__UploadProgram_Request *
ur_dashboard_msgs__srv__UploadProgram_Request__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__srv__UploadProgram_Request * msg = (ur_dashboard_msgs__srv__UploadProgram_Request *)allocator.allocate(sizeof(ur_dashboard_msgs__srv__UploadProgram_Request), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(ur_dashboard_msgs__srv__UploadProgram_Request));
  bool success = ur_dashboard_msgs__srv__UploadProgram_Request__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
ur_dashboard_msgs__srv__UploadProgram_Request__destroy(ur_dashboard_msgs__srv__UploadProgram_Request * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    ur_dashboard_msgs__srv__UploadProgram_Request__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__init(ur_dashboard_msgs__srv__UploadProgram_Request__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__srv__UploadProgram_Request * data = NULL;

  if (size) {
    data = (ur_dashboard_msgs__srv__UploadProgram_Request *)allocator.zero_allocate(size, sizeof(ur_dashboard_msgs__srv__UploadProgram_Request), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = ur_dashboard_msgs__srv__UploadProgram_Request__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        ur_dashboard_msgs__srv__UploadProgram_Request__fini(&data[i - 1]);
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
ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__fini(ur_dashboard_msgs__srv__UploadProgram_Request__Sequence * array)
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
      ur_dashboard_msgs__srv__UploadProgram_Request__fini(&array->data[i]);
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

ur_dashboard_msgs__srv__UploadProgram_Request__Sequence *
ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__srv__UploadProgram_Request__Sequence * array = (ur_dashboard_msgs__srv__UploadProgram_Request__Sequence *)allocator.allocate(sizeof(ur_dashboard_msgs__srv__UploadProgram_Request__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__destroy(ur_dashboard_msgs__srv__UploadProgram_Request__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__are_equal(const ur_dashboard_msgs__srv__UploadProgram_Request__Sequence * lhs, const ur_dashboard_msgs__srv__UploadProgram_Request__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!ur_dashboard_msgs__srv__UploadProgram_Request__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__copy(
  const ur_dashboard_msgs__srv__UploadProgram_Request__Sequence * input,
  ur_dashboard_msgs__srv__UploadProgram_Request__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(ur_dashboard_msgs__srv__UploadProgram_Request);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    ur_dashboard_msgs__srv__UploadProgram_Request * data =
      (ur_dashboard_msgs__srv__UploadProgram_Request *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!ur_dashboard_msgs__srv__UploadProgram_Request__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          ur_dashboard_msgs__srv__UploadProgram_Request__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!ur_dashboard_msgs__srv__UploadProgram_Request__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


// Include directives for member types
// Member `answer`
// Member `program_name`
// already included above
// #include "rosidl_runtime_c/string_functions.h"

bool
ur_dashboard_msgs__srv__UploadProgram_Response__init(ur_dashboard_msgs__srv__UploadProgram_Response * msg)
{
  if (!msg) {
    return false;
  }
  // answer
  if (!rosidl_runtime_c__String__init(&msg->answer)) {
    ur_dashboard_msgs__srv__UploadProgram_Response__fini(msg);
    return false;
  }
  // program_name
  if (!rosidl_runtime_c__String__init(&msg->program_name)) {
    ur_dashboard_msgs__srv__UploadProgram_Response__fini(msg);
    return false;
  }
  // success
  return true;
}

void
ur_dashboard_msgs__srv__UploadProgram_Response__fini(ur_dashboard_msgs__srv__UploadProgram_Response * msg)
{
  if (!msg) {
    return;
  }
  // answer
  rosidl_runtime_c__String__fini(&msg->answer);
  // program_name
  rosidl_runtime_c__String__fini(&msg->program_name);
  // success
}

bool
ur_dashboard_msgs__srv__UploadProgram_Response__are_equal(const ur_dashboard_msgs__srv__UploadProgram_Response * lhs, const ur_dashboard_msgs__srv__UploadProgram_Response * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // answer
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->answer), &(rhs->answer)))
  {
    return false;
  }
  // program_name
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->program_name), &(rhs->program_name)))
  {
    return false;
  }
  // success
  if (lhs->success != rhs->success) {
    return false;
  }
  return true;
}

bool
ur_dashboard_msgs__srv__UploadProgram_Response__copy(
  const ur_dashboard_msgs__srv__UploadProgram_Response * input,
  ur_dashboard_msgs__srv__UploadProgram_Response * output)
{
  if (!input || !output) {
    return false;
  }
  // answer
  if (!rosidl_runtime_c__String__copy(
      &(input->answer), &(output->answer)))
  {
    return false;
  }
  // program_name
  if (!rosidl_runtime_c__String__copy(
      &(input->program_name), &(output->program_name)))
  {
    return false;
  }
  // success
  output->success = input->success;
  return true;
}

ur_dashboard_msgs__srv__UploadProgram_Response *
ur_dashboard_msgs__srv__UploadProgram_Response__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__srv__UploadProgram_Response * msg = (ur_dashboard_msgs__srv__UploadProgram_Response *)allocator.allocate(sizeof(ur_dashboard_msgs__srv__UploadProgram_Response), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(ur_dashboard_msgs__srv__UploadProgram_Response));
  bool success = ur_dashboard_msgs__srv__UploadProgram_Response__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
ur_dashboard_msgs__srv__UploadProgram_Response__destroy(ur_dashboard_msgs__srv__UploadProgram_Response * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    ur_dashboard_msgs__srv__UploadProgram_Response__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__init(ur_dashboard_msgs__srv__UploadProgram_Response__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__srv__UploadProgram_Response * data = NULL;

  if (size) {
    data = (ur_dashboard_msgs__srv__UploadProgram_Response *)allocator.zero_allocate(size, sizeof(ur_dashboard_msgs__srv__UploadProgram_Response), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = ur_dashboard_msgs__srv__UploadProgram_Response__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        ur_dashboard_msgs__srv__UploadProgram_Response__fini(&data[i - 1]);
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
ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__fini(ur_dashboard_msgs__srv__UploadProgram_Response__Sequence * array)
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
      ur_dashboard_msgs__srv__UploadProgram_Response__fini(&array->data[i]);
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

ur_dashboard_msgs__srv__UploadProgram_Response__Sequence *
ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ur_dashboard_msgs__srv__UploadProgram_Response__Sequence * array = (ur_dashboard_msgs__srv__UploadProgram_Response__Sequence *)allocator.allocate(sizeof(ur_dashboard_msgs__srv__UploadProgram_Response__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__destroy(ur_dashboard_msgs__srv__UploadProgram_Response__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__are_equal(const ur_dashboard_msgs__srv__UploadProgram_Response__Sequence * lhs, const ur_dashboard_msgs__srv__UploadProgram_Response__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!ur_dashboard_msgs__srv__UploadProgram_Response__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__copy(
  const ur_dashboard_msgs__srv__UploadProgram_Response__Sequence * input,
  ur_dashboard_msgs__srv__UploadProgram_Response__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(ur_dashboard_msgs__srv__UploadProgram_Response);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    ur_dashboard_msgs__srv__UploadProgram_Response * data =
      (ur_dashboard_msgs__srv__UploadProgram_Response *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!ur_dashboard_msgs__srv__UploadProgram_Response__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          ur_dashboard_msgs__srv__UploadProgram_Response__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!ur_dashboard_msgs__srv__UploadProgram_Response__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
