use std::hash::Hash;

pub trait Sigma: Eq + Hash + Default {
    fn bit_length(&self) -> u32;
    fn check_conflict(&self, ex: &Self, ey: &Self) -> bool;
    fn mask_one(&self, bit: usize) -> bool;
}

impl Sigma for u8 {
    fn bit_length(&self) -> u32 {
        self.ilog2()
    }

    fn check_conflict(&self, ex: &Self, ey: &Self) -> bool {
        self & (*ex | *ey) > 0
    }

    fn mask_one(&self, bit: usize) -> bool {
        self & (1 << bit) > 0
    }
}
