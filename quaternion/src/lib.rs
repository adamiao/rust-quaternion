use std::fmt;
mod utils;

#[derive(Debug)]
pub struct Quaternion {
    s: f32,
    i: f32,
    j: f32,
    k: f32,
}

impl Quaternion {

    // Creation of new `Quaternion` instance (origin).
    pub fn origin() -> Quaternion {
        Quaternion {
            s: 0f32,
            i: 0f32,
            j: 0f32,
            k: 0f32,
        }
    }

    // Creation of identity instance of `Quaternion`.
    pub fn one() -> Quaternion {
        Quaternion {
            s: 1f32,
            i: 0f32,
            j: 0f32,
            k: 0f32,
        }
    }

    // Creation of new instance of `Quaternion` by just feeding coordinates.
    pub fn new(s: f32, i: f32, j: f32, k:f32) -> Quaternion {
        Quaternion {
            s,
            i,
            j,
            k,
        }
    }

    // Creation of a `Quaternion` instance from a vector slice.
    pub fn from_vec(point: &[f32]) -> Quaternion {
        Quaternion {
            s: point[0],
            i: point[1],
            j: point[2],
            k: point[3],
        }
    }

    // Here we implement the magnitude/absolute value of the `Quaternion` struct.
    pub fn abs(&self) -> f32 {
        (self.s.powf(2.)+self.i.powf(2.)+self.j.powf(2.)+self.k.powf(2.)).powf(0.5)
    }

    // Here we implement the conjugation operation for the `Quaternion` struct.
    pub fn conj(&self) -> Quaternion {
        Quaternion {
            s: self.s,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }

    // Here we implement the conjugation operation for the `Quaternion` struct.
    pub fn inv(&self) -> Quaternion {
        if self.abs() == 0f32 {
            panic!("Can't divide by the zero quaternion!");
        }
        self.conj()/self.abs().powf(2.)
    }
}

// Here the `Display` trait is defined for the `Quaternion` struct.
impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {:>+width$.prec$} {:>+width$.prec$} i\
                    {:>+width$.prec$} j {:>+width$.prec$} k ",
               self.s, self.i, self.j, self.k,
               width=12, prec=4)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
