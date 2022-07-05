use crate::messages::{MD5HashCashInput, MD5HashCashOutput};
use crate::recoversecret::Challenge;

pub struct Md5Resolver
{
    input: MD5HashCashInput
}

impl Challenge for Md5Resolver
{
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "MD5HashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        Md5Resolver {
            input,
        }
    }

    fn solve(&self) -> Self::Output {
        let digest = md5::compute(self.input.message.as_bytes());
        let mut digest_string = String::new();
        for byte in digest.iter()
        {
            digest_string.push_str(&format!("{:x}", byte));
        }
        MD5HashCashOutput {
            seed: get_seed(self.input.complexity),
            hashcode: digest_string,
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}

fn get_seed(complexity: u32) -> u64 {
    format!("{:x}", complexity).parse::<u64>().unwrap()
}