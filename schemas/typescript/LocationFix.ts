// Generated from LocationFix by @foxglove/schemas

import { PositionCovarianceType } from "./PositionCovarianceType";

/** A navigation satellite fix for any Global Navigation Satellite System */
export type LocationFix = {
  /** Latitude in degrees */
  latitude: number;

  /** Longitude in degrees */
  longitude: number;

  /** Altitude in meters */
  altitude: number;

  /** Position covariance (m^2) defined relative to a tangential plane through the reported position. The components are East, North, and Up (ENU), in row-major order. */
  position_covariance: [number, number, number, number, number, number, number, number, number];

  /** If `position_covariance` is available, `position_covariance_type` must be set to indicate the type of covariance. */
  position_covariance_type: PositionCovarianceType;
};
