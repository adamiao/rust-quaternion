/**************************************************************************************************/
/*          OPERATOR OVERLOADING FOR ALGEBRAIC MANIPULATIONS OF THE `QUATERNION` STRUCT           */
/**************************************************************************************************/

/*
We will define in the following the operations for addition, subtraction, multiplication, and
division for the `Quaternion` struct. What will be noticed is the need to make the same type of
definitions for all combinations of references and borrowed values:

                                            T + U
                                            &T + U
                                            T + &U
                                            &T + &U

As of this moment I don't quite understand how the original coders of `Rust` implemented it all
using `macros`. The use of `macros` reduces this need of code repetition for sure. This will
be part of my to do list in the future.
*/

use std::ops;
use std::cmp;
use crate::Quaternion;

// ADDITION

impl ops::Add for Quaternion {
    type Output = Quaternion;

    fn add (self, _rhs: Quaternion) -> Quaternion {
        Quaternion {
            s: self.s + _rhs.s,
            i: self.i + _rhs.i,
            j: self.j + _rhs.j,
            k: self.k + _rhs.k,
        }
    }
}

impl<'a> ops::Add<Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn add (self, _rhs: Quaternion) -> Quaternion {
        Quaternion {
            s: self.s + _rhs.s,
            i: self.i + _rhs.i,
            j: self.j + _rhs.j,
            k: self.k + _rhs.k,
        }
    }
}

impl<'b> ops::Add<&'b Quaternion> for Quaternion {
    type Output = Quaternion;

    fn add (self, _rhs: &'b Quaternion) -> Quaternion {
        Quaternion {
            s: self.s + _rhs.s,
            i: self.i + _rhs.i,
            j: self.j + _rhs.j,
            k: self.k + _rhs.k,
        }
    }
}

impl<'a, 'b> ops::Add<&'b Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn add (self, _rhs: &'b Quaternion) -> Quaternion {
        Quaternion {
            s: self.s + _rhs.s,
            i: self.i + _rhs.i,
            j: self.j + _rhs.j,
            k: self.k + _rhs.k,
        }
    }
}

// SUBTRACTION

impl ops::Sub for Quaternion {
    type Output = Quaternion;

    fn sub (self, _rhs: Quaternion) -> Quaternion {
        Quaternion {
            s: self.s - _rhs.s,
            i: self.i - _rhs.i,
            j: self.j - _rhs.j,
            k: self.k - _rhs.k,
        }
    }
}

impl<'a> ops::Sub<Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn sub (self, _rhs: Quaternion) -> Quaternion {
        Quaternion {
            s: self.s - _rhs.s,
            i: self.i - _rhs.i,
            j: self.j - _rhs.j,
            k: self.k - _rhs.k,
        }
    }
}

impl<'b> ops::Sub<&'b Quaternion> for Quaternion {
    type Output = Quaternion;

    fn sub (self, _rhs: &'b Quaternion) -> Quaternion {
        Quaternion {
            s: self.s - _rhs.s,
            i: self.i - _rhs.i,
            j: self.j - _rhs.j,
            k: self.k - _rhs.k,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn sub (self, _rhs: &'b Quaternion) -> Quaternion {
        Quaternion {
            s: self.s - _rhs.s,
            i: self.i - _rhs.i,
            j: self.j - _rhs.j,
            k: self.k - _rhs.k,
        }
    }
}

// MULTIPLICATION

impl ops::Mul for Quaternion {
    type Output = Quaternion;

    fn mul (self, _rhs: Quaternion) -> Quaternion {
        Quaternion {
            s: self.s*_rhs.s - self.i*_rhs.i - self.j*_rhs.j - self.k*_rhs.k,
            i: self.i*_rhs.s + self.s*_rhs.i - self.k*_rhs.j + self.j*_rhs.k,
            j: self.j*_rhs.s + self.k*_rhs.i + self.s*_rhs.j - self.i*_rhs.k,
            k: self.k*_rhs.s - self.j*_rhs.i + self.i*_rhs.j + self.s*_rhs.k,
        }
    }
}

impl<'a> ops::Mul<Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn mul (self, _rhs: Quaternion) -> Quaternion {
        Quaternion {
            s: self.s*_rhs.s - self.i*_rhs.i - self.j*_rhs.j - self.k*_rhs.k,
            i: self.i*_rhs.s + self.s*_rhs.i - self.k*_rhs.j + self.j*_rhs.k,
            j: self.j*_rhs.s + self.k*_rhs.i + self.s*_rhs.j - self.i*_rhs.k,
            k: self.k*_rhs.s - self.j*_rhs.i + self.i*_rhs.j + self.s*_rhs.k,
        }
    }
}

