import numpy as np

from foxglove.schemas import Color, LinePrimitive, Point3


def points_to_line(
    points: np.ndarray, color: np.ndarray, thickness: float = 0.02
) -> LinePrimitive:
    """
    Convert two points to a line primitive.
    :points: np.ndarray((2, 3))
    """
    assert points.shape[0] == 2, "Expected 2 points to define a line"
    assert points.shape[1] == 3, "Expected 3 coordinates to define a point"
    return LinePrimitive(
        thickness=thickness,
        points=[Point3(x=point[0], y=point[1], z=point[2]) for point in points],
        color=Color(r=color[0], g=color[1], b=color[2], a=color[3]),
    )
