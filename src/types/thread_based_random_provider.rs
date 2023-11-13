use rand::Rng;

pub trait RandomProvider {
    fn generate(&self, base: &ThreadBasedRandomProvider) -> u32;
}

pub struct ThreadBasedRandomProvider {
    pub base_value: u32,
    pub dices: Vec<[u32; 2]>,
}

impl RandomProvider for ThreadBasedRandomProvider {
    fn generate(&self, _base: &ThreadBasedRandomProvider) -> u32 {
        let mut rng = rand::thread_rng();
        let mut result = self.base_value;
        for &dice in &self.dices {
            let roll = rng.gen_range(dice[0]..=dice[1]);
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
        let provider = ThreadBasedRandomProvider {
            base_value: 10,
            dices: vec![],
        };

        let result = provider.generate(&provider);

        assert_eq!(result, 10);
    }

    #[test]
    fn test_generate_with_single_dice() {
        let provider = ThreadBasedRandomProvider {
            base_value: 5,
            dices: vec![[1, 6]],
        };

        let result = provider.generate(&provider);

        assert!(result >= 6 && result <= 11);
    }

    #[test]
    fn test_generate_with_multiple_dices() {
        let provider = ThreadBasedRandomProvider {
            base_value: 0,
            dices: vec![[1, 6], [1, 8], [2, 10]],
        };

        let result = provider.generate(&provider);

        assert!(result >= 3 && result <= 24);
    }
}