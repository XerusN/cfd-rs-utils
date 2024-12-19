#[derive(Clone, Debug, Default)]
pub enum Boundary {
    #[default]
    NoSlip,
    Slip,
}