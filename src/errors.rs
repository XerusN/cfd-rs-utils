use thiserror::Error;

#[derive(Clone, Debug, Default, Error, PartialEq)]
pub enum MeshError {
    #[default]
    #[error(
        "An Unspecified error happened, you can blame the crate developer for the lack of details"
    )]
    Unspecified,
    #[error("Value not in the right range (expected in {expected:?}, got {got:?}")]
    WrongFloatValue { got: f64, expected: (f64, f64) },
}
