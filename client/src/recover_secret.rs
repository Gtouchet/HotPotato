use crate::messages::{RecoverSecretInput, RecoverSecretOutput};
use crate::challenge::Challenge;

pub struct RecoverSecret {
    input: RecoverSecretInput
}

impl RecoverSecret {
    pub fn swap(secret_sentence: &str, first_char_index: usize, second_char_index: usize) -> String {
        let mut chars: Vec<_> = secret_sentence.chars().collect();
        chars.swap(first_char_index, second_char_index);
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
        let mut have_changed = true;

        while have_changed
        {
            let mut last_index = 0;
            have_changed = false;

            for tuple_size in &self.input.tuple_sizes
            {
                for index in last_index..(*tuple_size + last_index)
                {
                    let char = &self.input.letters[index..=index];

                    if !secret_sentence.contains(char)
                    {
                        secret_sentence.push_str(char);
                    }
                    else if index != last_index
                    {
                        let previous_char = &self.input.letters[index - 1..=index - 1];

                        let first_char_index = secret_sentence.rfind(char).unwrap();
                        let second_char_index = secret_sentence.rfind(previous_char).unwrap();

                        if first_char_index < second_char_index
                        {
                            secret_sentence = RecoverSecret::swap(&secret_sentence, first_char_index, second_char_index);
                            have_changed = true;
                        }
                    }
                }
                last_index += *tuple_size;
            }
        }
        RecoverSecretOutput { secret_sentence }
    }

    fn verify(&self, answer: &Self::Output) -> bool
    {
        let mut last_index = 0;

        for tuple_size in &self.input.tuple_sizes
        {
            let mut char_indexes_in_answer : Vec<usize> = Vec::new();

            for index in last_index..(*tuple_size + last_index)
            {
                let char_at_index = self.input.letters.chars().nth(index).unwrap();
                char_indexes_in_answer.push(answer.secret_sentence.find(char_at_index).unwrap());
            }
            for i in 0..char_indexes_in_answer.len() - 1
            {
                if char_indexes_in_answer[i] > char_indexes_in_answer[i + 1]
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
    use crate::recover_secret::*;

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
