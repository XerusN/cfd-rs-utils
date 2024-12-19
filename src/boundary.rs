#[derive(Clone, Debug, Default)]
pub enum Boundary {
    #[default]
    None,
    NoSlip,
    Slip,
}
