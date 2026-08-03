"""Shared side-grasp geometry and tuning for the simulated bottle.

Both the one-shot client and the random trial runner import this module so a
change to the grasp pose cannot silently make the two test paths different.
All coordinates are expressed in ``base_link`` unless stated otherwise.
"""

import math


BOTTLE_X = 0.62
BOTTLE_Y = 0.0
BOTTLE_Z = 0.05

APPROACH_CLEARANCE = 0.12
TCP_GRASP_OFFSET = 0.06
GRASP_DEPTH_OFFSET = 0.015
LIFT_OFFSET = 0.15
OPEN_POSITION = 0.06
CLOSE_POSITION = 0.042
MAX_EFFORT = 100.0

TRANSPORT_Y_MIN = 0.06
TRANSPORT_Y_MAX = 0.10
TRANSPORT_X_MAX = 0.02

POUR_WRIST_ANGLE_DEG = 100.0
WRIST_3_POSITION_LIMIT = 2.0 * math.pi


def radial_side_grasp_geometry(
        object_x,
        object_y,
        approach_clearance=APPROACH_CLEARANCE,
        grasp_depth_offset=GRASP_DEPTH_OFFSET):
    """Return a horizontal TCP approach directed radially at the bottle."""

    radius = math.hypot(object_x, object_y)
    if radius < 1e-6:
        raise ValueError('Object cannot be placed at the base_link origin')

    direction_x = object_x / radius
    direction_y = object_y / radius
    grasp_x = object_x - grasp_depth_offset * direction_x
    grasp_y = object_y - grasp_depth_offset * direction_y
    approach_x = grasp_x - approach_clearance * direction_x
    approach_y = grasp_y - approach_clearance * direction_y

    # q=(0.5, 0.5, 0.5, 0.5) points TCP +Z along base_link +X. Rotate that
    # pose about base_link Z so the gripper points toward each random bottle.
    yaw = math.atan2(object_y, object_x)
    cosine = math.cos(0.5 * yaw)
    sine = math.sin(0.5 * yaw)
    quaternion = (
        0.5 * (cosine - sine),
        0.5 * (cosine + sine),
        0.5 * (cosine + sine),
        0.5 * (cosine - sine),
    )

    return {
        'approach_x': approach_x,
        'approach_y': approach_y,
        'advance_x': approach_clearance * direction_x,
        'advance_y': approach_clearance * direction_y,
        'grasp_x': grasp_x,
        'grasp_y': grasp_y,
        'quaternion': quaternion,
        'yaw': yaw,
    }


def pouring_joint_goal(current_arm_joints, requested_angle_deg):
    """Rotate only wrist_3 while keeping the other five arm joints fixed."""

    if len(current_arm_joints) != 6:
        raise ValueError('Pouring requires exactly six current arm joints')

    requested_delta = math.radians(float(requested_angle_deg))
    if abs(requested_delta) < 1e-6:
        raise ValueError('Pouring wrist angle must be non-zero')

    current_wrist = float(current_arm_joints[5])
    preferred_target = current_wrist + requested_delta
    alternate_delta = -requested_delta
    alternate_target = current_wrist + alternate_delta

    if -WRIST_3_POSITION_LIMIT <= preferred_target <= WRIST_3_POSITION_LIMIT:
        applied_delta = requested_delta
        target_wrist = preferred_target
    elif -WRIST_3_POSITION_LIMIT <= alternate_target <= WRIST_3_POSITION_LIMIT:
        # Preserve the requested tilt magnitude while avoiding the opposite
        # wrist limit. The log tells the operator that the direction flipped.
        applied_delta = alternate_delta
        target_wrist = alternate_target
    else:
        raise ValueError(
            'No wrist_3 target is available inside the +/-2*pi limits'
        )

    target_joints = [float(value) for value in current_arm_joints]
    target_joints[5] = target_wrist
    return target_joints, applied_delta
