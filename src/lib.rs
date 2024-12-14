use rand::Rng;
use thiserror::Error;

/// A generator for creating memorable word combinations from predefined or custom word lists.
///
/// # Examples
///
/// ```
/// use mnemonic_generator::MnemonicGenerator;
///
/// // Use the default generator
/// let generator = MnemonicGenerator::new();
/// let mnemonic = generator.generate().expect("Failed to generate mnemonic");
/// println!("Generated mnemonic: {}", mnemonic);
///
/// // Create a generator with custom words
/// let custom_generator = MnemonicGenerator::with_words(
///     vec!["amazing".to_string(), "friendly".to_string(), "epic".to_string()],
///     vec!["einstein".to_string(), "galileo".to_string()]
/// );
/// let custom_mnemonic = custom_generator.generate_with_separator("-")
///     .expect("Failed to generate custom mnemonic");
/// println!("Custom mnemonic: {}", custom_mnemonic);
/// ```
pub struct MnemonicGenerator {
    left_words: Vec<String>,
    right_words: Vec<String>,
}

/// Errors that can occur during mnemonic generation
#[derive(Error, Debug)]
pub enum MnemonicError {
    #[error("No words available for generation")]
    EmptyWordList,
}

/// Creates a new `MnemonicGenerator` with a default set of words.
///
/// # Examples
///
/// ```
/// use mnemonic_generator::MnemonicGenerator;
///
/// let generator = MnemonicGenerator::new();
/// let mnemonic = generator.generate().expect("Failed to generate mnemonic");
/// ```
impl MnemonicGenerator {
    /// Create a new MnemonicGenerator with default words
    pub fn new() -> Self {
        Self {
            left_words: vec!["crazy".to_string(), "amazing".to_string()],
            right_words: vec![
                "steve".to_string(),
                "alan".to_string(),
                "einstein".to_string(),
            ],
        }
    }

    /// Creates a `MnemonicGenerator` with custom word lists.
    ///
    /// # Arguments
    ///
    /// * `left_words` - A vector of words to be used as the first part of the mnemonic
    /// * `right_words` - A vector of words to be used as the second part of the mnemonic
    ///
    /// # Examples
    ///
    /// ```
    /// use mnemonic_generator::MnemonicGenerator;
    ///
    /// let generator = MnemonicGenerator::with_words(
    ///     vec!["hello".to_string(), "world".to_string()],
    ///     vec!["rust".to_string(), "programmer".to_string()]
    /// );
    /// let mnemonic = generator.generate().expect("Failed to generate mnemonic");
    /// ```
    pub fn with_words(left_words: Vec<String>, right_words: Vec<String>) -> Self {
        Self {
            left_words,
            right_words,
        }
    }

    /// Generates a mnemonic using the default underscore separator.
    ///
    /// # Errors
    ///
    /// Returns `MnemonicError::EmptyWordList` if no words are available for generation
    ///
    /// # Examples
    ///
    /// ```
    /// use mnemonic_generator::MnemonicGenerator;
    ///
    /// let generator = MnemonicGenerator::new();
    /// match generator.generate() {
    ///     Ok(mnemonic) => println!("Generated mnemonic: {}", mnemonic),
    ///     Err(e) => eprintln!("Error generating mnemonic: {}", e)
    /// }
    /// ```
    pub fn generate(&self) -> Result<String, MnemonicError> {
        self.generate_with_separator("_")
    }

    /// Generates a mnemonic using a custom separator.
    ///
    /// # Arguments
    ///
    /// * `separator` - A string slice to be used between the two words
    ///
    /// # Errors
    ///
    /// Returns `MnemonicError::EmptyWordList` if no words are available for generation
    ///
    /// # Examples
    ///
    /// ```
    /// use mnemonic_generator::MnemonicGenerator;
    ///
    /// let generator = MnemonicGenerator::new();
    /// match generator.generate_with_separator("-") {
    ///     Ok(mnemonic) => println!("Generated mnemonic: {}", mnemonic),
    ///     Err(e) => eprintln!("Error generating mnemonic: {}", e)
    /// }
    /// ```
    pub fn generate_with_separator(&self, separator: &str) -> Result<String, MnemonicError> {
        if self.left_words.is_empty() || self.right_words.is_empty() {
            return Err(MnemonicError::EmptyWordList);
        }

        let mut rng = rand::thread_rng();
        let left_idx = rng.gen_range(0..self.left_words.len());
        let right_idx = rng.gen_range(0..self.right_words.len());

        Ok(format!(
            "{}{}{}",
            &self.left_words[left_idx], separator, &self.right_words[right_idx]
        ))
    }
}

impl Default for MnemonicGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_default_mnemonic() {
        let generator = MnemonicGenerator::new();
        let mnemonic = generator.generate().expect("Should generate mnemonic");
        let parts: Vec<&str> = mnemonic.split('_').collect();
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn generate_custom_separator_mnemonic() {
        let generator = MnemonicGenerator::new();
        let mnemonic = generator
            .generate_with_separator("-")
            .expect("Should generate mnemonic with custom separator");
        let parts: Vec<&str> = mnemonic.split('-').collect();
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn generate_with_custom_words() {
        let custom_left = vec!["hello".to_string(), "world".to_string()];
        let custom_right = vec!["rust".to_string(), "programmer".to_string()];
        let generator = MnemonicGenerator::with_words(custom_left.clone(), custom_right.clone());

        let mnemonic = generator.generate().expect("Should generate mnemonic");
        let parts: Vec<&str> = mnemonic.split('_').collect();

        assert!(custom_left.contains(&parts[0].to_string()));
        assert!(custom_right.contains(&parts[1].to_string()));
    }

    #[test]
    fn error_on_empty_word_list() {
        let generator = MnemonicGenerator::with_words(vec![], vec![]);
        let result = generator.generate();
        assert!(result.is_err());
        assert!(matches!(result, Err(MnemonicError::EmptyWordList)));
    }
}
