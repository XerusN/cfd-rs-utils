#[derive(Clone, Debug, Default, PartialEq)]
pub enum Boundary {
    #[default]
    None,
    NoSlip,
    Slip,
}
