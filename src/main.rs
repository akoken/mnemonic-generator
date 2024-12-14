use mnemonic_generator::MnemonicGenerator;

fn main() {
    let mnemonic = MnemonicGenerator::new();
    println!("Mnemonic:{}", mnemonic.generate().unwrap());
}
