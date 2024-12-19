use thiserror::Error;

#[derive(Clone, Debug, Default, Error, PartialEq)]
pub enum MeshError {
    #[default]
    #[error(
        "An Unspecified error happened, you can blame the crate developer for the lack of details"
    )]
    Unspecified,
}