impl<'b> ops::Mul<&'b Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul (self, _rhs: &'b Quaternion) -> Quaternion {
        Quaternion {
            s: self.s*_rhs.s - self.i*_rhs.i - self.j*_rhs.j - self.k*_rhs.k,
            i: self.i*_rhs.s + self.s*_rhs.i - self.k*_rhs.j + self.j*_rhs.k,
            j: self.j*_rhs.s + self.k*_rhs.i + self.s*_rhs.j - self.i*_rhs.k,
            k: self.k*_rhs.s - self.j*_rhs.i + self.i*_rhs.j + self.s*_rhs.k,
        }
    }
}

impl<'a, 'b> ops::Mul<&'b Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn mul (self, _rhs: &'b Quaternion) -> Quaternion {
        Quaternion {
            s: self.s*_rhs.s - self.i*_rhs.i - self.j*_rhs.j - self.k*_rhs.k,
            i: self.i*_rhs.s + self.s*_rhs.i - self.k*_rhs.j + self.j*_rhs.k,
            j: self.j*_rhs.s + self.k*_rhs.i + self.s*_rhs.j - self.i*_rhs.k,
            k: self.k*_rhs.s - self.j*_rhs.i + self.i*_rhs.j + self.s*_rhs.k,
        }
    }
}

// DIVISION

impl ops::Div for Quaternion {
    type Output = Quaternion;

    fn div (self, _rhs: Quaternion) -> Quaternion {
        let _rhs_abs_sqr = _rhs.abs().powf(2.);

        Quaternion {
            s: (self.s*_rhs.s + self.i*_rhs.i + self.j*_rhs.j + self.k*_rhs.k) / _rhs_abs_sqr,
            i: (self.i*_rhs.s - self.s*_rhs.i + self.k*_rhs.j - self.j*_rhs.k) / _rhs_abs_sqr,
            j: (self.j*_rhs.s - self.k*_rhs.i - self.s*_rhs.j + self.i*_rhs.k) / _rhs_abs_sqr,
            k: (self.k*_rhs.s + self.j*_rhs.i - self.i*_rhs.j - self.s*_rhs.k) / _rhs_abs_sqr,
        }
    }
}

impl<'a> ops::Div<Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn div (self, _rhs: Quaternion) -> Quaternion {
        let _rhs_abs_sqr = _rhs.abs().powf(2.);

        Quaternion {
            s: (self.s*_rhs.s + self.i*_rhs.i + self.j*_rhs.j + self.k*_rhs.k) / _rhs_abs_sqr,
            i: (self.i*_rhs.s - self.s*_rhs.i + self.k*_rhs.j - self.j*_rhs.k) / _rhs_abs_sqr,
            j: (self.j*_rhs.s - self.k*_rhs.i - self.s*_rhs.j + self.i*_rhs.k) / _rhs_abs_sqr,
            k: (self.k*_rhs.s + self.j*_rhs.i - self.i*_rhs.j - self.s*_rhs.k) / _rhs_abs_sqr,
        }
    }
}

impl<'b> ops::Div<&'b Quaternion> for Quaternion {
    type Output = Quaternion;

    fn div (self, _rhs: &'b Quaternion) -> Quaternion {
        let _rhs_abs_sqr = _rhs.abs().powf(2.);

        Quaternion {
            s: (self.s*_rhs.s + self.i*_rhs.i + self.j*_rhs.j + self.k*_rhs.k) / _rhs_abs_sqr,
            i: (self.i*_rhs.s - self.s*_rhs.i + self.k*_rhs.j - self.j*_rhs.k) / _rhs_abs_sqr,
            j: (self.j*_rhs.s - self.k*_rhs.i - self.s*_rhs.j + self.i*_rhs.k) / _rhs_abs_sqr,
            k: (self.k*_rhs.s + self.j*_rhs.i - self.i*_rhs.j - self.s*_rhs.k) / _rhs_abs_sqr,
        }
    }
}

