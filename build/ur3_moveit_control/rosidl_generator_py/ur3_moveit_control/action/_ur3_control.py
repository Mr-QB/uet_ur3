# generated from rosidl_generator_py/resource/_idl.py.em
# with input from ur3_moveit_control:action/UR3Control.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_UR3Control_Goal(type):
    """Metaclass of message 'UR3Control_Goal'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'MOVE_HOME': 0,
        'MOVE_JOINT': 1,
        'MOVE_POSE': 2,
        'ATTACH_AND_LIFT': 3,
        'DETACH_OBJECT': 4,
        'MOVE_CARTESIAN': 5,
        'MOVE_TO_XY': 6,
        'PREPARE_NEXT_TRIAL': 7,
        'MOVE_CARTESIAN_STRICT': 8,
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('ur3_moveit_control')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur3_moveit_control.action.UR3Control_Goal')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ur3_control__goal
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ur3_control__goal
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ur3_control__goal
            cls._TYPE_SUPPORT = module.type_support_msg__action__ur3_control__goal
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ur3_control__goal

            from geometry_msgs.msg import PoseStamped
            if PoseStamped.__class__._TYPE_SUPPORT is None:
                PoseStamped.__class__.__import_type_support__()

            from sensor_msgs.msg import JointState
            if JointState.__class__._TYPE_SUPPORT is None:
                JointState.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'MOVE_HOME': cls.__constants['MOVE_HOME'],
            'MOVE_JOINT': cls.__constants['MOVE_JOINT'],
            'MOVE_POSE': cls.__constants['MOVE_POSE'],
            'ATTACH_AND_LIFT': cls.__constants['ATTACH_AND_LIFT'],
            'DETACH_OBJECT': cls.__constants['DETACH_OBJECT'],
            'MOVE_CARTESIAN': cls.__constants['MOVE_CARTESIAN'],
            'MOVE_TO_XY': cls.__constants['MOVE_TO_XY'],
            'PREPARE_NEXT_TRIAL': cls.__constants['PREPARE_NEXT_TRIAL'],
            'MOVE_CARTESIAN_STRICT': cls.__constants['MOVE_CARTESIAN_STRICT'],
        }

    @property
    def MOVE_HOME(self):
        """Message constant 'MOVE_HOME'."""
        return Metaclass_UR3Control_Goal.__constants['MOVE_HOME']

    @property
    def MOVE_JOINT(self):
        """Message constant 'MOVE_JOINT'."""
        return Metaclass_UR3Control_Goal.__constants['MOVE_JOINT']

    @property
    def MOVE_POSE(self):
        """Message constant 'MOVE_POSE'."""
        return Metaclass_UR3Control_Goal.__constants['MOVE_POSE']

    @property
    def ATTACH_AND_LIFT(self):
        """Message constant 'ATTACH_AND_LIFT'."""
        return Metaclass_UR3Control_Goal.__constants['ATTACH_AND_LIFT']

    @property
    def DETACH_OBJECT(self):
        """Message constant 'DETACH_OBJECT'."""
        return Metaclass_UR3Control_Goal.__constants['DETACH_OBJECT']

    @property
    def MOVE_CARTESIAN(self):
        """Message constant 'MOVE_CARTESIAN'."""
        return Metaclass_UR3Control_Goal.__constants['MOVE_CARTESIAN']

    @property
    def MOVE_TO_XY(self):
        """Message constant 'MOVE_TO_XY'."""
        return Metaclass_UR3Control_Goal.__constants['MOVE_TO_XY']

    @property
    def PREPARE_NEXT_TRIAL(self):
        """Message constant 'PREPARE_NEXT_TRIAL'."""
        return Metaclass_UR3Control_Goal.__constants['PREPARE_NEXT_TRIAL']

    @property
    def MOVE_CARTESIAN_STRICT(self):
        """Message constant 'MOVE_CARTESIAN_STRICT'."""
        return Metaclass_UR3Control_Goal.__constants['MOVE_CARTESIAN_STRICT']


class UR3Control_Goal(metaclass=Metaclass_UR3Control_Goal):
    """
    Message class 'UR3Control_Goal'.

    Constants:
      MOVE_HOME
      MOVE_JOINT
      MOVE_POSE
      ATTACH_AND_LIFT
      DETACH_OBJECT
      MOVE_CARTESIAN
      MOVE_TO_XY
      PREPARE_NEXT_TRIAL
      MOVE_CARTESIAN_STRICT
    """

    __slots__ = [
        '_command_type',
        '_joint_goal',
        '_pose_goal',
        '_cartesian_x_offset',
        '_cartesian_y_offset',
        '_cartesian_z_offset',
        '_target_x',
        '_target_y',
        '_object_x',
        '_object_y',
        '_object_z',
    ]

    _fields_and_field_types = {
        'command_type': 'uint8',
        'joint_goal': 'sensor_msgs/JointState',
        'pose_goal': 'geometry_msgs/PoseStamped',
        'cartesian_x_offset': 'double',
        'cartesian_y_offset': 'double',
        'cartesian_z_offset': 'double',
        'target_x': 'double',
        'target_y': 'double',
        'object_x': 'double',
        'object_y': 'double',
        'object_z': 'double',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['sensor_msgs', 'msg'], 'JointState'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['geometry_msgs', 'msg'], 'PoseStamped'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.command_type = kwargs.get('command_type', int())
        from sensor_msgs.msg import JointState
        self.joint_goal = kwargs.get('joint_goal', JointState())
        from geometry_msgs.msg import PoseStamped
        self.pose_goal = kwargs.get('pose_goal', PoseStamped())
        self.cartesian_x_offset = kwargs.get('cartesian_x_offset', float())
        self.cartesian_y_offset = kwargs.get('cartesian_y_offset', float())
        self.cartesian_z_offset = kwargs.get('cartesian_z_offset', float())
        self.target_x = kwargs.get('target_x', float())
        self.target_y = kwargs.get('target_y', float())
        self.object_x = kwargs.get('object_x', float())
        self.object_y = kwargs.get('object_y', float())
        self.object_z = kwargs.get('object_z', float())

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
        if self.command_type != other.command_type:
            return False
        if self.joint_goal != other.joint_goal:
            return False
        if self.pose_goal != other.pose_goal:
            return False
        if self.cartesian_x_offset != other.cartesian_x_offset:
            return False
        if self.cartesian_y_offset != other.cartesian_y_offset:
            return False
        if self.cartesian_z_offset != other.cartesian_z_offset:
            return False
        if self.target_x != other.target_x:
            return False
        if self.target_y != other.target_y:
            return False
        if self.object_x != other.object_x:
            return False
        if self.object_y != other.object_y:
            return False
        if self.object_z != other.object_z:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def command_type(self):
        """Message field 'command_type'."""
        return self._command_type

    @command_type.setter
    def command_type(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'command_type' field must be of type 'int'"
            assert value >= 0 and value < 256, \
                "The 'command_type' field must be an unsigned integer in [0, 255]"
        self._command_type = value

    @builtins.property
    def joint_goal(self):
        """Message field 'joint_goal'."""
        return self._joint_goal

    @joint_goal.setter
    def joint_goal(self, value):
        if __debug__:
            from sensor_msgs.msg import JointState
            assert \
                isinstance(value, JointState), \
                "The 'joint_goal' field must be a sub message of type 'JointState'"
        self._joint_goal = value

    @builtins.property
    def pose_goal(self):
        """Message field 'pose_goal'."""
        return self._pose_goal

    @pose_goal.setter
    def pose_goal(self, value):
        if __debug__:
            from geometry_msgs.msg import PoseStamped
            assert \
                isinstance(value, PoseStamped), \
                "The 'pose_goal' field must be a sub message of type 'PoseStamped'"
        self._pose_goal = value

    @builtins.property
    def cartesian_x_offset(self):
        """Message field 'cartesian_x_offset'."""
        return self._cartesian_x_offset

    @cartesian_x_offset.setter
    def cartesian_x_offset(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'cartesian_x_offset' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'cartesian_x_offset' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._cartesian_x_offset = value

    @builtins.property
    def cartesian_y_offset(self):
        """Message field 'cartesian_y_offset'."""
        return self._cartesian_y_offset

    @cartesian_y_offset.setter
    def cartesian_y_offset(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'cartesian_y_offset' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'cartesian_y_offset' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._cartesian_y_offset = value

    @builtins.property
    def cartesian_z_offset(self):
        """Message field 'cartesian_z_offset'."""
        return self._cartesian_z_offset

    @cartesian_z_offset.setter
    def cartesian_z_offset(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'cartesian_z_offset' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'cartesian_z_offset' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._cartesian_z_offset = value

    @builtins.property
    def target_x(self):
        """Message field 'target_x'."""
        return self._target_x

    @target_x.setter
    def target_x(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'target_x' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'target_x' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._target_x = value

    @builtins.property
    def target_y(self):
        """Message field 'target_y'."""
        return self._target_y

    @target_y.setter
    def target_y(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'target_y' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'target_y' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._target_y = value

    @builtins.property
    def object_x(self):
        """Message field 'object_x'."""
        return self._object_x

    @object_x.setter
    def object_x(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'object_x' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'object_x' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._object_x = value

    @builtins.property
    def object_y(self):
        """Message field 'object_y'."""
        return self._object_y

    @object_y.setter
    def object_y(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'object_y' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'object_y' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._object_y = value

    @builtins.property
    def object_z(self):
        """Message field 'object_z'."""
        return self._object_z

    @object_z.setter
    def object_z(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'object_z' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'object_z' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._object_z = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_UR3Control_Result(type):
    """Metaclass of message 'UR3Control_Result'."""

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
            module = import_type_support('ur3_moveit_control')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur3_moveit_control.action.UR3Control_Result')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ur3_control__result
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ur3_control__result
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ur3_control__result
            cls._TYPE_SUPPORT = module.type_support_msg__action__ur3_control__result
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ur3_control__result

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class UR3Control_Result(metaclass=Metaclass_UR3Control_Result):
    """Message class 'UR3Control_Result'."""

    __slots__ = [
        '_success',
        '_message',
    ]

    _fields_and_field_types = {
        'success': 'boolean',
        'message': 'string',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.success = kwargs.get('success', bool())
        self.message = kwargs.get('message', str())

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
        if self.message != other.message:
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
    def message(self):
        """Message field 'message'."""
        return self._message

    @message.setter
    def message(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'message' field must be of type 'str'"
        self._message = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_UR3Control_Feedback(type):
    """Metaclass of message 'UR3Control_Feedback'."""

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
            module = import_type_support('ur3_moveit_control')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur3_moveit_control.action.UR3Control_Feedback')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ur3_control__feedback
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ur3_control__feedback
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ur3_control__feedback
            cls._TYPE_SUPPORT = module.type_support_msg__action__ur3_control__feedback
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ur3_control__feedback

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class UR3Control_Feedback(metaclass=Metaclass_UR3Control_Feedback):
    """Message class 'UR3Control_Feedback'."""

    __slots__ = [
        '_state',
    ]

    _fields_and_field_types = {
        'state': 'string',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.state = kwargs.get('state', str())

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
        if self.state != other.state:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def state(self):
        """Message field 'state'."""
        return self._state

    @state.setter
    def state(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'state' field must be of type 'str'"
        self._state = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_UR3Control_SendGoal_Request(type):
    """Metaclass of message 'UR3Control_SendGoal_Request'."""

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
            module = import_type_support('ur3_moveit_control')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur3_moveit_control.action.UR3Control_SendGoal_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ur3_control__send_goal__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ur3_control__send_goal__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ur3_control__send_goal__request
            cls._TYPE_SUPPORT = module.type_support_msg__action__ur3_control__send_goal__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ur3_control__send_goal__request

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

            from ur3_moveit_control.action import UR3Control
            if UR3Control.Goal.__class__._TYPE_SUPPORT is None:
                UR3Control.Goal.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class UR3Control_SendGoal_Request(metaclass=Metaclass_UR3Control_SendGoal_Request):
    """Message class 'UR3Control_SendGoal_Request'."""

    __slots__ = [
        '_goal_id',
        '_goal',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
        'goal': 'ur3_moveit_control/UR3Control_Goal',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['ur3_moveit_control', 'action'], 'UR3Control_Goal'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())
        from ur3_moveit_control.action._ur3_control import UR3Control_Goal
        self.goal = kwargs.get('goal', UR3Control_Goal())

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
        if self.goal_id != other.goal_id:
            return False
        if self.goal != other.goal:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value

    @builtins.property
    def goal(self):
        """Message field 'goal'."""
        return self._goal

    @goal.setter
    def goal(self, value):
        if __debug__:
            from ur3_moveit_control.action._ur3_control import UR3Control_Goal
            assert \
                isinstance(value, UR3Control_Goal), \
                "The 'goal' field must be a sub message of type 'UR3Control_Goal'"
        self._goal = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_UR3Control_SendGoal_Response(type):
    """Metaclass of message 'UR3Control_SendGoal_Response'."""

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
            module = import_type_support('ur3_moveit_control')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur3_moveit_control.action.UR3Control_SendGoal_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ur3_control__send_goal__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ur3_control__send_goal__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ur3_control__send_goal__response
            cls._TYPE_SUPPORT = module.type_support_msg__action__ur3_control__send_goal__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ur3_control__send_goal__response

            from builtin_interfaces.msg import Time
            if Time.__class__._TYPE_SUPPORT is None:
                Time.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class UR3Control_SendGoal_Response(metaclass=Metaclass_UR3Control_SendGoal_Response):
    """Message class 'UR3Control_SendGoal_Response'."""

    __slots__ = [
        '_accepted',
        '_stamp',
    ]

    _fields_and_field_types = {
        'accepted': 'boolean',
        'stamp': 'builtin_interfaces/Time',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['builtin_interfaces', 'msg'], 'Time'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.accepted = kwargs.get('accepted', bool())
        from builtin_interfaces.msg import Time
        self.stamp = kwargs.get('stamp', Time())

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
        if self.accepted != other.accepted:
            return False
        if self.stamp != other.stamp:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def accepted(self):
        """Message field 'accepted'."""
        return self._accepted

    @accepted.setter
    def accepted(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'accepted' field must be of type 'bool'"
        self._accepted = value

    @builtins.property
    def stamp(self):
        """Message field 'stamp'."""
        return self._stamp

    @stamp.setter
    def stamp(self, value):
        if __debug__:
            from builtin_interfaces.msg import Time
            assert \
                isinstance(value, Time), \
                "The 'stamp' field must be a sub message of type 'Time'"
        self._stamp = value


class Metaclass_UR3Control_SendGoal(type):
    """Metaclass of service 'UR3Control_SendGoal'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('ur3_moveit_control')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur3_moveit_control.action.UR3Control_SendGoal')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__action__ur3_control__send_goal

            from ur3_moveit_control.action import _ur3_control
            if _ur3_control.Metaclass_UR3Control_SendGoal_Request._TYPE_SUPPORT is None:
                _ur3_control.Metaclass_UR3Control_SendGoal_Request.__import_type_support__()
            if _ur3_control.Metaclass_UR3Control_SendGoal_Response._TYPE_SUPPORT is None:
                _ur3_control.Metaclass_UR3Control_SendGoal_Response.__import_type_support__()


