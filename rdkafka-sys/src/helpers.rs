//! Utility functions.

use crate::types::RDKafkaErrorCode;
use crate::types::RDKafkaErrorCode::*;
use crate::types::RDKafkaRespErr;
use crate::types::RDKafkaRespErr::*;

pub fn rd_kafka_resp_err_t_to_rdkafka_error(err: RDKafkaRespErr) -> RDKafkaErrorCode {
    match err {

        RD_KAFKA_RESP_ERR__BEGIN => Begin,
        RD_KAFKA_RESP_ERR__BAD_MSG => BadMessage,
        RD_KAFKA_RESP_ERR__BAD_COMPRESSION => BadCompression,
        RD_KAFKA_RESP_ERR__DESTROY => BrokerDestroy,
        RD_KAFKA_RESP_ERR__FAIL => Fail,
        RD_KAFKA_RESP_ERR__TRANSPORT => BrokerTransportFailure,
        RD_KAFKA_RESP_ERR__CRIT_SYS_RESOURCE => CriticalSystemResource,
        RD_KAFKA_RESP_ERR__RESOLVE => Resolve,
        RD_KAFKA_RESP_ERR__MSG_TIMED_OUT => MessageTimedOut,
        RD_KAFKA_RESP_ERR__PARTITION_EOF => PartitionEOF,
        RD_KAFKA_RESP_ERR__UNKNOWN_PARTITION => UnknownPartition,
        RD_KAFKA_RESP_ERR__FS => FileSystem,
        RD_KAFKA_RESP_ERR__UNKNOWN_TOPIC => UnknownTopic,
        RD_KAFKA_RESP_ERR__ALL_BROKERS_DOWN => AllBrokersDown,
        RD_KAFKA_RESP_ERR__INVALID_ARG => InvalidArgument,
        RD_KAFKA_RESP_ERR__TIMED_OUT => OperationTimedOut,
        RD_KAFKA_RESP_ERR__QUEUE_FULL => QueueFull,
        RD_KAFKA_RESP_ERR__ISR_INSUFF => ISRInsufficient,
        RD_KAFKA_RESP_ERR__NODE_UPDATE => NodeUpdate,
        RD_KAFKA_RESP_ERR__SSL => SSL,
        RD_KAFKA_RESP_ERR__WAIT_COORD => WaitingForCoordinator,
        RD_KAFKA_RESP_ERR__UNKNOWN_GROUP => UnknownGroup,
        RD_KAFKA_RESP_ERR__IN_PROGRESS => InProgress,
        RD_KAFKA_RESP_ERR__PREV_IN_PROGRESS => PreviousInProgress,
        RD_KAFKA_RESP_ERR__EXISTING_SUBSCRIPTION => ExistingSubscription,
        RD_KAFKA_RESP_ERR__ASSIGN_PARTITIONS => AssignPartitions,
        RD_KAFKA_RESP_ERR__REVOKE_PARTITIONS => RevokePartitions,
        RD_KAFKA_RESP_ERR__CONFLICT => Conflict,
        RD_KAFKA_RESP_ERR__STATE => State,
        RD_KAFKA_RESP_ERR__UNKNOWN_PROTOCOL => UnknownProtocol,
        RD_KAFKA_RESP_ERR__NOT_IMPLEMENTED => NotImplemented,
        RD_KAFKA_RESP_ERR__AUTHENTICATION => Authentication,
        RD_KAFKA_RESP_ERR__NO_OFFSET => NoOffset,
        RD_KAFKA_RESP_ERR__OUTDATED => Outdated,
        RD_KAFKA_RESP_ERR__TIMED_OUT_QUEUE => TimedOutQueue,
        RD_KAFKA_RESP_ERR__UNSUPPORTED_FEATURE => UnsupportedFeature,
        RD_KAFKA_RESP_ERR__WAIT_CACHE => WaitCache,
        RD_KAFKA_RESP_ERR__INTR => Interrupted,
        RD_KAFKA_RESP_ERR__KEY_SERIALIZATION => KeySerialization,
        RD_KAFKA_RESP_ERR__VALUE_SERIALIZATION => ValueSerialization,
        RD_KAFKA_RESP_ERR__KEY_DESERIALIZATION => KeyDeserialization,
        RD_KAFKA_RESP_ERR__VALUE_DESERIALIZATION => ValueDeserialization,
        RD_KAFKA_RESP_ERR__PARTIAL => Partial,
        RD_KAFKA_RESP_ERR__READ_ONLY => ReadOnly,
        RD_KAFKA_RESP_ERR__NOENT => NoEnt,
        RD_KAFKA_RESP_ERR__UNDERFLOW => Underflow,
        RD_KAFKA_RESP_ERR__INVALID_TYPE => InvalidType,
        RD_KAFKA_RESP_ERR__RETRY => Retry,
        RD_KAFKA_RESP_ERR__PURGE_QUEUE => PurgeQueue,
        RD_KAFKA_RESP_ERR__PURGE_INFLIGHT => PurgeInflight,
        RD_KAFKA_RESP_ERR__FATAL => Fatal,
        RD_KAFKA_RESP_ERR__INCONSISTENT => Inconsistent,
        RD_KAFKA_RESP_ERR__GAPLESS_GUARANTEE => GaplessGuarantee,
        RD_KAFKA_RESP_ERR__MAX_POLL_EXCEEDED => PollExceeded,
        RD_KAFKA_RESP_ERR__UNKNOWN_BROKER => UnknownBroker,
        RD_KAFKA_RESP_ERR__NOT_CONFIGURED => NotConfigured,
        RD_KAFKA_RESP_ERR__FENCED => Fenced,
        RD_KAFKA_RESP_ERR__APPLICATION => Application,
        RD_KAFKA_RESP_ERR__ASSIGNMENT_LOST => AssignmentLost,
        RD_KAFKA_RESP_ERR__NOOP => Noop,
        RD_KAFKA_RESP_ERR__AUTO_OFFSET_RESET => AutoOffsetReset,
        RD_KAFKA_RESP_ERR__END => End,
        RD_KAFKA_RESP_ERR_UNKNOWN => Unknown,
        RD_KAFKA_RESP_ERR_NO_ERROR => NoError,
        RD_KAFKA_RESP_ERR_OFFSET_OUT_OF_RANGE => OffsetOutOfRange,
        RD_KAFKA_RESP_ERR_INVALID_MSG => InvalidMessage,
        RD_KAFKA_RESP_ERR_UNKNOWN_TOPIC_OR_PART => UnknownTopicOrPartition,
        RD_KAFKA_RESP_ERR_INVALID_MSG_SIZE => InvalidMessageSize,
        RD_KAFKA_RESP_ERR_LEADER_NOT_AVAILABLE => LeaderNotAvailable,
        RD_KAFKA_RESP_ERR_NOT_LEADER_FOR_PARTITION => NotLeaderForPartition,
        RD_KAFKA_RESP_ERR_REQUEST_TIMED_OUT => RequestTimedOut,
        RD_KAFKA_RESP_ERR_BROKER_NOT_AVAILABLE => BrokerNotAvailable,
        RD_KAFKA_RESP_ERR_REPLICA_NOT_AVAILABLE => ReplicaNotAvailable,
        RD_KAFKA_RESP_ERR_MSG_SIZE_TOO_LARGE => MessageSizeTooLarge,
        RD_KAFKA_RESP_ERR_STALE_CTRL_EPOCH => StaleControllerEpoch,
        RD_KAFKA_RESP_ERR_OFFSET_METADATA_TOO_LARGE => OffsetMetadataTooLarge,
        RD_KAFKA_RESP_ERR_NETWORK_EXCEPTION => NetworkException,
        RD_KAFKA_RESP_ERR_COORDINATOR_LOAD_IN_PROGRESS => CoordinatorLoadInProgress,
        RD_KAFKA_RESP_ERR_COORDINATOR_NOT_AVAILABLE => CoordinatorNotAvailable,
        RD_KAFKA_RESP_ERR_NOT_COORDINATOR => NotCoordinator,
        RD_KAFKA_RESP_ERR_TOPIC_EXCEPTION => InvalidTopic,
        RD_KAFKA_RESP_ERR_RECORD_LIST_TOO_LARGE => MessageBatchTooLarge,
        RD_KAFKA_RESP_ERR_NOT_ENOUGH_REPLICAS => NotEnoughReplicas,
        RD_KAFKA_RESP_ERR_NOT_ENOUGH_REPLICAS_AFTER_APPEND => NotEnoughReplicasAfterAppend,
        RD_KAFKA_RESP_ERR_INVALID_REQUIRED_ACKS => InvalidRequiredAcks,
        RD_KAFKA_RESP_ERR_ILLEGAL_GENERATION => IllegalGeneration,
        RD_KAFKA_RESP_ERR_INCONSISTENT_GROUP_PROTOCOL => InconsistentGroupProtocol,
        RD_KAFKA_RESP_ERR_INVALID_GROUP_ID => InvalidGroupId,
        RD_KAFKA_RESP_ERR_UNKNOWN_MEMBER_ID => UnknownMemberId,
        RD_KAFKA_RESP_ERR_INVALID_SESSION_TIMEOUT => InvalidSessionTimeout,
        RD_KAFKA_RESP_ERR_REBALANCE_IN_PROGRESS => RebalanceInProgress,
        RD_KAFKA_RESP_ERR_INVALID_COMMIT_OFFSET_SIZE => InvalidCommitOffsetSize,
        RD_KAFKA_RESP_ERR_TOPIC_AUTHORIZATION_FAILED => TopicAuthorizationFailed,
        RD_KAFKA_RESP_ERR_GROUP_AUTHORIZATION_FAILED => GroupAuthorizationFailed,
        RD_KAFKA_RESP_ERR_CLUSTER_AUTHORIZATION_FAILED => ClusterAuthorizationFailed,
        RD_KAFKA_RESP_ERR_INVALID_TIMESTAMP => InvalidTimestamp,
        RD_KAFKA_RESP_ERR_UNSUPPORTED_SASL_MECHANISM => UnsupportedSASLMechanism,
        RD_KAFKA_RESP_ERR_ILLEGAL_SASL_STATE => IllegalSASLState,
        RD_KAFKA_RESP_ERR_UNSUPPORTED_VERSION => UnsupportedVersion,
        RD_KAFKA_RESP_ERR_TOPIC_ALREADY_EXISTS => TopicAlreadyExists,
        RD_KAFKA_RESP_ERR_INVALID_PARTITIONS => InvalidPartitions,
        RD_KAFKA_RESP_ERR_INVALID_REPLICATION_FACTOR => InvalidReplicationFactor,
        RD_KAFKA_RESP_ERR_INVALID_REPLICA_ASSIGNMENT => InvalidReplicaAssignment,
        RD_KAFKA_RESP_ERR_INVALID_CONFIG => InvalidConfig,
        RD_KAFKA_RESP_ERR_NOT_CONTROLLER => NotController,
        RD_KAFKA_RESP_ERR_INVALID_REQUEST => InvalidRequest,
        RD_KAFKA_RESP_ERR_UNSUPPORTED_FOR_MESSAGE_FORMAT => UnsupportedForMessageFormat,
        RD_KAFKA_RESP_ERR_POLICY_VIOLATION => PolicyViolation,
        RD_KAFKA_RESP_ERR_OUT_OF_ORDER_SEQUENCE_NUMBER => OutOfOrderSequenceNumber,
        RD_KAFKA_RESP_ERR_DUPLICATE_SEQUENCE_NUMBER => DuplicateSequenceNumber,
        RD_KAFKA_RESP_ERR_INVALID_PRODUCER_EPOCH => InvalidProducerEpoch,
        RD_KAFKA_RESP_ERR_INVALID_TXN_STATE => InvalidTransactionalState,
        RD_KAFKA_RESP_ERR_INVALID_PRODUCER_ID_MAPPING => InvalidProducerIdMapping,
        RD_KAFKA_RESP_ERR_INVALID_TRANSACTION_TIMEOUT => InvalidTransactionTimeout,
        RD_KAFKA_RESP_ERR_CONCURRENT_TRANSACTIONS => ConcurrentTransactions,
        RD_KAFKA_RESP_ERR_TRANSACTION_COORDINATOR_FENCED => TransactionCoordinatorFenced,
        RD_KAFKA_RESP_ERR_TRANSACTIONAL_ID_AUTHORIZATION_FAILED => {
            TransactionalIdAuthorizationFailed
        }
        RD_KAFKA_RESP_ERR_SECURITY_DISABLED => SecurityDisabled,
        RD_KAFKA_RESP_ERR_OPERATION_NOT_ATTEMPTED => OperationNotAttempted,
        RD_KAFKA_RESP_ERR_KAFKA_STORAGE_ERROR => KafkaStorageError,
        RD_KAFKA_RESP_ERR_LOG_DIR_NOT_FOUND => LogDirNotFound,
        RD_KAFKA_RESP_ERR_SASL_AUTHENTICATION_FAILED => SaslAuthenticationFailed,
        RD_KAFKA_RESP_ERR_UNKNOWN_PRODUCER_ID => UnknownProducerId,
        RD_KAFKA_RESP_ERR_REASSIGNMENT_IN_PROGRESS => ReassignmentInProgress,
        RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_AUTH_DISABLED => DelegationTokenAuthDisabled,
        RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_NOT_FOUND => DelegationTokenNotFound,
        RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_OWNER_MISMATCH => DelegationTokenOwnerMismatch,
        RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_REQUEST_NOT_ALLOWED => DelegationTokenRequestNotAllowed,
        RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_AUTHORIZATION_FAILED => {
            DelegationTokenAuthorizationFailed
        }
        RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_EXPIRED => DelegationTokenExpired,
        RD_KAFKA_RESP_ERR_INVALID_PRINCIPAL_TYPE => InvalidPrincipalType,
        RD_KAFKA_RESP_ERR_NON_EMPTY_GROUP => NonEmptyGroup,
        RD_KAFKA_RESP_ERR_GROUP_ID_NOT_FOUND => GroupIdNotFound,
        RD_KAFKA_RESP_ERR_FETCH_SESSION_ID_NOT_FOUND => FetchSessionIdNotFound,
        RD_KAFKA_RESP_ERR_INVALID_FETCH_SESSION_EPOCH => InvalidFetchSessionEpoch,
        RD_KAFKA_RESP_ERR_LISTENER_NOT_FOUND => ListenerNotFound,
        RD_KAFKA_RESP_ERR_TOPIC_DELETION_DISABLED => TopicDeletionDisabled,
        RD_KAFKA_RESP_ERR_FENCED_LEADER_EPOCH => FencedLeaderEpoch,
        RD_KAFKA_RESP_ERR_UNKNOWN_LEADER_EPOCH => UnknownLeaderEpoch,
        RD_KAFKA_RESP_ERR_UNSUPPORTED_COMPRESSION_TYPE => UnsupportedCompressionType,
        RD_KAFKA_RESP_ERR_STALE_BROKER_EPOCH => StaleBrokerEpoch,
        RD_KAFKA_RESP_ERR_OFFSET_NOT_AVAILABLE => OffsetNotAvailable,
        RD_KAFKA_RESP_ERR_MEMBER_ID_REQUIRED => MemberIdRequired,
        RD_KAFKA_RESP_ERR_PREFERRED_LEADER_NOT_AVAILABLE => PreferredLeaderNotAvailable,
        RD_KAFKA_RESP_ERR_GROUP_MAX_SIZE_REACHED => GroupMaxSizeReached,
        RD_KAFKA_RESP_ERR_FENCED_INSTANCE_ID => FencedInstanceId,
        RD_KAFKA_RESP_ERR_ELIGIBLE_LEADERS_NOT_AVAILABLE => EligibleLeadersNotAvailable,
        RD_KAFKA_RESP_ERR_ELECTION_NOT_NEEDED => ElectionNotNeeded,
        RD_KAFKA_RESP_ERR_NO_REASSIGNMENT_IN_PROGRESS => NoReassignmentInProgress,
        RD_KAFKA_RESP_ERR_GROUP_SUBSCRIBED_TO_TOPIC => GroupSubscribedToTopic,
        RD_KAFKA_RESP_ERR_INVALID_RECORD => InvalidRecord,
        RD_KAFKA_RESP_ERR_UNSTABLE_OFFSET_COMMIT => UnstableOffsetCommit,
        RD_KAFKA_RESP_ERR_THROTTLING_QUOTA_EXCEEDED => ThrottlingQuotaExceeded,
        RD_KAFKA_RESP_ERR_PRODUCER_FENCED => ProducerFenced,
        RD_KAFKA_RESP_ERR_RESOURCE_NOT_FOUND => ResourceNotFound,
        RD_KAFKA_RESP_ERR_DUPLICATE_RESOURCE => DuplicateResource,
        RD_KAFKA_RESP_ERR_UNACCEPTABLE_CREDENTIAL => UnacceptableCredential,
        RD_KAFKA_RESP_ERR_INCONSISTENT_VOTER_SET => InconsistentVoterSet,
        RD_KAFKA_RESP_ERR_INVALID_UPDATE_VERSION => InvalidUpdateVersion,
        RD_KAFKA_RESP_ERR_FEATURE_UPDATE_FAILED => FeatureUpdateFailed,
        RD_KAFKA_RESP_ERR_PRINCIPAL_DESERIALIZATION_FAILURE => PrincipalDeserializationFailure,
        RD_KAFKA_RESP_ERR_END_ALL => EndAll,
        RD_KAFKA_RESP_ERR__LOG_TRUNCATION => LogTruncation,

        RD_KAFKA_RESP_ERR__INVALID_DIFFERENT_RECORD => BadMessage,
        RD_KAFKA_RESP_ERR_UNKNOWN_TOPIC_ID => BadMessage,
        RD_KAFKA_RESP_ERR_FENCED_MEMBER_EPOCH => BadMessage,
        RD_KAFKA_RESP_ERR_UNRELEASED_INSTANCE_ID => BadMessage,
        RD_KAFKA_RESP_ERR_UNSUPPORTED_ASSIGNOR => BadMessage,
        RD_KAFKA_RESP_ERR_STALE_MEMBER_EPOCH => BadMessage,

        RD_KAFKA_RESP_ERR_UNKNOWN_SUBSCRIPTION_ID => BadMessage,
        RD_KAFKA_RESP_ERR_TELEMETRY_TOO_LARGE => BadMessage,
    }
}
