use crate::challenge::Challenge;
use crate::messages::{MD5HashCashInput, MD5HashCashOutput};
use std::time::{Duration, Instant};

pub struct Md5Resolver {
    input: MD5HashCashInput,
}

impl Challenge for Md5Resolver {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "MD5HashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        Md5Resolver { input }
    }

    fn solve(&self) -> Self::Output {
        let mut seed = 0;

        let start = Instant::now();
        let mut digest;
        loop {
            digest = md5::compute(format!("{:016X}{}", seed, self.input.message));

            let string = format!("{:032X}", digest);
            let as_int = u128::from_str_radix(&string, 16);

            if as_int.unwrap().leading_zeros() >= self.input.complexity {
                break;
            }

            seed += 1;
            if start.elapsed() > Duration::from_millis(1_500) {
                println!("\n\n\n\nTOOK TOO LONG\n\n\n\n");
                break;
            }
        }

        let digest_str = format!("{:32X}", digest);

        MD5HashCashOutput {
            seed,
            hashcode: digest_str,
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge::Challenge;
    use crate::md5_resolver::Md5Resolver;
    use crate::messages::MD5HashCashInput;

    #[test]
    fn test() {
        let input = MD5HashCashInput {
            complexity: 9,
            message: "hello".to_string(),
        };
        let challenge = Md5Resolver::new(input);
        let output = challenge.solve();

        assert_eq!(output.hashcode, "00441745D9BDF8E5D3C7872AC9DBB2C3");
        assert_eq!(output.seed, 0x000000000000034C);
    }
}