impl<'a, 'b> ops::Div<&'b Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn div (self, _rhs: &'b Quaternion) -> Quaternion {
        let _rhs_abs_sqr = _rhs.abs().powf(2.);

        Quaternion {
            s: (self.s*_rhs.s + self.i*_rhs.i + self.j*_rhs.j + self.k*_rhs.k) / _rhs_abs_sqr,
            i: (self.i*_rhs.s - self.s*_rhs.i + self.k*_rhs.j - self.j*_rhs.k) / _rhs_abs_sqr,
            j: (self.j*_rhs.s - self.k*_rhs.i - self.s*_rhs.j + self.i*_rhs.k) / _rhs_abs_sqr,
            k: (self.k*_rhs.s + self.j*_rhs.i - self.i*_rhs.j - self.s*_rhs.k) / _rhs_abs_sqr,
        }
    }
}

/**************************************************************************************************/
/*              OPERATOR OVERLOADING FOR `UNARY` OPERATIONS SUCH AS NEGATION                      */
/**************************************************************************************************/

/*
We will define in the following the unary operation for negation of a quaternion, which is simply
the point-wise negative operation for an input quaternion. This means that:

        q = q_s + q_i i + q_j j + q_k k     =>      -q = -q_s + -q_i i + -q_j j + -q_k k

This operation will be defined for both the owned value as well as the borrowed value of a
quaternion (as was done for all operations so far).
*/

impl ops::Neg for Quaternion {
    type Output = Quaternion;

    fn neg(self) -> Quaternion {
        Quaternion {
            s: -self.s,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }
}

impl ops::Neg for &Quaternion {
    type Output = Quaternion;

    fn neg(self) -> Quaternion {
        Quaternion {
            s: -self.s,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }
}

/**************************************************************************************************/
/*                            OPERATOR OVERLOADING FOR ASSIGN OPERATIONS                          */
/**************************************************************************************************/

/*
We now define the `assign` operations for addition, subtraction, multiplication and division:

                                            q += r;
                                            q -= r;
                                            q *= r;
                                            q /= r;

Remark: once again I've defined the same implementations for both the owned objects as well as the
borrowed objects. Furthermore I've decided to leave two "different" ways of writing the code: the
first I explicitly use the struct `Quaternion` while in the other I do the implementation using the
`Self` object. I left these two ways of doing things so I can come back and remember things can be
done in multiple ways.
*/

// ADDITION

impl ops::AddAssign for Quaternion {
    fn add_assign(&mut self, _rhs: Quaternion) {
        *self = Quaternion {
            s: self.s + _rhs.s,
            i: self.i + _rhs.i,
            j: self.j + _rhs.j,
            k: self.k + _rhs.k,
        };
    }
}

impl<'a> ops::AddAssign<&'a Quaternion> for Quaternion {
    fn add_assign(&mut self, _rhs: &'a Quaternion) {
        *self = Quaternion {
            s: self.s + _rhs.s,
            i: self.i + _rhs.i,
            j: self.j + _rhs.j,
            k: self.k + _rhs.k,
        };
    }
}

// SUBTRACTION

impl ops::SubAssign for Quaternion {
    fn sub_assign(&mut self, _rhs: Self) {
        *self = Self {
            s: self.s - _rhs.s,
            i: self.i - _rhs.i,
            j: self.j - _rhs.j,
            k: self.k - _rhs.k,
        };
    }
}

impl<'a> ops::SubAssign<&'a Self> for Quaternion {
    fn sub_assign(&mut self, _rhs: &Self) {
        *self = Self {
            s: self.s - _rhs.s,
            i: self.i - _rhs.i,
            j: self.j - _rhs.j,
            k: self.k - _rhs.k,
        };
    }
}

// MULTIPLICATION

impl ops::MulAssign for Quaternion {
    fn mul_assign(&mut self, _rhs: Self) {
        *self = Self {
            s: self.s * _rhs.s,
            i: self.i * _rhs.i,
            j: self.j * _rhs.j,
            k: self.k * _rhs.k,
        };
    }
}

impl<'a>  ops::MulAssign<&'a Self> for Quaternion {
    fn mul_assign(&mut self, _rhs: &Self) {
        *self = Self {
            s: self.s * _rhs.s,
            i: self.i * _rhs.i,
            j: self.j * _rhs.j,
            k: self.k * _rhs.k,
        };
    }
}

// DIVISION

impl ops::DivAssign for Quaternion {
    fn div_assign(&mut self, _rhs: Self) {
        *self = Self {
            s: self.s / _rhs.s,
            i: self.i / _rhs.i,
            j: self.j / _rhs.j,
            k: self.k / _rhs.k,
        };
    }
}

impl<'a> ops::DivAssign<&'a Self> for Quaternion {
    fn div_assign(&mut self, _rhs: &Self) {
        *self = Self {
            s: self.s / _rhs.s,
            i: self.i / _rhs.i,
            j: self.j / _rhs.j,
            k: self.k / _rhs.k,
        };
    }
}

/**************************************************************************************************/
/*        COMMUTATIVE SCALAR MULTIPLICATION AND DIVISION FOR THE QUATERNION STRUCT                */
/**************************************************************************************************/

/*
In what follows we'll define commutative scalar multiplication and division for the `Quaternion`
struct objects. This is a mathematical property of quaternions that we will want to have available
in our code capabilities.

If `c` is some constant of type belonging to the set {i32, f64}, then we want to be able to perform
the following operations:
                                                c * q
                                                q * c
                                                c / q
                                                q / c

These operations, ultimately, are just point-wise multiplication/division of the quaternion and the
scalar constant `c`.

Remark: I also had to include the situations where we're borrowing the `Quaternion` object when
doing the operations. Furthermore, I needed to include separate definitions for the order in which
the operation happens. This means that we have four implementations per operation and per data type
the constant `c` belongs to (i32, f64).
*/

// MULTIPLICATION OF `QUATERNION` AND `f64`

impl ops::Mul<Quaternion> for f64 {
    type Output = Quaternion;

