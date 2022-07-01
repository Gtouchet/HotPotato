use rand::Rng;
use rand::rngs::ThreadRng;

pub struct Random
{
    pub(crate) random: ThreadRng,
}

impl Random
{
    pub(crate) fn generate_name(&mut self) -> String
    {
        let num = self.random.gen_range(0..10_000);
        num.to_string()
    }
}
