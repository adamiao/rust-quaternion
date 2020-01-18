use std::fmt;
mod utils;

#[derive(Debug)]
pub struct Quaternion {
    s: f64,
    i: f64,
    j: f64,
    k: f64,
}

impl Quaternion {

    // Creation of new `Quaternion` instance (origin).
    pub fn origin() -> Quaternion {
        Quaternion {
            s: 0f64,
            i: 0f64,
            j: 0f64,
            k: 0f64,
        }
    }

    // Creation of identity instance of `Quaternion`.
    pub fn one() -> Quaternion {
        Quaternion {
            s: 1f64,
            i: 0f64,
            j: 0f64,
            k: 0f64,
        }
    }

    // Creation of new instance of `Quaternion` by just feeding coordinates.
    pub fn new(s: f64, i: f64, j: f64, k:f64) -> Quaternion {
        Quaternion {
            s,
            i,
            j,
            k,
        }
    }

    // Creation of a `Quaternion` instance from a vector slice.
    pub fn from_vec(point: &[f64]) -> Quaternion {
        Quaternion {
            s: point[0],
            i: point[1],
            j: point[2],
            k: point[3],
        }
    }

    // Here we implement the magnitude/absolute value of the `Quaternion` struct.
    pub fn abs(&self) -> f64 {
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
        if self.abs() == 0f64 {
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
    use crate::Quaternion;

    #[test]
    fn addition() {
        let quat1 = Quaternion::from_vec(&[2., 2., 2., 2.]);
        let quat2 = Quaternion::from_vec(&[3., -2., -2., 3.]);
        let quat3 = Quaternion::new(5., 0., 0., 5.);
        assert_eq!(quat3, quat1 + quat2);
    }
    
    #[test]
    fn multiplication_and_inverse() {
        static EPSILON: f64 = 0.00000001;
        let quat1 = Quaternion::new(-2., 3., 4., 5.);
        let quat2 = quat1.inv();
        let identity = Quaternion::one();
        assert!((&identity - &quat2 * &quat1).abs() < EPSILON && (identity - quat2 * quat1).abs() > 0.);
    }

    #[test]
    #[should_panic]
    fn divided_by_zero() {
        let zero = Quaternion::origin();
        let _panic_operation = zero.inv();
    }
}
