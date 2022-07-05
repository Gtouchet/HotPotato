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
            hashcode: digest_string,
            seed: 0,
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}

fn get_seed(complexity: u32) -> u64 {
    let mut seed = 0;
    for _ in 0..complexity
    {
        seed = seed * 10 + rand::random::<u64>() % 10;
    }
    seed
}

#[cfg(test)]
mod tests {
    use crate::md5_resolver::Md5Resolver;
    use crate::messages::MD5HashCashInput;
    use crate::recoversecret::Challenge;

    #[test]
    fn test() {
        let input = MD5HashCashInput {
            complexity: 9,
            message: "Hello".to_string(),
        };
        let challenge = Md5Resolver::new(input);
        let output = challenge.solve();

        assert_eq!(output.hashcode, "00441745D9BDF8E5D3C7872AC9DBB2C3");
        assert_eq!(output.seed.to_string(), "000000000000034C".to_string());
    }
}
