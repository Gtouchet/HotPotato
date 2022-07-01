use crate::messages::{RecoverSecretInput, RecoverSecretOutput};

pub(crate) trait Challenge {
    /// Données en entrée du challenge
    type Input;
    /// Données en sortie du challenge
    type Output;
    /// Nom du challenge
    fn name() -> String;
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self;
    /// Résout le challenge
    fn solve(&self) -> Self::Output;
    /// Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: &Self::Output) -> bool;
}

pub struct RecoverSecret {
    input: RecoverSecretInput
}

impl RecoverSecret {
    pub fn swap(secret_sentence: &str, first_index: usize, second_index: usize) -> String {
        let mut chars: Vec<_> = secret_sentence.chars().collect();
        chars.swap(first_index, second_index);
        chars.into_iter().collect()
    }
}

impl Challenge for RecoverSecret {
    type Input = RecoverSecretInput;
    type Output = RecoverSecretOutput;

    fn name() -> String {
        "Recover Secret".to_string()
    }

    fn new(input: Self::Input) -> Self {
        RecoverSecret { input }
    }

    fn solve(&self) -> Self::Output
    {
        let mut secret_sentence = String::new();
        let mut last_index = 0;

        for tuple_size in &self.input.tuple_sizes
        {
            for index in last_index..(*tuple_size + last_index)
            {
                let char = &self.input.letters[index..=index];

                if !secret_sentence.contains(char) {
                    secret_sentence.push_str(char);
                }
                else if index != last_index
                {
                    let first_index = secret_sentence.rfind(char).unwrap();
                    let second_index = secret_sentence.rfind(&self.input.letters[index-1..=index-1]).unwrap();

                    if first_index < second_index
                    {
                        secret_sentence = RecoverSecret::swap(&secret_sentence, first_index, second_index);
                    }
                }
            }
            last_index += *tuple_size;
        }
        RecoverSecretOutput { secret_sentence }
    }

    fn verify(&self, answer: &Self::Output) -> bool
    {
        let mut last_index = 0;

        for tuple_size in &self.input.tuple_sizes
        {
            let mut tuple_indexes : Vec<usize> = Vec::new();

            for index in last_index..(*tuple_size + last_index)
            {
                let c = self.input.letters.chars().nth(index).unwrap();
                tuple_indexes.push( answer.secret_sentence.find(c).unwrap());
            }
            for i in 0..tuple_indexes.len() - 1
            {
                if tuple_indexes[i] > tuple_indexes[i + 1]
                {
                    return false;
                }
            }
            last_index += *tuple_size;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::recoversecret::*;

    #[test]
    fn it_works() {
        let recover_secret: RecoverSecret = Challenge::new(RecoverSecretInput {
            word_count: 1,
            letters: "abcabede".to_string(),
            tuple_sizes: vec![3, 3, 2],
        });

        let answer = recover_secret.solve();
        assert_eq!(recover_secret.verify(&answer), true);
        assert_eq!(answer.secret_sentence, "abcde");
    }
}
