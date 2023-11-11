use crate::models::generation_base::GenerationBase;

mod interfaces;
mod models;

fn main() {
    let generation_base = GenerationBase {
        base_value: 10,
        dices: vec![],
    };
    println!("Hello, world! {:?}", generation_base);
}
