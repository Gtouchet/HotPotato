use crate::messages::{Md5ResolverInput, Md5ResolverOutput};
use crate::recoversecret::Challenge;

pub struct Md5Resolver
{
    input: Md5ResolverInput
}

impl Challenge for Md5Resolver
{
    type Input = Md5ResolverInput;
    type Output = Md5ResolverOutput;

    fn name() -> String {
        "MD5HashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        Md5Resolver {
            input,
        }
    }

    fn solve(&self) -> Self::Output {
        let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
        Md5ResolverOutput { secret_sentence: "".to_string() }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}