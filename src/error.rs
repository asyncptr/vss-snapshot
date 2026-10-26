use std::fmt;

use windows_core::HRESULT;

use crate::bindings::*;

/// A `Result` whose error is a failed VSS or COM call.
pub type Result<T> = std::result::Result<T, Error>;

/// A failed VSS or COM call.
///
/// Carries the name of the call that failed and the raw `HRESULT` it returned.
/// Use [`kind`](Self::kind) to branch on the well-known VSS failures.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Error {
    operation: &'static str,
    code: i32,
}

impl Error {
    #[cfg_attr(not(test), allow(dead_code))] // Used by the snapshot API.
    pub(crate) fn new(operation: &'static str, code: HRESULT) -> Self {
        Error {
            operation,
            code: code.0,
        }
    }

    /// The VSS or COM call that failed, e.g. `"DoSnapshotSet"`.
    pub fn operation(&self) -> &'static str {
        self.operation
    }

    /// The raw `HRESULT` the call returned.
    pub fn hresult(&self) -> i32 {
        self.code
    }

    /// The well-known failure this `HRESULT` stands for, if any.
    pub fn kind(&self) -> ErrorKind {
        ErrorKind::from_hresult(HRESULT(self.code))
    }

    /// Whether the same call may succeed if retried after a short wait.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self.kind(),
            ErrorKind::SnapshotSetInProgress
                | ErrorKind::WritesTimeout
                | ErrorKind::WriterRetryable
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind().description() {
            Some(description) => {
                write!(
                    f,
                    "{} failed: {} (0x{:08X})",
                    self.operation, description, self.code as u32
                )
            }
            None => write!(
                f,
                "{} failed with HRESULT 0x{:08X}",
                self.operation, self.code as u32
            ),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("operation", &self.operation)
            .field("kind", &self.kind())
            .field("hresult", &format_args!("0x{:08X}", self.code as u32))
            .finish()
    }
}

impl std::error::Error for Error {}

/// Well-known VSS and COM failures.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// The process is not elevated and lacks the backup privilege.
    AccessDenied,
    /// An argument was rejected, e.g. a malformed volume name.
    InvalidArgument,
    /// VSS calls were made in the wrong order.
    BadState,
    /// The snapshot or snapshot set does not exist.
    ObjectNotFound,
    /// The volume cannot be shadow-copied, e.g. a FAT or network volume.
    VolumeNotSupported,
    /// The snapshot provider refused the operation.
    ProviderVeto,
    /// The snapshot provider failed in an unexpected way.
    UnexpectedProviderError,
    /// The snapshot set already holds the maximum number of volumes.
    MaxVolumesReached,
    /// The system already holds the maximum number of shadow copies.
    MaxSnapshotsReached,
    /// Another snapshot is being created. Retryable.
    SnapshotSetInProgress,
    /// Writes could not be flushed or held in time. Retryable.
    WritesTimeout,
    /// There is not enough shadow-copy storage space.
    InsufficientStorage,
    /// A VSS writer failed with an error that may clear on retry. Retryable.
    WriterRetryable,
    /// A VSS writer failed with an error that will not clear on retry.
    WriterFailed,
    /// The VSS writer infrastructure is not working.
    WriterInfrastructure,
    /// VSS failed in an unexpected way. The Windows Application event log
    /// usually has the cause.
    Unexpected,
    /// Any other `HRESULT`.
    Other,
}

impl ErrorKind {
    fn from_hresult(code: HRESULT) -> Self {
        match code {
            E_ACCESSDENIED => ErrorKind::AccessDenied,
            E_INVALIDARG => ErrorKind::InvalidArgument,
            VSS_E_BAD_STATE => ErrorKind::BadState,
            VSS_E_OBJECT_NOT_FOUND => ErrorKind::ObjectNotFound,
            VSS_E_VOLUME_NOT_SUPPORTED | VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER => {
                ErrorKind::VolumeNotSupported
            }
            VSS_E_PROVIDER_VETO => ErrorKind::ProviderVeto,
            VSS_E_UNEXPECTED_PROVIDER_ERROR => ErrorKind::UnexpectedProviderError,
            VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED => ErrorKind::MaxVolumesReached,
            VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED => ErrorKind::MaxSnapshotsReached,
            VSS_E_SNAPSHOT_SET_IN_PROGRESS => ErrorKind::SnapshotSetInProgress,
            VSS_E_FLUSH_WRITES_TIMEOUT | VSS_E_HOLD_WRITES_TIMEOUT => ErrorKind::WritesTimeout,
            VSS_E_INSUFFICIENT_STORAGE => ErrorKind::InsufficientStorage,
            VSS_E_WRITERERROR_RETRYABLE => ErrorKind::WriterRetryable,
            VSS_E_WRITERERROR_NONRETRYABLE => ErrorKind::WriterFailed,
            VSS_E_WRITER_INFRASTRUCTURE => ErrorKind::WriterInfrastructure,
            VSS_E_UNEXPECTED => ErrorKind::Unexpected,
            _ => ErrorKind::Other,
        }
    }

