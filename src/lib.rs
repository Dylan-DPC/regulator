#![deny(rust_2018_idioms)]

use crate::sigma::Sigma;
use hasheimer::{oom::OneOrMany, Hasheimer};
use std::ops::{BitAnd, BitOrAssign};
use std::{error::Error, fmt::Display};

pub mod sigma;

#[allow(clippy::type_complexity)]
pub struct Regulator<T, S>
where
    S: Sigma + BitOrAssign + BitAnd + Clone + Copy,
{
    funcs: Vec<Box<dyn Fn(&mut T)>>,
    exclude: Hasheimer<S, S>,
    sigma: S,
}

impl<T, S> Regulator<T, S>
where
    S: Sigma + BitOrAssign + BitAnd + Clone + Copy,
{
    #[must_use]
    #[allow(clippy::type_complexity)]
    pub fn new(funcs: Vec<Box<dyn Fn(&mut T)>>, exclude: Hasheimer<S, S>, sigma: S) -> Self {
        Self {
            funcs,
            exclude,
            sigma,
        }
    }

    pub fn regulate(&self, item: &mut T) -> Result<(), RegulatorError> {
        self.exclude
            .iter()
            .find(|(ex, ey)| {
                match ey {
                    OneOrMany::Single(ey) => {
                        self.sigma.check_conflict(ex, ey)
                        // self.sigma & (*ex | *ey) > S::default()
                    }
                    OneOrMany::Many(eys) => {
                        let ey = eys.iter().fold(S::default(), |mut excls, ey| {
                            excls |= *ey;
                            excls
                        });

                        self.sigma.check_conflict(ex, &ey)
                    }
                }
            })
            .map(|_| Result::<(), RegulatorError>::Err(RegulatorError::ConflictDetected))
            .transpose()?;

        let nob = self.sigma.bit_length() as usize;
        (0..=nob).rev().for_each(|bit| {
            if self.sigma.mask_one(bit) {
                // if self.sigma & (1 << bit) > S::default() {
                self.funcs[bit](item);
            }
        });
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum RegulatorError {
    ConflictDetected,
}

impl Display for RegulatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conflict detected")
    }
}

impl Error for RegulatorError {}

#[macro_export]
macro_rules! regulator {
    ($sigma: ident, $($item:ident),*) => {
        Regulator::new(vec![$(Box::new($item)),*], Hasheimer::default(), $sigma);
    };

    ($sigma: expr, $($item:expr),*, >| $($conflict_from: expr => $conflict_to: expr),*) => {
        {
        let mut hashmap = Hasheimer::default();
        $(hashmap.raw_insert($conflict_from, $conflict_to.into()));*;

        Regulator::new(vec![$(Box::new($item)),*], hashmap, $sigma)
        }
    };
}
