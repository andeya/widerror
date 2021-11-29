use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct WidError {
    /// error message
    pub message: Message,
    /// error code
    /// suggest digits [100000000, 999999999]
    pub code: u32,
    /// error name
    pub name: String,
    /// suggest as a prefix of code, 5 digits [10000, 99999]
    pub namespace: u32,
    pub kind: Kind,
    pub scope: Scope,
    /// error level [0, 255]
    /// via: https://www.yisu.com/zixun/404282.html
    pub level: u8,
    pub retry_mode: RetryMode,
    pub pass_through_mode: PassThroughMode,
    pub mapping_code: i64,
    source_error: Option<Box<WidError>>,
}

impl WidError {
    pub fn new(code: u32, message: Message) -> WidError {
        WidError {
            code,
            message,
            ..WidError::default()
        }
    }
    pub fn set_source(mut self, e: WidError) -> WidError {
        self.source_error = Some(Box::new(e));
        self
    }
}

impl Display for WidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "code={}, name={}, namespace={}, scope={}, kind={}, level={}, message={}, retry_mode={}, pass_through_mode={}, mapping_code={}, source_error=({})",
               self.code, self.name, self.namespace, self.scope, self.kind, self.level, self.message, self.retry_mode, self.pass_through_mode, self.mapping_code,
               if self.source_error.is_some() { format!("{}", self.source_error.as_ref().unwrap()) } else { String::new() }
        )
    }
}

impl Error for WidError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(ref e) = self.source_error {
            Some(e as &dyn Error)
        } else {
            None
        }
    }
}

/// Refer to the authoritative error code of gRPC APIs.
/// https:///github.com/googleapis/googleapis/blob/master/google/rpc/code.proto
/// https://cloud.google.com/apis/design/errors
///
/// Sometimes multiple error codes may apply.  Services should return
/// the most specific error code that applies.  For example, prefer
/// `OutOfRange` over `FailedPrecondition` if both codes apply.
/// Similarly prefer `NotFound` or `AlreadyExists` over `FailedPrecondition`.
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Copy, Clone)]
#[repr(i8)]
pub enum Kind {
    /// Not an error; returned on success
    ///
    /// HTTP Mapping: 200 Ok
    Ok = 0,

    /// The operation was cancelled, typically by the caller.
    ///
    /// HTTP Mapping: 499 Client Closed Request
    Cancelled = 1,

    /// Unknown error.  For example, this error may be returned when
    /// a `Status` value received from another address space belongs to
    /// an error space that is not known in this address space.  Also
    /// errors raised by APIs that do not return enough error information
    /// may be converted to this error.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    Unknown = 2,

    /// The client specified an invalid argument.  Note that this differs
    /// from `FailedPrecondition`.  `InvalidArgument` indicates arguments
    /// that are problematic regardless of the state of the system
    /// (e.g., a malformed file name).
    ///
    /// HTTP Mapping: 400 Bad Request
    InvalidArgument = 3,

    /// The deadline expired before the operation could complete. For operations
    /// that change the state of the system, this error may be returned
    /// even if the operation has completed successfully.  For example, a
    /// successful response from a server could have been delayed long
    /// enough for the deadline to expire.
    ///
    /// HTTP Mapping: 504 Gateway Timeout
    DeadlineExceeded = 4,

    /// Some requested entity (e.g., file or directory) was not found.
    ///
    /// Note to server developers: if a request is denied for an entire class
    /// of users, such as gradual feature rollout or undocumented whitelist,
    /// `NotFound` may be used. If a request is denied for some users within
    /// a class of users, such as user-based access control, `PermissionDenied`
    /// must be used.
    ///
    /// HTTP Mapping: 404 Not Found
    NotFound = 5,

    /// The entity that a client attempted to create (e.g., file or directory)
    /// already exists.
    ///
    /// HTTP Mapping: 409 Conflict
    AlreadyExists = 6,

    /// The caller does not have permission to execute the specified
    /// operation. `PermissionDenied` must not be used for rejections
    /// caused by exhausting some resource (use `ResourceExhausted`
    /// instead for those errors). `PermissionDenied` must not be
    /// used if the caller can not be identified (use `Unauthenticated`
    /// instead for those errors). This error code does not imply the
    /// request is valid or the requested entity exists or satisfies
    /// other pre-conditions.
    ///
    /// HTTP Mapping: 403 Forbidden
    PermissionDenied = 7,

    /// The request does not have valid authentication credentials for the
    /// operation.
    ///
    /// HTTP Mapping: 401 Unauthorized
    Unauthenticated = 16,

