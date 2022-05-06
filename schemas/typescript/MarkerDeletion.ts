// Generated from MarkerDeletion by @foxglove/message-schemas

import { MarkerDeletionType } from "./MarkerDeletionType";
import { Time } from "./Time";

/** Deletion command to remove previously published markers */
export type MarkerDeletion = {
  /** Timestamp of the marker. Only matching markers earlier than this timestamp will be deleted. */
  timestamp: Time;

  /** Type of deletion action to perform */
  type: MarkerDeletionType;

  /** Namespace which must match if `kind` is `MATCHING_NAMESPACE_AND_ID` or `MATCHING_NAMESPACE`. */
  namespace: string;

  /** Numeric identifier which must match if `kind` is `MATCHING_NAMESPACE_AND_ID`. */
  id: string;
};
