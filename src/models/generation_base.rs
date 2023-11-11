// Representation of N + kDM
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct GenerationBase {
    pub base_value: u32,
    pub dices: Vec<u32>
}