    fn mul(self, _rhs: Quaternion) -> Quaternion {
        Quaternion {
            s: self * _rhs.s,
            i: self * _rhs.i,
            j: self * _rhs.j,
            k: self * _rhs.k,
        }
    }
}

impl ops::Mul<f64> for Quaternion {
    type Output = Quaternion;

    fn mul(self, _rhs: f64) -> Quaternion {
        Quaternion {
            s: self.s * _rhs,
            i: self.i * _rhs,
            j: self.j * _rhs,
            k: self.k * _rhs,
        }
    }
}

impl ops::Mul<&Quaternion> for f64 {
    type Output = Quaternion;

    fn mul(self, _rhs: &Quaternion) -> Quaternion {
        Quaternion {
            s: self * _rhs.s,
            i: self * _rhs.i,
            j: self * _rhs.j,
            k: self * _rhs.k,
        }
    }
}

impl ops::Mul<f64> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, _rhs: f64) -> Quaternion {
        Quaternion {
            s: self.s * _rhs,
            i: self.i * _rhs,
            j: self.j * _rhs,
            k: self.k * _rhs,
        }
    }
}

// MULTIPLICATION OF `QUATERNION` AND `i32`

impl ops::Mul<Quaternion> for i32 {
    type Output = Quaternion;

    fn mul(self, _rhs: Quaternion) -> Quaternion {
        Quaternion {
            s: (self as f64) * _rhs.s,
            i: (self as f64) * _rhs.i,
            j: (self as f64) * _rhs.j,
            k: (self as f64) * _rhs.k,
        }
    }
}

impl ops::Mul<i32> for Quaternion {
    type Output = Quaternion;

    fn mul(self, _rhs: i32) -> Quaternion {
        Quaternion {
            s: self.s * (_rhs as f64),
            i: self.i * (_rhs as f64),
            j: self.j * (_rhs as f64),
            k: self.k * (_rhs as f64),
        }
    }
}

impl ops::Mul<&Quaternion> for i32 {
    type Output = Quaternion;

    fn mul(self, _rhs: &Quaternion) -> Quaternion {
        Quaternion {
            s: (self as f64) * _rhs.s,
            i: (self as f64) * _rhs.i,
            j: (self as f64) * _rhs.j,
            k: (self as f64) * _rhs.k,
        }
    }
}

impl ops::Mul<i32> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, _rhs: i32) -> Quaternion {
        Quaternion {
            s: self.s * (_rhs as f64),
            i: self.i * (_rhs as f64),
            j: self.j * (_rhs as f64),
            k: self.k * (_rhs as f64),
        }
    }
}

