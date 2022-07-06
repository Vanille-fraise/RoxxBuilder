#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum DamageCalculation {
    Minimized,
    Min,
    Average,
    Max,
}