use serde::Deserialize;

/// `DockerError` enum.
#[derive(Debug)]
pub enum DockerError {
    /// Bad parameters (HTTP status is 401)
    BadParameters(ErrorMessage), // 401

    /// Server error (HTTP status is 500)
    ServerError(ErrorMessage), // 500

    /// Server error (HTTP status is 404)
    NotFound(ErrorMessage), // 404

    /// Server error (HTTP status is 409)
    NotRunning(ErrorMessage), // 409

    /// Server error (HTTP status is 304)
    AlreadyStarted(ErrorMessage), // 304

    /// Server error (HTTP status is 409)
    ContainerExists(ErrorMessage), // 409

    /// Busy by container (HTTP status is 409)
    Busy(ErrorMessage), // 409

    /// Unknown status
    UnknownStatus(ErrorMessage),

    /// Closed connection
    ClosedConnection(ErrorMessage),
}


/// `ErrorMessage` struct.
#[derive(Deserialize, Debug, Clone)]
pub struct ErrorMessage {
    /// Error message get from response.
    pub message: String,
}

impl ErrorMessage {
    pub fn map_err_hyper(e: hyper::Error) -> self::DockerError {
        self::DockerError::ClosedConnection(self::ErrorMessage {
            message: e.message().to_string()
        })
    }

    pub fn map_err_serde_json(e: serde_json::Error) -> self::DockerError {
        self::DockerError::ClosedConnection(self::ErrorMessage {
            message: e.to_string()
        })
    }
}
