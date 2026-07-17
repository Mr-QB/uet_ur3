# generated from rosidl_generator_py/resource/_idl.py.em
# with input from ur_dashboard_msgs:srv/GenerateFlightReport.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_GenerateFlightReport_Request(type):
    """Metaclass of message 'GenerateFlightReport_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'CONTROLLER': 'controller',
        'SOFTWARE': 'software',
        'SYSTEM': 'system',
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
                'ur_dashboard_msgs.srv.GenerateFlightReport_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__generate_flight_report__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__generate_flight_report__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__generate_flight_report__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__generate_flight_report__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__generate_flight_report__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'CONTROLLER': cls.__constants['CONTROLLER'],
            'SOFTWARE': cls.__constants['SOFTWARE'],
            'SYSTEM': cls.__constants['SYSTEM'],
            'REPORT_TYPE__DEFAULT': 'SYSTEM',
        }

    @property
    def CONTROLLER(self):
        """Message constant 'CONTROLLER'."""
        return Metaclass_GenerateFlightReport_Request.__constants['CONTROLLER']

    @property
    def SOFTWARE(self):
        """Message constant 'SOFTWARE'."""
        return Metaclass_GenerateFlightReport_Request.__constants['SOFTWARE']

    @property
    def SYSTEM(self):
        """Message constant 'SYSTEM'."""
        return Metaclass_GenerateFlightReport_Request.__constants['SYSTEM']

    @property
    def REPORT_TYPE__DEFAULT(cls):
        """Return default value for message field 'report_type'."""
        return 'SYSTEM'


class GenerateFlightReport_Request(metaclass=Metaclass_GenerateFlightReport_Request):
    """
    Message class 'GenerateFlightReport_Request'.

    Constants:
      CONTROLLER
      SOFTWARE
      SYSTEM
    """

    __slots__ = [
        '_report_type',
    ]

    _fields_and_field_types = {
        'report_type': 'string',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.report_type = kwargs.get(
            'report_type', GenerateFlightReport_Request.REPORT_TYPE__DEFAULT)

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
        if self.report_type != other.report_type:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def report_type(self):
        """Message field 'report_type'."""
        return self._report_type

    @report_type.setter
    def report_type(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'report_type' field must be of type 'str'"
        self._report_type = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_GenerateFlightReport_Response(type):
    """Metaclass of message 'GenerateFlightReport_Response'."""

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
                'ur_dashboard_msgs.srv.GenerateFlightReport_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__generate_flight_report__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__generate_flight_report__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__generate_flight_report__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__generate_flight_report__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__generate_flight_report__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class GenerateFlightReport_Response(metaclass=Metaclass_GenerateFlightReport_Response):
    """Message class 'GenerateFlightReport_Response'."""

    __slots__ = [
        '_success',
        '_answer',
        '_report_id',
    ]

    _fields_and_field_types = {
        'success': 'boolean',
        'answer': 'string',
        'report_id': 'string',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.success = kwargs.get('success', bool())
        self.answer = kwargs.get('answer', str())
        self.report_id = kwargs.get('report_id', str())

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
        if self.success != other.success:
            return False
        if self.answer != other.answer:
            return False
        if self.report_id != other.report_id:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

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
    def report_id(self):
        """Message field 'report_id'."""
        return self._report_id

    @report_id.setter
    def report_id(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'report_id' field must be of type 'str'"
        self._report_id = value


class Metaclass_GenerateFlightReport(type):
    """Metaclass of service 'GenerateFlightReport'."""

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
                'ur_dashboard_msgs.srv.GenerateFlightReport')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__generate_flight_report

            from ur_dashboard_msgs.srv import _generate_flight_report
            if _generate_flight_report.Metaclass_GenerateFlightReport_Request._TYPE_SUPPORT is None:
                _generate_flight_report.Metaclass_GenerateFlightReport_Request.__import_type_support__()
            if _generate_flight_report.Metaclass_GenerateFlightReport_Response._TYPE_SUPPORT is None:
                _generate_flight_report.Metaclass_GenerateFlightReport_Response.__import_type_support__()


class GenerateFlightReport(metaclass=Metaclass_GenerateFlightReport):
    from ur_dashboard_msgs.srv._generate_flight_report import GenerateFlightReport_Request as Request
    from ur_dashboard_msgs.srv._generate_flight_report import GenerateFlightReport_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
