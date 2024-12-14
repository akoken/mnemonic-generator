use rand::Rng;

pub struct MnemonicGenerator {}

impl MnemonicGenerator {
    pub fn generate() -> String {
        let left = ["crazy", "amazing"];
        let right = ["steve", "alan", "einstein"];

        let left_idx = rand::thread_rng().gen_range(0..left.len());
        let right_idx = rand::thread_rng().gen_range(0..right.len());

        format!("{}_{}", &left[left_idx], &right[right_idx])
    }
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_two_words() {
        let mnemonic = MnemonicGenerator::generate();
        let parts: Vec<&str> = mnemonic.split('_').collect();
        assert_eq!(parts.len(), 2);
    }
}
