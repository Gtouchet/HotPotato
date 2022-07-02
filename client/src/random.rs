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
        self.random.gen_range(0..10_000).to_string()
    }

    pub(crate) fn get_number(&mut self, min: usize, max: usize) -> usize
    {
        self.random.gen_range(min..=max)
    }
}