class UR3Control_SendGoal(metaclass=Metaclass_UR3Control_SendGoal):
    from ur3_moveit_control.action._ur3_control import UR3Control_SendGoal_Request as Request
    from ur3_moveit_control.action._ur3_control import UR3Control_SendGoal_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_UR3Control_GetResult_Request(type):
    """Metaclass of message 'UR3Control_GetResult_Request'."""

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
            module = import_type_support('ur3_moveit_control')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur3_moveit_control.action.UR3Control_GetResult_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ur3_control__get_result__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ur3_control__get_result__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ur3_control__get_result__request
            cls._TYPE_SUPPORT = module.type_support_msg__action__ur3_control__get_result__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ur3_control__get_result__request

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class UR3Control_GetResult_Request(metaclass=Metaclass_UR3Control_GetResult_Request):
    """Message class 'UR3Control_GetResult_Request'."""

    __slots__ = [
        '_goal_id',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())

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
        if self.goal_id != other.goal_id:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_UR3Control_GetResult_Response(type):
    """Metaclass of message 'UR3Control_GetResult_Response'."""

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
            module = import_type_support('ur3_moveit_control')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur3_moveit_control.action.UR3Control_GetResult_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ur3_control__get_result__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ur3_control__get_result__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ur3_control__get_result__response
            cls._TYPE_SUPPORT = module.type_support_msg__action__ur3_control__get_result__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ur3_control__get_result__response

            from ur3_moveit_control.action import UR3Control
            if UR3Control.Result.__class__._TYPE_SUPPORT is None:
                UR3Control.Result.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class UR3Control_GetResult_Response(metaclass=Metaclass_UR3Control_GetResult_Response):
    """Message class 'UR3Control_GetResult_Response'."""

    __slots__ = [
        '_status',
        '_result',
    ]

    _fields_and_field_types = {
        'status': 'int8',
        'result': 'ur3_moveit_control/UR3Control_Result',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int8'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['ur3_moveit_control', 'action'], 'UR3Control_Result'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.status = kwargs.get('status', int())
        from ur3_moveit_control.action._ur3_control import UR3Control_Result
        self.result = kwargs.get('result', UR3Control_Result())

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
        if self.status != other.status:
            return False
        if self.result != other.result:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def status(self):
        """Message field 'status'."""
        return self._status

    @status.setter
    def status(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'status' field must be of type 'int'"
            assert value >= -128 and value < 128, \
                "The 'status' field must be an integer in [-128, 127]"
        self._status = value

    @builtins.property
    def result(self):
        """Message field 'result'."""
        return self._result

    @result.setter
    def result(self, value):
        if __debug__:
            from ur3_moveit_control.action._ur3_control import UR3Control_Result
            assert \
                isinstance(value, UR3Control_Result), \
                "The 'result' field must be a sub message of type 'UR3Control_Result'"
        self._result = value


class Metaclass_UR3Control_GetResult(type):
    """Metaclass of service 'UR3Control_GetResult'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('ur3_moveit_control')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur3_moveit_control.action.UR3Control_GetResult')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__action__ur3_control__get_result

            from ur3_moveit_control.action import _ur3_control
            if _ur3_control.Metaclass_UR3Control_GetResult_Request._TYPE_SUPPORT is None:
                _ur3_control.Metaclass_UR3Control_GetResult_Request.__import_type_support__()
            if _ur3_control.Metaclass_UR3Control_GetResult_Response._TYPE_SUPPORT is None:
                _ur3_control.Metaclass_UR3Control_GetResult_Response.__import_type_support__()


class UR3Control_GetResult(metaclass=Metaclass_UR3Control_GetResult):
    from ur3_moveit_control.action._ur3_control import UR3Control_GetResult_Request as Request
    from ur3_moveit_control.action._ur3_control import UR3Control_GetResult_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_UR3Control_FeedbackMessage(type):
    """Metaclass of message 'UR3Control_FeedbackMessage'."""

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
            module = import_type_support('ur3_moveit_control')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur3_moveit_control.action.UR3Control_FeedbackMessage')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__ur3_control__feedback_message
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__ur3_control__feedback_message
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__ur3_control__feedback_message
            cls._TYPE_SUPPORT = module.type_support_msg__action__ur3_control__feedback_message
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__ur3_control__feedback_message

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

            from ur3_moveit_control.action import UR3Control
            if UR3Control.Feedback.__class__._TYPE_SUPPORT is None:
                UR3Control.Feedback.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class UR3Control_FeedbackMessage(metaclass=Metaclass_UR3Control_FeedbackMessage):
    """Message class 'UR3Control_FeedbackMessage'."""

    __slots__ = [
        '_goal_id',
        '_feedback',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
        'feedback': 'ur3_moveit_control/UR3Control_Feedback',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['ur3_moveit_control', 'action'], 'UR3Control_Feedback'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())
        from ur3_moveit_control.action._ur3_control import UR3Control_Feedback
        self.feedback = kwargs.get('feedback', UR3Control_Feedback())

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
        if self.goal_id != other.goal_id:
            return False
        if self.feedback != other.feedback:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value

    @builtins.property
    def feedback(self):
        """Message field 'feedback'."""
        return self._feedback

    @feedback.setter
    def feedback(self, value):
        if __debug__:
            from ur3_moveit_control.action._ur3_control import UR3Control_Feedback
            assert \
                isinstance(value, UR3Control_Feedback), \
                "The 'feedback' field must be a sub message of type 'UR3Control_Feedback'"
        self._feedback = value


class Metaclass_UR3Control(type):
    """Metaclass of action 'UR3Control'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('ur3_moveit_control')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ur3_moveit_control.action.UR3Control')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_action__action__ur3_control

            from action_msgs.msg import _goal_status_array
            if _goal_status_array.Metaclass_GoalStatusArray._TYPE_SUPPORT is None:
                _goal_status_array.Metaclass_GoalStatusArray.__import_type_support__()
            from action_msgs.srv import _cancel_goal
            if _cancel_goal.Metaclass_CancelGoal._TYPE_SUPPORT is None:
                _cancel_goal.Metaclass_CancelGoal.__import_type_support__()

            from ur3_moveit_control.action import _ur3_control
            if _ur3_control.Metaclass_UR3Control_SendGoal._TYPE_SUPPORT is None:
                _ur3_control.Metaclass_UR3Control_SendGoal.__import_type_support__()
            if _ur3_control.Metaclass_UR3Control_GetResult._TYPE_SUPPORT is None:
                _ur3_control.Metaclass_UR3Control_GetResult.__import_type_support__()
            if _ur3_control.Metaclass_UR3Control_FeedbackMessage._TYPE_SUPPORT is None:
                _ur3_control.Metaclass_UR3Control_FeedbackMessage.__import_type_support__()


class UR3Control(metaclass=Metaclass_UR3Control):

    # The goal message defined in the action definition.
    from ur3_moveit_control.action._ur3_control import UR3Control_Goal as Goal
    # The result message defined in the action definition.
    from ur3_moveit_control.action._ur3_control import UR3Control_Result as Result
    # The feedback message defined in the action definition.
    from ur3_moveit_control.action._ur3_control import UR3Control_Feedback as Feedback

    class Impl:

        # The send_goal service using a wrapped version of the goal message as a request.
        from ur3_moveit_control.action._ur3_control import UR3Control_SendGoal as SendGoalService
        # The get_result service using a wrapped version of the result message as a response.
        from ur3_moveit_control.action._ur3_control import UR3Control_GetResult as GetResultService
        # The feedback message with generic fields which wraps the feedback message.
        from ur3_moveit_control.action._ur3_control import UR3Control_FeedbackMessage as FeedbackMessage

        # The generic service to cancel a goal.
        from action_msgs.srv._cancel_goal import CancelGoal as CancelGoalService
        # The generic message for get the status of a goal.
        from action_msgs.msg._goal_status_array import GoalStatusArray as GoalStatusMessage

    def __init__(self):
        raise NotImplementedError('Action classes can not be instantiated')
