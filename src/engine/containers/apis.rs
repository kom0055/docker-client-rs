pub use crate::engine::containers::types;
pub use crate::common::errors;

pub trait ContainerApi {
    fn list_containers(&self, req: types::ListContainerRequest) -> Result<Vec<types::Container>, errors::DockerError>;
}