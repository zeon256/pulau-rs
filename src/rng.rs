use rand_core::RngCore;

/// Zero-sized marker type for RNG-less heuristics
#[derive(Default, Debug)]
pub struct PhantomRng;

impl AsMut<Self> for PhantomRng {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl AsRef<Self> for PhantomRng {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl RngCore for PhantomRng {
    fn next_u32(&mut self) -> u32 {
        unreachable!("Should not be called!")
    }

    fn next_u64(&mut self) -> u64 {
        unreachable!("Should not be called!")
    }

    fn fill_bytes(&mut self, _dst: &mut [u8]) {
        unreachable!("Should not be called!")
    }
}
