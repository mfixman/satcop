use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

pub(crate) struct DefaultRng(SmallRng);

impl DefaultRng {
    #[inline]
    pub(crate) fn seed(&mut self, seed: u64) {
        self.0 = SmallRng::seed_from_u64(seed);
    }

    #[inline]
    pub(crate) fn index(&mut self, max: usize) -> usize {
        self.0.gen_range(0..max)
    }

    #[inline]
    pub(crate) fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        slice.choose(&mut self.0)
    }

    #[inline]
    pub(crate) fn shuffle<T>(&mut self, slice: &mut [T]) {
        slice.shuffle(&mut self.0);
    }

    // Efraimidis-Spirakis: ascending Exp(1) / weight is Plackett-Luce order.
    pub(crate) fn weighted_shuffle<T: Copy>(
        &mut self,
        slice: &mut [T],
        weight: impl Fn(T) -> f64,
    ) {
        let mut keyed = slice
            .iter()
            .map(|item| {
                (-(1.0 - self.0.gen::<f64>()).ln() / weight(*item), *item)
            })
            .collect::<Vec<_>>();
        keyed.sort_by(|left, right| left.0.total_cmp(&right.0));
        for (slot, (_, item)) in slice.iter_mut().zip(keyed) {
            *slot = item;
        }
    }
}

impl Default for DefaultRng {
    fn default() -> Self {
        Self(SmallRng::seed_from_u64(0))
    }
}
