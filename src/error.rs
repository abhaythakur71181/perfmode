use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sysfs node not found: {path}")]
    NodeNotFound { path: PathBuf },

    #[error("permission denied: {path} (try running with sudo)")]
    PermissionDenied { path: PathBuf },

    #[error("failed to read sysfs node {path}: {source}")]
    ReadFailed {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to write to sysfs node {path}: {source}")]
    WriteFailed {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("unexpected value '{value}' read from {path}")]
    UnexpectedValue { path: PathBuf, value: String },

    #[error("no supported ASUS platform driver detected (checked asus-nb-wmi and faustus)")]
    NoDriverDetected,

    #[error("operation '{operation}' is not valid for '{subsystem}'")]
    #[allow(dead_code)]
    InvalidOperation {
        subsystem: String,
        operation: String,
    },

    #[error("battery sysfs node not found — charge limit control is not supported on this device")]
    BatteryNotSupported,
}

pub type Result<T> = std::result::Result<T, Error>;
