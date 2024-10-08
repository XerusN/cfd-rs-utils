use std::ops::{Add, Div, Mul, Sub};

/// 2D vector type using carthesian coordinates system.
/// Using f64.
#[derive(Clone, PartialEq, Debug)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

/// Performs a addition between 2D vectors.
///
/// # Example
///
/// ```rust
/// use cfd_rs_utils::*;
///
/// let a = Vector2D::new(2.0, 2.0);
/// let b = Vector2D::new(4.0, 3.0);
///
/// assert_eq!(&a + &b, Vector2D::new(2.0 + 4.0, 2.0 + 3.0));
/// ```
impl Add for &Vector2D {
    type Output = Vector2D;

    #[inline(always)]
    fn add(self, other: Self) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Performs a substraction between 2D vectors.
///
/// # Example
///
/// ```rust
/// use cfd_rs_utils::*;
///
/// let a = Vector2D::new(2.0, 2.0);
/// let b = Vector2D::new(4.0, 3.0);
///
/// assert_eq!(&a - &b, Vector2D::new(2.0 - 4.0, 2.0 - 3.0));
/// ```
impl Sub for &Vector2D {
    type Output = Vector2D;

    #[inline(always)]
    fn sub(self, other: Self) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Performs a dot product between 2D vectors.
///
/// # Example
///
/// ```rust
/// use cfd_rs_utils::*;
///
/// let a = Vector2D::new(2.0, 1.0);
/// let b = Vector2D::new(4.0, 3.0);
///
/// assert_eq!(&a * &b, 2.0*4.0 + 1.0*3.0);
/// ```
impl Mul for &Vector2D {
    type Output = f64;

    #[inline(always)]
    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

/// Performs a division between a 2D vector and a float.
///
/// # Example
///
/// ```rust
/// use cfd_rs_utils::*;
///
/// let a = Vector2D::new(1.0, 3.0);
///
/// assert_eq!(&a * 2.0, Vector2D::new(2.0 * 1.0, 2.0 * 3.0));
/// ```
impl Mul<f64> for &Vector2D {
    type Output = Vector2D;

    #[inline(always)]
    fn mul(self, other: f64) -> Vector2D {
        Vector2D {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

/// Performs a division between a 2D vector and a float.
///
/// # Example
///
/// ```rust
/// use cfd_rs_utils::*;
///
/// let a = Vector2D::new(4.0, 2.0);
///
/// assert_eq!(&a / 2.0, Vector2D::new(2.0, 1.0));
/// ```
impl Div<f64> for &Vector2D {
    type Output = Vector2D;

    #[inline(always)]
    fn div(self, other: f64) -> Vector2D {
        Vector2D {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Vector2D {
    /// Creates a new Vector2D
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = Vector2D::new(4.0, 2.0);
    /// let b = Vector2D{x: 4.0, y: 2.0,};
    /// assert_eq!(a, b)
    /// ```
    #[inline(always)]
    pub fn new(x: f64, y: f64) -> Vector2D {
        Vector2D { x, y }
    }

    /// Computes the Euclidean norm of the 2D vector
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = Vector2D::new(4.0, 2.0);
    ///
    /// assert_eq!(a.norm(), (a.x * a.x + a.y * a.y).sqrt());
    /// ```
    #[inline(always)]
    pub fn norm(&self) -> f64 {
        (self * self).sqrt()
    }

    /// Normalizes the 2D vector. The norm will then equal to 1.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = Vector2D::new(4.0, 2.0);
    /// let b = a.normalize();
    ///
    /// assert_eq!(b, &a / a.norm());
    /// ```
    #[inline(always)]
    pub fn normalize(&self) -> Vector2D {
        let norm = self.norm();
        self / norm
    }

    /// Returns an orthogonal 2D vector counter-clockwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = Vector2D::new(4.0, 2.0);
    /// let b = Vector2D::new(2.0, -4.0);
    ///
    /// assert_eq!(b, a.orthogonal_vector());
    /// ```
    // With z pointing away from the drawing
    #[inline(always)]
    pub fn orthogonal_vector(&self) -> Vector2D {
        Vector2D {
            x: self.y,
            y: -self.x,
        }
    }
}