    fn description(self) -> Option<&'static str> {
        Some(match self {
            ErrorKind::AccessDenied => {
                "access denied; VSS needs administrator rights or the backup privilege"
            }
            ErrorKind::InvalidArgument => "invalid argument",
            ErrorKind::BadState => "VSS calls were made in the wrong order",
            ErrorKind::ObjectNotFound => "snapshot not found",
            ErrorKind::VolumeNotSupported => "the volume does not support shadow copies",
            ErrorKind::ProviderVeto => "the snapshot provider refused the operation",
            ErrorKind::UnexpectedProviderError => "the snapshot provider failed unexpectedly",
            ErrorKind::MaxVolumesReached => "too many volumes in one snapshot set",
            ErrorKind::MaxSnapshotsReached => "the maximum number of shadow copies was reached",
            ErrorKind::SnapshotSetInProgress => "another snapshot is being created",
            ErrorKind::WritesTimeout => "writes could not be paused in time",
            ErrorKind::InsufficientStorage => "not enough shadow-copy storage space",
            ErrorKind::WriterRetryable => "a VSS writer failed with a transient error",
            ErrorKind::WriterFailed => "a VSS writer failed",
            ErrorKind::WriterInfrastructure => "the VSS writer infrastructure is not working",
            ErrorKind::Unexpected => "unexpected VSS error; see the Windows Application event log",
            ErrorKind::Other => return None,
        })
    }
}

/// Attaches the name of the failed call to a `windows_core` result or a raw
/// `HRESULT`.
#[cfg_attr(not(test), allow(dead_code))] // Used by the snapshot API.
pub(crate) trait Context<T> {
    fn context(self, operation: &'static str) -> Result<T>;
}

impl<T> Context<T> for windows_core::Result<T> {
    fn context(self, operation: &'static str) -> Result<T> {
        self.map_err(|err| Error::new(operation, err.code()))
    }
}

impl Context<()> for HRESULT {
    fn context(self, operation: &'static str) -> Result<()> {
        self.ok().context(operation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const S_FALSE: HRESULT = HRESULT(1);

    #[test]
    fn kind_names_well_known_codes() {
        assert_eq!(
            Error::new("Op", E_ACCESSDENIED).kind(),
            ErrorKind::AccessDenied
        );
        assert_eq!(
            Error::new("Op", VSS_E_HOLD_WRITES_TIMEOUT).kind(),
            ErrorKind::WritesTimeout
        );
        assert_eq!(
            Error::new("Op", VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER).kind(),
            ErrorKind::VolumeNotSupported
        );
        assert_eq!(
            Error::new("Op", HRESULT(0x8000_4005_u32 as i32)).kind(),
            ErrorKind::Other
        );
    }

    #[test]
    fn only_transient_failures_are_retryable() {
        assert!(Error::new("Op", VSS_E_SNAPSHOT_SET_IN_PROGRESS).is_retryable());
        assert!(Error::new("Op", VSS_E_FLUSH_WRITES_TIMEOUT).is_retryable());
        assert!(Error::new("Op", VSS_E_WRITERERROR_RETRYABLE).is_retryable());
        assert!(!Error::new("Op", VSS_E_WRITERERROR_NONRETRYABLE).is_retryable());
        assert!(!Error::new("Op", E_ACCESSDENIED).is_retryable());
    }

    #[test]
    fn display_names_the_call_and_the_code() {
        assert_eq!(
            Error::new("DoSnapshotSet", VSS_E_INSUFFICIENT_STORAGE).to_string(),
            "DoSnapshotSet failed: not enough shadow-copy storage space (0x8004231F)"
        );
        assert_eq!(
            Error::new("Query", HRESULT(0x8000_4005_u32 as i32)).to_string(),
            "Query failed with HRESULT 0x80004005"
        );
    }

    #[test]
    fn context_keeps_success_and_records_failures() {
        assert_eq!(S_FALSE.context("Query"), Ok(()));
        assert_eq!(
            VSS_E_BAD_STATE.context("AddToSnapshotSet"),
            Err(Error::new("AddToSnapshotSet", VSS_E_BAD_STATE))
        );
        let failed: windows_core::Result<u32> = Err(VSS_E_PROVIDER_VETO.into());
        assert_eq!(
            failed.context("DoSnapshotSet"),
            Err(Error::new("DoSnapshotSet", VSS_E_PROVIDER_VETO))
        );
    }
}
