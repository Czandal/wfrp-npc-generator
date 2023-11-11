use crate::models::generation_base::GenerationBase;

pub trait RandomProvider {
    fn generate(&self, base: &GenerationBase) -> u32;
}