    /// Some resource has been exhausted, perhaps a per-user quota, or
    /// perhaps the entire file system is out of space.
    ///
    /// HTTP Mapping: 429 Too Many Requests
    ResourceExhausted = 8,

    /// The operation was rejected because the system is not in a state
    /// required for the operation's execution.  For example, the directory
    /// to be deleted is non-empty, an rmdir operation is applied to
    /// a non-directory, etc.
    ///
    /// Service implementors can use the following guidelines to decide
    /// between `FailedPrecondition`, `Aborted`, and `Unavailable`:
    ///  (a) Use `Unavailable` if the client can retry just the failing call.
    ///  (b) Use `Aborted` if the client should retry at a higher level
    ///      (e.g., when a client-specified test-and-set fails, indicating the
    ///      client should restart a read-modify-write sequence).
    ///  (c) Use `FailedPrecondition` if the client should not retry until
    ///      the system state has been explicitly fixed.  E.g., if an "rmdir"
    ///      fails because the directory is non-empty, `FailedPrecondition`
    ///      should be returned since the client should not retry unless
    ///      the files are deleted from the directory.
    ///
    /// HTTP Mapping: 400 Bad Request
    FailedPrecondition = 9,

    /// The operation was Aborted, typically due to a concurrency issue such as
    /// a sequencer check failure or transaction abort.
    ///
    /// See the guidelines above for deciding between `FailedPrecondition`,
    /// `Aborted`, and `Unavailable`.
    ///
    /// HTTP Mapping: 409 Conflict
    Aborted = 10,

    /// The operation was attempted past the valid range.  E.g., seeking or
    /// reading past end-of-file.
    ///
    /// Unlike `InvalidArgument`, this error indicates a problem that may
    /// be fixed if the system state changes. For example, a 32-bit file
    /// system will generate `InvalidArgument` if asked to read at an
    /// offset that is not in the range [0,2^32-1], but it will generate
    /// `OutOfRange` if asked to read from an offset past the current
    /// file size.
    ///
    /// There is a fair bit of overlap between `FailedPrecondition` and
    /// `OutOfRange`.  We recommend using `OutOfRange` (the more specific
    /// error) when it applies so that callers who are iterating through
    /// a space can easily look for an `OutOfRange` error to detect when
    /// they are done.
    ///
    /// HTTP Mapping: 400 Bad Request
    OutOfRange = 11,

    /// The operation is not implemented or is not supported/enabled in this
    /// service.
    ///
    /// HTTP Mapping: 501 Not Implemented
    Unimplemented = 12,

    /// Internal errors.  This means that some invariants expected by the
    /// underlying system have been broken.  This error code is reserved
    /// for serious errors.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    Internal = 13,

    /// The service is currently Unavailable.  This is most likely a
    /// transient condition, which can be corrected by retrying with
    /// a backoff. Note that it is not always safe to retry
    /// non-idempotent operations.
    ///
    /// See the guidelines above for deciding between `FailedPrecondition`,
    /// `Aborted`, and `Unavailable`.
    ///
    /// HTTP Mapping: 503 Service Unavailable
    Unavailable = 14,

    /// Unrecoverable data loss or corruption.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    DataLoss = 15,
}

impl Default for Kind {
    fn default() -> Self {
        Self::Ok
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone() as i8)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Default(String),
    I18n(String),
}

impl Default for Message {
    fn default() -> Self {
        Self::Default(String::new())
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Copy, Clone)]
#[repr(i8)]
pub enum Scope {
    Internal = 0,
    Clientside = 1,
    Serverside = 2,
}

impl Default for Scope {
    fn default() -> Self {
        Self::Internal
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone() as i8)
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Copy, Clone)]
#[repr(i8)]
pub enum RetryMode {
    Unknown = 0,
    Allowed = 1,
    Denied = 2,
}

impl Default for RetryMode {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for RetryMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone() as i8)
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Copy, Clone)]
#[repr(i8)]
pub enum PassThroughMode {
    Auto = 0,
    Should = 1,
    Never = 2,
}

impl Default for PassThroughMode {
    fn default() -> Self {
        Self::Auto
    }
}

impl Display for PassThroughMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone() as i8)
    }
}

#[test]
fn max_digits() {
    println!("u8::MAX {}", u8::MAX);
    println!("u16::MAX {}", u16::MAX);
    println!("u32::MAX {}", u32::MAX);
    println!("i32::MAX {}", i32::MAX);
    println!("i16::MAX {}", i16::MAX);
}
