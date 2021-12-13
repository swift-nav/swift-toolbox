pub(crate) const HEARTBEAT_LOCK_MUTEX_FAILURE: &str = "unable to lock heartbeat mutex";
pub const SHARED_STATE_LOCK_MUTEX_FAILURE: &str = "unable to lock shared_state mutex";
pub(crate) const UPDATE_STATUS_LOCK_MUTEX_FAILURE: &str = "unable to lock update status mutex";
pub(crate) const CAP_N_PROTO_SERIALIZATION_FAILURE: &str = "unable to serialize capnproto message";
#[allow(dead_code)]
pub const CAP_N_PROTO_DESERIALIZATION_FAILURE: &str = "unable to deserialize capnproto message";
pub const CONVERT_TO_STR_FAILURE: &str = "error converting to str";
pub(crate) const UNABLE_TO_STOP_TIMER_THREAD_FAILURE: &str = "unable to kill running timer thread";
pub(crate) const UNABLE_TO_SEND_INS_UPDATE_FAILURE: &str = "unable to send an ins status update";
pub(crate) const THREAD_JOIN_FAILURE: &str = "thread join failure";
pub(crate) const CONSOLE_LOG_JSON_TO_STRING_FAILURE: &str = "unable to convert json to string";
pub(crate) const CROSSBEAM_SCOPE_UNWRAP_FAILURE: &str = "unable to unwrap crossbeam scope";
pub(crate) const UNABLE_TO_CLONE_UPDATE_SHARED: &str = "unable to clone update shared";
pub(crate) const FILEIO_CHANNEL_SEND_FAILURE: &str =
    "failure attempting to send via fileio channel";
pub(crate) const SOLUTION_POSITION_UNIT_SELECTION_NOT_AVAILABLE: &str =
    "solution position unit selection not available";
pub(crate) const PROCESS_MESSAGES_FAILURE: &str = "process_messages thread panicked";
pub(crate) const THREAD_START_FAILURE: &str = "failed to start a new thread";
