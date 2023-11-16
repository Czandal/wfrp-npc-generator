use rand::Rng;
use crate::models::generation_base::GenerationBase;
use crate::interfaces::random_provider::RandomProvider;

pub struct ThreadBasedRandomProvider {}

impl RandomProvider for ThreadBasedRandomProvider {
    fn generate(&self, base: &GenerationBase) -> u32 {
        let mut rng = rand::thread_rng();
        let mut result = base.base_value;
        for dice in base.dices.iter() {
            let roll = rng.gen_range(1..=*dice);
            result += roll;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_with_zero_dices() {
        let provider = ThreadBasedRandomProvider {};
        let values = GenerationBase{base_value: 10, dices: vec![]};
        let result = provider.generate(&values);

        assert_eq!(result, 10);
    }

    #[test]
    fn test_generate_with_single_dice() {
        let provider = ThreadBasedRandomProvider {};
        let values = GenerationBase{base_value: 5, dices: vec![6]};
        let result = provider.generate(&values);

        assert!(result >= 6 && result <= 11);
    }

    #[test]
    fn test_generate_with_multiple_dices() {
        let provider = ThreadBasedRandomProvider {};
        let values = GenerationBase{base_value: 1, dices: vec![7,8,9]};
        let result = provider.generate(&values);

        assert!(result >= 4 && result <= 25);
    }
}