// DIVISION OF `QUATERNION` AND `f64`

impl ops::Div<Quaternion> for f64 {
    type Output = Quaternion;

    fn div(self, _rhs: Quaternion) -> Quaternion {
        let _rhs_abs_sqr = _rhs.abs().powf(2.);

        Quaternion {
            s: (self * _rhs.s)/_rhs_abs_sqr,
            i: (self * _rhs.i)/_rhs_abs_sqr,
            j: (self * _rhs.j)/_rhs_abs_sqr,
            k: (self * _rhs.k)/_rhs_abs_sqr,
        }
    }
}

impl ops::Div<f64> for Quaternion {
    type Output = Quaternion;

    fn div(self, _rhs: f64) -> Quaternion {
        Quaternion {
            s: self.s / _rhs,
            i: self.i / _rhs,
            j: self.j / _rhs,
            k: self.k / _rhs,
        }
    }
}

impl ops::Div<&Quaternion> for f64 {
    type Output = Quaternion;

    fn div(self, _rhs: &Quaternion) -> Quaternion {
        let _rhs_abs_sqr = _rhs.abs().powf(2.);

        Quaternion {
            s: (self * _rhs.s)/_rhs_abs_sqr,
            i: (self * _rhs.i)/_rhs_abs_sqr,
            j: (self * _rhs.j)/_rhs_abs_sqr,
            k: (self * _rhs.k)/_rhs_abs_sqr,
        }
    }
}

impl ops::Div<f64> for &Quaternion {
    type Output = Quaternion;

    fn div(self, _rhs: f64) -> Quaternion {
        Quaternion {
            s: self.s / _rhs,
            i: self.i / _rhs,
            j: self.j / _rhs,
            k: self.k / _rhs,
        }
    }
}

// DIVISION OF `QUATERNION` AND `i32`

impl ops::Div<Quaternion> for i32 {
    type Output = Quaternion;

    fn div(self, _rhs: Quaternion) -> Quaternion {
        let _rhs_abs_sqr = _rhs.abs().powf(2.);

        Quaternion {
            s: ((self as f64) * _rhs.s)/_rhs_abs_sqr,
            i: ((self as f64) * _rhs.i)/_rhs_abs_sqr,
            j: ((self as f64) * _rhs.j)/_rhs_abs_sqr,
            k: ((self as f64) * _rhs.k)/_rhs_abs_sqr,
        }
    }
}

impl ops::Div<i32> for Quaternion {
    type Output = Quaternion;

    fn div(self, _rhs: i32) -> Quaternion {
        Quaternion {
            s: self.s / (_rhs as f64),
            i: self.i / (_rhs as f64),
            j: self.j / (_rhs as f64),
            k: self.k / (_rhs as f64),
        }
    }
}

impl ops::Div<&Quaternion> for i32 {
    type Output = Quaternion;

    fn div(self, _rhs: &Quaternion) -> Quaternion {
        let _rhs_abs_sqr = _rhs.abs().powf(2.);

        Quaternion {
            s: ((self as f64) * _rhs.s)/_rhs_abs_sqr,
            i: ((self as f64) * _rhs.i)/_rhs_abs_sqr,
            j: ((self as f64) * _rhs.j)/_rhs_abs_sqr,
            k: ((self as f64) * _rhs.k)/_rhs_abs_sqr,
        }
    }
}

impl ops::Div<i32> for &Quaternion {
    type Output = Quaternion;

    fn div(self, _rhs: i32) -> Quaternion {
        Quaternion {
            s: self.s / (_rhs as f64),
            i: self.i / (_rhs as f64),
            j: self.j / (_rhs as f64),
            k: self.k / (_rhs as f64),
        }
    }
}

/**************************************************************************************************/
/*                 OVERLOAD OF THE `cmp::PartialEq` FOR THE QUATERNION STRUCT                     */
/**************************************************************************************************/

/*
Here we define what it means for two `Quaternions` to be equal to each other. Essentially we just
make sure that all coordinates are point-wise the same for each of these objects.
*/

impl cmp::PartialEq for Quaternion {
    fn eq(&self, _rhs: &Self) -> bool {
        self.s == _rhs.s && self.i == _rhs.i && self.j == _rhs.j && self.k == _rhs.k
    }
}