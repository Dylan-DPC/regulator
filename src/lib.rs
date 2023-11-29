#![deny(rust_2018_idioms)]

use std::{fmt::Display, error::Error};

use hasheimer::{Hasheimer, oom::OneOrMany};

pub struct Regulator<T> {
    funcs: Vec<Box<dyn Fn(&mut T)>>,
    exclude: Hasheimer<u8, u8>,
    sigma: u8,
}

impl<T> Regulator<T> {
    pub fn new(funcs: Vec<Box<dyn Fn(&mut T)>>, exclude: Hasheimer<u8, u8>, sigma: u8) -> Self {
        Self {
            funcs,
            exclude,
            sigma,
        }
    }

    

    pub fn regulate(&self, item: &mut T) -> Result<(), RegulatorError> {
        self.exclude.iter().find(|(ex, ey)| {
            match ey {
                OneOrMany::Single(ey) => {
                    self.sigma & (*ex | *ey) > 0
                },
                OneOrMany::Many(eys) => {
                    let ey = eys.iter().fold(0, |mut excls, ey| {
                        excls |= ey;
                        excls
                    });

                    self.sigma & (*ex | ey) > 0
                },
            }
        }).map(|_| {
            return Result::<(), RegulatorError>::Err(RegulatorError::ConflictDetected)
        });

        let nob = self.sigma.ilog2();
        (0..=nob).rev().for_each(|bit| {
            let m = self.sigma & dbg!(1 << bit) > 0;
            if m == true {
                self.funcs[bit as usize](item);
            }
        });
        Ok(())
    }


}

#[derive(Clone, Debug)]
pub enum RegulatorError {
    ConflictDetected
}

impl Display for RegulatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conflict detected")  
    }
}

impl Error for RegulatorError {}
        

#[macro_export]
macro_rules! regulate {
    ($sigma: ident, $($item:ident),*) => {
        Regulator::new(vec![$(Box::new($item)),*], Hasheimer::default(), $sigma);
    }
}
