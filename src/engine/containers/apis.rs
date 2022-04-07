pub use crate::engine::containers::types;
pub use crate::common::errors;

pub trait ContainerApi {
    fn list_containers(&self) -> Result<Vec<types::Container>, errors::DockerError>;
}