use std::ops::{Add, Div, Mul, Sub};
use super::Vector2D;

/// 2D point type using carthesian coordinates system.
/// Using f64.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// Performs a addition between coordinates points.
///
/// # Example
///
/// ```rust
/// use cfd_rs_utils::*;
///
/// let a = Point2D::new(2.0, 2.0);
/// let b = Point2D::new(4.0, 3.0);
///
/// assert_eq!(&a + &b, Point2D::new(2.0 + 4.0, 2.0 + 3.0));
/// ```
impl Add for &Point2D {
    type Output = Point2D;
    
    #[inline(always)]
    fn add(self, other: Self) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Performs a substraction between 2D coordinates points.
///
/// # Example
///
/// ```rust
/// use cfd_rs_utils::*;
///
/// let a = Point2D::new(2.0, 2.0);
/// let b = Point2D::new(4.0, 3.0);
///
/// assert_eq!(&a - &b, Point2D::new(2.0 - 4.0, 2.0 - 3.0));
/// ```
impl Sub for &Point2D {
    type Output = Point2D;
    
    #[inline(always)]
    fn sub(self, other: Self) -> Point2D {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Performs a division between 2D coordinates and a float.
///
/// # Example
///
/// ```rust
/// use cfd_rs_utils::*;
///
/// let a = Point2D::new(1.0, 3.0);
///
/// assert_eq!(&a * 2.0, Point2D::new(2.0 * 1.0, 2.0 * 3.0));
/// ```
impl Mul<f64> for &Point2D {
    type Output = Point2D;
    
    #[inline(always)]
    fn mul(self, other: f64) -> Point2D {
        Point2D {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

/// Performs a division between 2D coordinates and a float.
///
/// # Example
///
/// ```rust
/// use cfd_rs_utils::*;
///
/// let a = Point2D::new(4.0, 2.0);
///
/// assert_eq!(&a / 2.0, Point2D::new(2.0, 1.0));
/// ```
impl Div<f64> for &Point2D {
    type Output = Point2D;
    
    #[inline(always)]
    fn div(self, other: f64) -> Point2D {
        Point2D {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Point2D {
    /// Creates a Point2D
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = Point2D::new(3.0, 2.0);
    /// let b = Point2D{x: 3.0, y: 2.0,};
    /// 
    /// assert_eq!(a, b);
    /// ```
    #[inline(always)]
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D {
            x: x,
            y: y,
        }
    }
    
    /// Creates a 2D vector between two points (similar to a substraction but returns a Vector2D).
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = Point2D::new(2.0, 2.0);
    /// let b = Point2D::new(4.0, 3.0);
    ///
    /// assert_eq!(b.vector_to(&a), Vector2D::new(2.0 - 4.0, 2.0 - 3.0));
    /// ```
    #[inline(always)]
    pub fn vector_to(&self, other: &Self) -> Vector2D {
        Vector2D {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
    
    /// Convert a Point2D to a Vector2D
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = Point2D::new(2.0, 3.0);
    /// let a_vector = Vector2D::new(2.0, 3.0);
    ///
    /// assert_eq!(a.to_vector(), a_vector);
    /// ```
    #[inline(always)]
    pub fn to_vector(&self) -> Vector2D {
        Vector2D {
            x: self.x,
            y: self.y,
        }
    }
}
