# generated from rosidl_generator_py/resource/_idl.py.em
# with input from ur_dashboard_msgs:msg/ProgramInformation.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_ProgramInformation(type):
    """Metaclass of message 'ProgramInformation'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'PROGRAM_STATE_DRAFT': 'DRAFT',
        'PROGRAM_STATE_FINAL': 'FINAL',
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
                'ur_dashboard_msgs.msg.ProgramInformation')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__program_information
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__program_information
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__program_information
            cls._TYPE_SUPPORT = module.type_support_msg__msg__program_information
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__program_information

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'PROGRAM_STATE_DRAFT': cls.__constants['PROGRAM_STATE_DRAFT'],
            'PROGRAM_STATE_FINAL': cls.__constants['PROGRAM_STATE_FINAL'],
        }

    @property
    def PROGRAM_STATE_DRAFT(self):
        """Message constant 'PROGRAM_STATE_DRAFT'."""
        return Metaclass_ProgramInformation.__constants['PROGRAM_STATE_DRAFT']

    @property
    def PROGRAM_STATE_FINAL(self):
        """Message constant 'PROGRAM_STATE_FINAL'."""
        return Metaclass_ProgramInformation.__constants['PROGRAM_STATE_FINAL']


class ProgramInformation(metaclass=Metaclass_ProgramInformation):
    """
    Message class 'ProgramInformation'.

    Constants:
      PROGRAM_STATE_DRAFT
      PROGRAM_STATE_FINAL
    """

    __slots__ = [
        '_name',
        '_description',
        '_created_date',
        '_last_saved_date',
        '_last_modified_date',
        '_program_state',
    ]

    _fields_and_field_types = {
        'name': 'string',
        'description': 'string',
        'created_date': 'uint64',
        'last_saved_date': 'uint64',
        'last_modified_date': 'uint64',
        'program_state': 'string',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('uint64'),  # noqa: E501
        rosidl_parser.definition.BasicType('uint64'),  # noqa: E501
        rosidl_parser.definition.BasicType('uint64'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.name = kwargs.get('name', str())
        self.description = kwargs.get('description', str())
        self.created_date = kwargs.get('created_date', int())
        self.last_saved_date = kwargs.get('last_saved_date', int())
        self.last_modified_date = kwargs.get('last_modified_date', int())
        self.program_state = kwargs.get('program_state', str())

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
        if self.name != other.name:
            return False
        if self.description != other.description:
            return False
        if self.created_date != other.created_date:
            return False
        if self.last_saved_date != other.last_saved_date:
            return False
        if self.last_modified_date != other.last_modified_date:
            return False
        if self.program_state != other.program_state:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def name(self):
        """Message field 'name'."""
        return self._name

    @name.setter
    def name(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'name' field must be of type 'str'"
        self._name = value

    @builtins.property
    def description(self):
        """Message field 'description'."""
        return self._description

    @description.setter
    def description(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'description' field must be of type 'str'"
        self._description = value

    @builtins.property
    def created_date(self):
        """Message field 'created_date'."""
        return self._created_date

    @created_date.setter
    def created_date(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'created_date' field must be of type 'int'"
            assert value >= 0 and value < 18446744073709551616, \
                "The 'created_date' field must be an unsigned integer in [0, 18446744073709551615]"
        self._created_date = value

    @builtins.property
    def last_saved_date(self):
        """Message field 'last_saved_date'."""
        return self._last_saved_date

    @last_saved_date.setter
    def last_saved_date(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'last_saved_date' field must be of type 'int'"
            assert value >= 0 and value < 18446744073709551616, \
                "The 'last_saved_date' field must be an unsigned integer in [0, 18446744073709551615]"
        self._last_saved_date = value

    @builtins.property
    def last_modified_date(self):
        """Message field 'last_modified_date'."""
        return self._last_modified_date

    @last_modified_date.setter
    def last_modified_date(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'last_modified_date' field must be of type 'int'"
            assert value >= 0 and value < 18446744073709551616, \
                "The 'last_modified_date' field must be an unsigned integer in [0, 18446744073709551615]"
        self._last_modified_date = value

    @builtins.property
    def program_state(self):
        """Message field 'program_state'."""
        return self._program_state

    @program_state.setter
    def program_state(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'program_state' field must be of type 'str'"
        self._program_state = value
