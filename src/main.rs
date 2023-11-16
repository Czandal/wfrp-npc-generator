use crate::models::generation_base::GenerationBase;
use crate::interfaces::random_provider::RandomProvider;

mod interfaces;
mod models;
mod types;

fn main() {
    let generation_base = GenerationBase {
        base_value: 10,
        dices: vec![],
    };
    println!("Hello, world! {:?}", generation_base);
}
