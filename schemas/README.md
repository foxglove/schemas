# Foxglove schemas

Generated by https://github.com/foxglove/schemas

## Contents

- [enum LogLevel](#enum-loglevel)
- [enum NumericType](#enum-numerictype)
- [enum PointsAnnotationType](#enum-pointsannotationtype)
- [enum PositionCovarianceType](#enum-positioncovariancetype)
- [CameraCalibration](#cameracalibration)
- [CircleAnnotation](#circleannotation)
- [Color](#color)
- [CompressedImage](#compressedimage)
- [FrameTransform](#frametransform)
- [GeoJSON](#geojson)
- [Grid](#grid)
- [ImageAnnotations](#imageannotations)
- [LaserScan](#laserscan)
- [LocationFix](#locationfix)
- [Log](#log)
- [PackedElementField](#packedelementfield)
- [Point2](#point2)
- [Point3](#point3)
- [PointCloud](#pointcloud)
- [PointsAnnotation](#pointsannotation)
- [Pose](#pose)
- [PoseInFrame](#poseinframe)
- [PosesInFrame](#posesinframe)
- [Quaternion](#quaternion)
- [RawImage](#rawimage)
- [Vector2](#vector2)
- [Vector3](#vector3)

----

## enum LogLevel

Log level

name | value | description
---- | ----- | -----------
`UNKNOWN` | 0 | 
`DEBUG` | 1 | 
`INFO` | 2 | 
`WARNING` | 3 | 
`ERROR` | 4 | 
`FATAL` | 5 | 



## enum NumericType

Numeric type

name | value | description
---- | ----- | -----------
`UNKNOWN` | 0 | 
`UINT8` | 1 | 
`INT8` | 2 | 
`UINT16` | 3 | 
`INT16` | 4 | 
`UINT32` | 5 | 
`INT32` | 6 | 
`FLOAT32` | 7 | 
`FLOAT64` | 8 | 



## enum PointsAnnotationType

Type of points annotation

name | value | description
---- | ----- | -----------
`UNKNOWN` | 0 | 
`POINTS` | 1 | 
`LINE_LOOP` | 2 | 
`LINE_STRIP` | 3 | 
`LINE_LIST` | 4 | 



## enum PositionCovarianceType

Type of position covariance

name | value | description
---- | ----- | -----------
`UNKNOWN` | 0 | 
`APPROXIMATED` | 1 | 
`DIAGONAL_KNOWN` | 2 | 
`KNOWN` | 3 | 



## CameraCalibration

Camera calibration parameters

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>timestamp</code></td>
<td>

time

</td>
<td>

Timestamp of calibration data

</td>
</tr>
<tr>
<td><code>width</code></td>
<td>

uint32

</td>
<td>

Image width

</td>
</tr>
<tr>
<td><code>height</code></td>
<td>

uint32

</td>
<td>

Image height

</td>
</tr>
<tr>
<td><code>distortion_model</code></td>
<td>

string

</td>
<td>

Name of distortion model

</td>
</tr>
<tr>
<td><code>D</code></td>
<td>

float64[]

</td>
<td>

Distortion parameters

</td>
</tr>
<tr>
<td><code>K</code></td>
<td>

float64[9]

</td>
<td>

Intrinsic camera matrix (3x3 row-major matrix)

A 3x3 row-major matrix for the raw (distorted) image.

Projects 3D points in the camera coordinate frame to 2D pixel coordinates using the focal lengths (fx, fy) and principal point (cx, cy).

```
    [fx  0 cx]
K = [ 0 fy cy]
    [ 0  0  1]
```


</td>
</tr>
<tr>
<td><code>R</code></td>
<td>

float64[9]

</td>
<td>

Rectification matrix (3x3 row-major matrix)

A rotation matrix aligning the camera coordinate system to the ideal stereo image plane so that epipolar lines in both stereo images are parallel.

</td>
</tr>
<tr>
<td><code>P</code></td>
<td>

float64[12]

</td>
<td>

Projection/camera matrix (3x4 row-major matrix)

```
    [fx'  0  cx' Tx]
P = [ 0  fy' cy' Ty]
    [ 0   0   1   0]
```

By convention, this matrix specifies the intrinsic (camera) matrix of the processed (rectified) image. That is, the left 3x3 portion is the normal camera intrinsic matrix for the rectified image.

It projects 3D points in the camera coordinate frame to 2D pixel coordinates using the focal lengths (fx', fy') and principal point (cx', cy') - these may differ from the values in K.

For monocular cameras, Tx = Ty = 0. Normally, monocular cameras will also have R = the identity and P[1:3,1:3] = K.

For a stereo pair, the fourth column [Tx Ty 0]' is related to the position of the optical center of the second camera in the first camera's frame. We assume Tz = 0 so both cameras are in the same stereo image plane. The first camera always has Tx = Ty = 0. For the right (second) camera of a horizontal stereo pair, Ty = 0 and Tx = -fx' * B, where B is the baseline between the cameras.

Given a 3D point [X Y Z]', the projection (x, y) of the point onto the rectified image is given by:

```
[u v w]' = P * [X Y Z 1]'
       x = u / w
       y = v / w
```

This holds for both images of a stereo pair.


</td>
</tr>
</table>

## CircleAnnotation

A circle annotation on a 2D image

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>timestamp</code></td>
<td>

time

</td>
<td>

Timestamp of circle

</td>
</tr>
<tr>
<td><code>position</code></td>
<td>

[Point2](#point2)

</td>
<td>

Center of the circle in 2D image coordinates

</td>
</tr>
<tr>
<td><code>diameter</code></td>
<td>

float64

</td>
<td>

Circle diameter

</td>
</tr>
<tr>
<td><code>thickness</code></td>
<td>

float64

</td>
<td>

Line thickness

</td>
</tr>
<tr>
<td><code>fill_color</code></td>
<td>

[Color](#color)

</td>
<td>

Fill color

</td>
</tr>
<tr>
<td><code>outline_color</code></td>
<td>

[Color](#color)

</td>
<td>

Outline color

</td>
</tr>
</table>

## Color

A color in RGBA format

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>r</code></td>
<td>

float64

</td>
<td>

Red value between 0 and 1

</td>
</tr>
<tr>
<td><code>g</code></td>
<td>

float64

</td>
<td>

Green value between 0 and 1

</td>
</tr>
<tr>
<td><code>b</code></td>
<td>

float64

</td>
<td>

Blue value between 0 and 1

</td>
</tr>
<tr>
<td><code>a</code></td>
<td>

float64

</td>
<td>

Alpha value between 0 and 1

</td>
</tr>
</table>

## CompressedImage

A compressed image

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>timestamp</code></td>
<td>

time

</td>
<td>

Timestamp of image

</td>
</tr>
<tr>
<td><code>data</code></td>
<td>

bytes

</td>
<td>

Compressed image data

</td>
</tr>
<tr>
<td><code>format</code></td>
<td>

string

</td>
<td>

Image format

</td>
</tr>
</table>

## FrameTransform

A transform between two reference frames in 3D space

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>timestamp</code></td>
<td>

time

</td>
<td>

Timestamp of transform

</td>
</tr>
<tr>
<td><code>parent_frame_id</code></td>
<td>

string

</td>
<td>

Name of the parent frame

</td>
</tr>
<tr>
<td><code>child_frame_id</code></td>
<td>

string

</td>
<td>

Name of the child frame

</td>
</tr>
<tr>
<td><code>translation</code></td>
<td>

[Vector3](#vector3)

</td>
<td>

Translation component of the transform

</td>
</tr>
<tr>
<td><code>rotation</code></td>
<td>

[Quaternion](#quaternion)

</td>
<td>

Rotation component of the transform

</td>
</tr>
</table>

## GeoJSON

GeoJSON data for annotating maps

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>geojson</code></td>
<td>

string

</td>
<td>

GeoJSON data encoded as a UTF-8 string

</td>
</tr>
</table>

## Grid

A 2D grid of data

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>timestamp</code></td>
<td>

time

</td>
<td>

Timestamp of grid

</td>
</tr>
<tr>
<td><code>frame_id</code></td>
<td>

string

</td>
<td>

Frame of reference

</td>
</tr>
<tr>
<td><code>pose</code></td>
<td>

[Pose](#pose)

</td>
<td>

Origin of grid's corner relative to frame of reference; grid is positioned in the x-y plane relative to this origin

</td>
</tr>
<tr>
<td><code>column_count</code></td>
<td>

uint32

</td>
<td>

Number of grid columns

</td>
</tr>
<tr>
<td><code>cell_size</code></td>
<td>

[Vector2](#vector2)

</td>
<td>

Size of single grid cell along x and y axes, relative to `pose`

</td>
</tr>
<tr>
<td><code>row_stride</code></td>
<td>

uint32

</td>
<td>

Number of bytes between rows in `data`

</td>
</tr>
<tr>
<td><code>cell_stride</code></td>
<td>

uint32

</td>
<td>

Number of bytes between cells within a row in `data`

</td>
</tr>
<tr>
<td><code>fields</code></td>
<td>

[PackedElementField](#packedelementfield)[]

</td>
<td>

Fields in `data`

</td>
</tr>
<tr>
<td><code>data</code></td>
<td>

bytes

</td>
<td>

Grid cell data, interpreted using `fields`, in row-major (y-major) order

</td>
</tr>
</table>

## ImageAnnotations

Array of annotations for a 2D image

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>circles</code></td>
<td>

[CircleAnnotation](#circleannotation)[]

</td>
<td>

Circle annotations

</td>
</tr>
<tr>
<td><code>points</code></td>
<td>

[PointsAnnotation](#pointsannotation)[]

</td>
<td>

Points annotations

</td>
</tr>
</table>

## LaserScan

A single scan from a planar laser range-finder

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>timestamp</code></td>
<td>

time

</td>
<td>

Timestamp of scan

</td>
</tr>
<tr>
<td><code>frame_id</code></td>
<td>

string

</td>
<td>

Frame of reference

</td>
</tr>
<tr>
<td><code>pose</code></td>
<td>

[Pose](#pose)

</td>
<td>

Origin of scan relative to frame of reference; points are positioned in the x-y plane relative to this origin; angles are interpreted as counterclockwise rotations around the z axis with 0 rad being in the +x direction

</td>
</tr>
<tr>
<td><code>start_angle</code></td>
<td>

float64

</td>
<td>

Bearing of first point, in radians

</td>
</tr>
<tr>
<td><code>end_angle</code></td>
<td>

float64

</td>
<td>

Bearing of last point, in radians

</td>
</tr>
<tr>
<td><code>ranges</code></td>
<td>

float64[]

</td>
<td>

Distance of detections from origin; assumed to be at equally-spaced angles between `start_angle` and `end_angle`

</td>
</tr>
<tr>
<td><code>intensities</code></td>
<td>

float64[]

</td>
<td>

Intensity of detections

</td>
</tr>
</table>

## LocationFix

A navigation satellite fix for any Global Navigation Satellite System

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>latitude</code></td>
<td>

float64

</td>
<td>

Latitude in degrees

</td>
</tr>
<tr>
<td><code>longitude</code></td>
<td>

float64

</td>
<td>

Longitude in degrees

</td>
</tr>
<tr>
<td><code>altitude</code></td>
<td>

float64

</td>
<td>

Altitude in meters

</td>
</tr>
<tr>
<td><code>position_covariance</code></td>
<td>

float64[9]

</td>
<td>

Position covariance (m^2) defined relative to a tangential plane through the reported position. The components are East, North, and Up (ENU), in row-major order.

</td>
</tr>
<tr>
<td><code>position_covariance_type</code></td>
<td>

[enum PositionCovarianceType](#enum-positioncovariancetype)

</td>
<td>

If `position_covariance` is available, `position_covariance_type` must be set to indicate the type of covariance.

</td>
</tr>
</table>

## Log

A log message

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>timestamp</code></td>
<td>

time

</td>
<td>

Timestamp of log message

</td>
</tr>
<tr>
<td><code>level</code></td>
<td>

[enum LogLevel](#enum-loglevel)

</td>
<td>

Log level

</td>
</tr>
<tr>
<td><code>message</code></td>
<td>

string

</td>
<td>

Log message

</td>
</tr>
<tr>
<td><code>name</code></td>
<td>

string

</td>
<td>

Process or node name

</td>
</tr>
<tr>
<td><code>file</code></td>
<td>

string

</td>
<td>

Filename

</td>
</tr>
<tr>
<td><code>line</code></td>
<td>

uint32

</td>
<td>

Line number in the file

</td>
</tr>
</table>

## PackedElementField

A field present within each element in a byte array of packed elements.

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>name</code></td>
<td>

string

</td>
<td>

Name of the field

</td>
</tr>
<tr>
<td><code>offset</code></td>
<td>

uint32

</td>
<td>

Byte offset from start of data buffer

</td>
</tr>
<tr>
<td><code>type</code></td>
<td>

[enum NumericType](#enum-numerictype)

</td>
<td>

Type of data in the field. Integers are stored using little-endian byte order.

</td>
</tr>
</table>

## Point2

A point representing a position in 2D space

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>x</code></td>
<td>

float64

</td>
<td>

x coordinate position

</td>
</tr>
<tr>
<td><code>y</code></td>
<td>

float64

</td>
<td>

y coordinate position

</td>
</tr>
</table>

## Point3

A point representing a position in 3D space

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>x</code></td>
<td>

float64

</td>
<td>

x coordinate position

</td>
</tr>
<tr>
<td><code>y</code></td>
<td>

float64

</td>
<td>

y coordinate position

</td>
</tr>
<tr>
<td><code>z</code></td>
<td>

float64

</td>
<td>

z coordinate position

</td>
</tr>
</table>

## PointCloud

A collection of N-dimensional points, which may contain additional fields with information like normals, intensity, etc.

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>timestamp</code></td>
<td>

time

</td>
<td>

Timestamp of point cloud

</td>
</tr>
<tr>
<td><code>frame_id</code></td>
<td>

string

</td>
<td>

Frame of reference

</td>
</tr>
<tr>
<td><code>pose</code></td>
<td>

[Pose](#pose)

</td>
<td>

The origin of the point cloud relative to the frame of reference

</td>
</tr>
<tr>
<td><code>point_stride</code></td>
<td>

uint32

</td>
<td>

Number of bytes between points in the `data`

</td>
</tr>
<tr>
<td><code>fields</code></td>
<td>

[PackedElementField](#packedelementfield)[]

</td>
<td>

Fields in the `data`

</td>
</tr>
<tr>
<td><code>data</code></td>
<td>

bytes

</td>
<td>

Point data, interpreted using `fields`

</td>
</tr>
</table>

## PointsAnnotation

An array of points on a 2D image

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>timestamp</code></td>
<td>

time

</td>
<td>

Timestamp of annotation

</td>
</tr>
<tr>
<td><code>type</code></td>
<td>

[enum PointsAnnotationType](#enum-pointsannotationtype)

</td>
<td>

Type of points annotation to draw

</td>
</tr>
<tr>
<td><code>points</code></td>
<td>

[Point2](#point2)[]

</td>
<td>

Points in 2D image coordinates

</td>
</tr>
<tr>
<td><code>outline_colors</code></td>
<td>

[Color](#color)[]

</td>
<td>

Outline colors

</td>
</tr>
<tr>
<td><code>fill_color</code></td>
<td>

[Color](#color)

</td>
<td>

Fill color

</td>
</tr>
</table>

## Pose

A position and orientation for an object or reference frame in 3D space

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>position</code></td>
<td>

[Vector3](#vector3)

</td>
<td>

Point denoting position in 3D space

</td>
</tr>
<tr>
<td><code>orientation</code></td>
<td>

[Quaternion](#quaternion)

</td>
<td>

Quaternion denoting orientation in 3D space

</td>
</tr>
</table>

## PoseInFrame

A timestamped pose for an object or reference frame in 3D space

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>timestamp</code></td>
<td>

time

</td>
<td>

Timestamp of pose

</td>
</tr>
<tr>
<td><code>frame_id</code></td>
<td>

string

</td>
<td>

Frame of reference for pose position and orientation

</td>
</tr>
<tr>
<td><code>pose</code></td>
<td>

[Pose](#pose)

</td>
<td>

Pose in 3D space

</td>
</tr>
</table>

## PosesInFrame

An array of timestamped poses for an object or reference frame in 3D space

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>timestamp</code></td>
<td>

time

</td>
<td>

Timestamp of pose

</td>
</tr>
<tr>
<td><code>frame_id</code></td>
<td>

string

</td>
<td>

Frame of reference for pose position and orientation

</td>
</tr>
<tr>
<td><code>poses</code></td>
<td>

[Pose](#pose)[]

</td>
<td>

Poses in 3D space

</td>
</tr>
</table>

## Quaternion

A [quaternion](https://eater.net/quaternions) representing a rotation in 3D space

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>x</code></td>
<td>

float64

</td>
<td>

x value

</td>
</tr>
<tr>
<td><code>y</code></td>
<td>

float64

</td>
<td>

y value

</td>
</tr>
<tr>
<td><code>z</code></td>
<td>

float64

</td>
<td>

z value

</td>
</tr>
<tr>
<td><code>w</code></td>
<td>

float64

</td>
<td>

w value

</td>
</tr>
</table>

## RawImage

A raw image

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>timestamp</code></td>
<td>

time

</td>
<td>

Timestamp of image

</td>
</tr>
<tr>
<td><code>width</code></td>
<td>

uint32

</td>
<td>

Image width

</td>
</tr>
<tr>
<td><code>height</code></td>
<td>

uint32

</td>
<td>

Image height

</td>
</tr>
<tr>
<td><code>encoding</code></td>
<td>

string

</td>
<td>

Encoding of the raw image data

</td>
</tr>
<tr>
<td><code>step</code></td>
<td>

uint32

</td>
<td>

Byte length of a single row

</td>
</tr>
<tr>
<td><code>data</code></td>
<td>

bytes

</td>
<td>

Raw image data

</td>
</tr>
</table>

## Vector2

A vector in 2D space that represents a direction only

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>x</code></td>
<td>

float64

</td>
<td>

x coordinate length

</td>
</tr>
<tr>
<td><code>y</code></td>
<td>

float64

</td>
<td>

y coordinate length

</td>
</tr>
</table>

## Vector3

A vector in 3D space that represents a direction only

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
<tr>
<td><code>x</code></td>
<td>

float64

</td>
<td>

x coordinate length

</td>
</tr>
<tr>
<td><code>y</code></td>
<td>

float64

</td>
<td>

y coordinate length

</td>
</tr>
<tr>
<td><code>z</code></td>
<td>

float64

</td>
<td>

z coordinate length

</td>
</tr>
</table>