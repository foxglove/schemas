import numpy as np

from foxglove.schemas import Pose, Quaternion, Vector3


def point_to_pose(point: np.ndarray) -> Pose:
    """
    Convert a point to a pose.
    :point: np.ndarray((3,))
    """
    assert point.shape == (3,), "Expected 3 coordinates to define a pose"
    return Pose(
        position=Vector3(x=point[0], y=point[1], z=point[2]),
        orientation=Quaternion(x=0, y=0, z=0, w=1),
    )
