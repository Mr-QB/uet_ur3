# generated from rosidl_generator_py/resource/_idl.py.em
# with input from ur_dashboard_msgs:srv/GetPolyScopeVersion.idl
# generated code does not contain a copyright notice


# Import statements for member types

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_GetPolyScopeVersion_Request(type):
    """Metaclass of message 'GetPolyScopeVersion_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('ur_dashboard_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur_dashboard_msgs.srv.GetPolyScopeVersion_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__get_poly_scope_version__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__get_poly_scope_version__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__get_poly_scope_version__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__get_poly_scope_version__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__get_poly_scope_version__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class GetPolyScopeVersion_Request(metaclass=Metaclass_GetPolyScopeVersion_Request):
    """Message class 'GetPolyScopeVersion_Request'."""

    __slots__ = [
    ]

    _fields_and_field_types = {
    }

    SLOT_TYPES = (
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)


# Import statements for member types

import builtins  # noqa: E402, I100

# already imported above
# import rosidl_parser.definition


class Metaclass_GetPolyScopeVersion_Response(type):
    """Metaclass of message 'GetPolyScopeVersion_Response'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('ur_dashboard_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur_dashboard_msgs.srv.GetPolyScopeVersion_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__get_poly_scope_version__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__get_poly_scope_version__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__get_poly_scope_version__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__get_poly_scope_version__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__get_poly_scope_version__response

            from ur_dashboard_msgs.msg import VersionInformation
            if VersionInformation.__class__._TYPE_SUPPORT is None:
                VersionInformation.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class GetPolyScopeVersion_Response(metaclass=Metaclass_GetPolyScopeVersion_Response):
    """Message class 'GetPolyScopeVersion_Response'."""

    __slots__ = [
        '_answer',
        '_success',
        '_version',
    ]

    _fields_and_field_types = {
        'answer': 'string',
        'success': 'boolean',
        'version': 'ur_dashboard_msgs/VersionInformation',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['ur_dashboard_msgs', 'msg'], 'VersionInformation'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.answer = kwargs.get('answer', str())
        self.success = kwargs.get('success', bool())
        from ur_dashboard_msgs.msg import VersionInformation
        self.version = kwargs.get('version', VersionInformation())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.answer != other.answer:
            return False
        if self.success != other.success:
            return False
        if self.version != other.version:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def answer(self):
        """Message field 'answer'."""
        return self._answer

    @answer.setter
    def answer(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'answer' field must be of type 'str'"
        self._answer = value

    @builtins.property
    def success(self):
        """Message field 'success'."""
        return self._success

    @success.setter
    def success(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'success' field must be of type 'bool'"
        self._success = value

    @builtins.property
    def version(self):
        """Message field 'version'."""
        return self._version

    @version.setter
    def version(self, value):
        if __debug__:
            from ur_dashboard_msgs.msg import VersionInformation
            assert \
                isinstance(value, VersionInformation), \
                "The 'version' field must be a sub message of type 'VersionInformation'"
        self._version = value


class Metaclass_GetPolyScopeVersion(type):
    """Metaclass of service 'GetPolyScopeVersion'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('ur_dashboard_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur_dashboard_msgs.srv.GetPolyScopeVersion')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__get_poly_scope_version

            from ur_dashboard_msgs.srv import _get_poly_scope_version
            if _get_poly_scope_version.Metaclass_GetPolyScopeVersion_Request._TYPE_SUPPORT is None:
                _get_poly_scope_version.Metaclass_GetPolyScopeVersion_Request.__import_type_support__()
            if _get_poly_scope_version.Metaclass_GetPolyScopeVersion_Response._TYPE_SUPPORT is None:
                _get_poly_scope_version.Metaclass_GetPolyScopeVersion_Response.__import_type_support__()


class GetPolyScopeVersion(metaclass=Metaclass_GetPolyScopeVersion):
    from ur_dashboard_msgs.srv._get_poly_scope_version import GetPolyScopeVersion_Request as Request
    from ur_dashboard_msgs.srv._get_poly_scope_version import GetPolyScopeVersion_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
