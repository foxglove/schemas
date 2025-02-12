from math import cos, sin

from foxglove.schemas import Quaternion


def euler_to_quaternion(roll: float, pitch: float, yaw: float) -> Quaternion:
    """Convert Euler angles to a rotation quaternion

    See e.g. https://danceswithcode.net/engineeringnotes/quaternions/quaternions.html

    :param roll: rotation around X axis (radians)
    :param pitch: rotation around Y axis (radians)
    :param yaw: rotation around Z axis (radians)
    :returns: a protobuf Quaternion
    """
    roll, pitch, yaw = roll * 0.5, pitch * 0.5, yaw * 0.5

    sin_r, cos_r = sin(roll), cos(roll)
    sin_p, cos_p = sin(pitch), cos(pitch)
    sin_y, cos_y = sin(yaw), cos(yaw)

    w = cos_r * cos_p * cos_y + sin_r * sin_p * sin_y
    x = sin_r * cos_p * cos_y - cos_r * sin_p * sin_y
    y = cos_r * sin_p * cos_y + sin_r * cos_p * sin_y
    z = cos_r * cos_p * sin_y - sin_r * sin_p * cos_y

    return Quaternion(x=x, y=y, z=z, w=w)
