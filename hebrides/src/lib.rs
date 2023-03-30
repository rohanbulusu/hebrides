//! Implementations for real and complex numbers.
//!
//! `Hebrides` provides two main structs: Real and Complex, which together
//! form the basis of the Marlin ecocystem, which as a whole constitutes the
//! whole of the implementation of DOSs in Rust.

#![deny(rust_2018_idioms, missing_docs)]

use std::ops::{Add, Sub, Mul, Div, Neg};

/// Evaluates approximate equality betwen two values.
/// 
/// ```
/// # use hebrides::approx_eq;
/// approx_eq(0.3_f64*0.2_f64, 0.06_f64);
/// ```
pub fn approx_eq(left: f64, right: f64) -> bool {
    (left - right).abs() <= f64::EPSILON
}

/// Error type for errors involving domain restrictions.
#[derive(Debug, Clone)]
pub struct DomainError;

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "evaluation error due to domain restriction")
    }
}

/// Error type for errors involving conversion between values.
#[derive(Debug, Clone)]
pub struct ConversionError;

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conversion error due to invalid predicate")
    }
}

/// Representation of angular values.
///
/// Angles are often described in either radians or degrees. This datatype is
/// a unitless value that allows conversion to either unit system.
pub struct Angle {
    radian_form: Option<f64>,
    degree_form: Option<f64>
}

impl Angle {

    /// Converts a `value` in radians into a value in degrees
    ///
    /// ```
    /// # use hebrides::Angle;
    /// assert_eq!(Angle::into_degrees(std::f64::consts::FRAC_PI_2), 90.0);
    /// ```
    pub fn into_degrees(value: f64) -> f64 {
        value * 180.0 / std::f64::consts::PI
    }

    /// Converts a `value` in degrees into a value in radians
    ///
    /// ```
    /// # use hebrides::Angle;
    /// assert_eq!(Angle::into_radians(90.0), std::f64::consts::FRAC_PI_2);
    /// ```
    pub fn into_radians(value: f64) -> f64 {
        value * std::f64::consts::PI / 180.0
    }

    /// Constructs an Angle from a given value in radians
    pub fn from_radians(radian_value: f64) -> Self {
        Angle {
            radian_form: Some(radian_value),
            degree_form: None
        }
    }

    /// Constructs an Angle from a given value in degrees
    pub fn from_degrees(degree_value: f64) -> Self {
        Angle {
            radian_form: None,
            degree_form: Some(degree_value)
        }
    }

    /// Consumes `self`, returning a value in radians.
    /// 
    /// ```
    /// # use hebrides::Angle;
    /// let theta = Angle::from_degrees(90.0);
    /// assert_eq!(theta.to_radians(), std::f64::consts::FRAC_PI_2);
    /// ```
    pub fn to_radians(self) -> f64 {
        match self.radian_form {
            Some(in_radians) => in_radians,
            None => Angle::into_radians(self.degree_form.unwrap())
        }
    }

    /// Consumes `self`, returning a value in degrees.
    ///
    /// ```
    /// # use hebrides::Angle;
    /// let theta = Angle::from_radians(std::f64::consts::FRAC_PI_2);
    /// assert_eq!(theta.to_degrees(), 90.0);
    /// ```
    pub fn to_degrees(self) -> f64 {
        match self.degree_form {
            Some(in_degrees) => in_degrees,
            None => Angle::into_degrees(self.radian_form.unwrap())
        }
    }

}

/// Representation of real numbers.
///
/// Real numbers can be thought of, without loss of generality, as the set of
/// integers union the set of rationals union the set of irrationals. The only
/// primitive data type native to Rust that is able to represent all three of 
/// these classes is f64 (not counting the impossibility of completely 
/// describing irrational numbers as decimals), which is why it's used as the
/// struct's internal representation of real values.
#[derive(Copy, Clone, Debug)]
pub struct Real {
    inner: f64
}

impl Real {

    /// shortcut for describing Real::new(0.0)
    pub const ONE: Real = Real { inner: 1.0 };

    /// shortcut for describing Real::new(1.0)
    pub const ZERO: Real = Real { inner: 0.0 };

    /// Constructs a Real from an f64.
    pub fn new(inner: f64) -> Self {
        Real { inner }
    }

    /// Returns a copy of `self` as an f64.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let y = Real::new(12.34);
    /// assert_eq!(y.value(), 12.34);
    /// ```
    pub fn value(&self) -> f64 {
        self.inner
    }


    /// Whether or not `self` is positive.
    /// 
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(0.4);
    /// assert!(x.positive());
    /// let y = Real::new(-0.4);
    /// assert!(!y.positive());
    /// ```
    pub fn positive(&self) -> bool {
        if self.inner > 0.0 {
            return true;
        }
        false
    }

    /// Whether or not `self` is negative.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(0.5);
    /// assert!(!x.negative());
    /// let y = Real::new(-0.5);
    /// assert!(y.negative());
    /// ```
    pub fn negative(&self) -> bool {
        if self.inner < 0.0 {
            return true;
        }
        false
    }

    /// Absolute value of `self`.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::ONE;
    /// assert_eq!(x.abs(), Real::ONE);
    /// let y = -Real::ONE;
    /// assert_eq!(y.abs(), Real::ONE);
    /// ```
    pub fn abs(&self) -> Self {
        Real::new(self.inner.abs())
    }

    /// Returns the ceiling of `self`.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(0.02342);
    /// assert_eq!(x.ceil(), Real::ONE);
    /// ```
    pub fn ceil(&self) -> Self {
        Real::new(self.inner.ceil())
    }

    /// Returns the floor of `self`.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(0.02322);
    /// assert_eq!(x.floor(), Real::ZERO);
    /// ```
    pub fn floor(&self) -> Self {
        Real::new(self.inner.floor())
    }

    /// Sine.
    /// 
    /// ```
    /// # use hebrides::Real;
    /// let theta = Real::new(std::f64::consts::PI);
    /// assert_eq!(theta.sin(), Real::ZERO);
    /// ```
    pub fn sin(&self) -> Self {
        Real::new(self.inner.sin())
    }

    /// Cosine.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let theta = Real::new(std::f64::consts::PI);
    /// assert_eq!(theta.cos(), -Real::ONE);
    /// ```
    pub fn cos(&self) -> Self {
        Real::new(self.inner.cos())
    }

    /// Tangent.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let theta = Real::new(std::f64::consts::FRAC_PI_4);
    /// assert_eq!(theta.tan(), Real::ONE);
    /// ```
    pub fn tan(&self) -> Self {
        Real::new(self.inner.tan())
    }

    /// Arcsine.
    ///
    /// Inputs to arcsin must be between negative one and one; its
    /// range extends from [pi/2, -pi/2].
    ///
    /// ```
    /// # use hebrides::Real;
    /// let h = Real::new(4.0 / 5.0);
    /// assert_eq!(h.arcsin().unwrap().to_radians(), 0.8_f64.asin());
    /// ```
    pub fn arcsin(&self) -> Result<Angle, DomainError> {
        if self.inner < -1.0 || self.inner > 1.0 {
            return Err(DomainError);
        }
        Ok(Angle::from_radians(self.inner.asin()))
    }

    /// Arccosine.
    ///
    /// Inputs to arccos must be between negative one and one; its
    /// range extends from [0, pi].
    /// 
    /// ```
    /// # use hebrides::Real;
    /// let h = Real::new(3.0 / 5.0);
    /// assert_eq!(h.arccos().unwrap().to_radians(), 0.6_f64.acos());
    /// ```
    pub fn arccos(&self) -> Result<Angle, DomainError> {
        if self.inner < -1.0 || self.inner > 1.0 {
            return Err(DomainError);
        }
        Ok(Angle::from_radians(self.inner.acos()))
    }

    /// Arctangent.
    ///
    /// The range of arctan extends between [-pi/2, pi/2].
    ///
    /// ```
    /// # use hebrides::Real;
    /// let h = Real::ONE;
    /// assert_eq!(h.arctan().to_radians(), 1.0_f64.atan());
    /// ```
    pub fn arctan(&self) -> Angle {
        Angle::from_radians(self.inner.atan())
    }

    /// Hyperbolic sine.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(2.0);
    /// assert_eq!(x.sinh(), Real::new(2.0_f64.sinh()));
    /// ```
    pub fn sinh(&self) -> Self {
        Real::new(self.inner.sinh())
    }

    /// Hyperbolic cosine.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(3.0);
    /// assert_eq!(x.cosh(), Real::new(3.0_f64.cosh()));
    /// ```
    pub fn cosh(&self) -> Self {
        Real::new(self.inner.cosh())
    }

    /// Hyperbolic tangent.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(4.0);
    /// assert_eq!(x.tanh(), Real::new(4.0_f64.tanh()));
    /// ```
    pub fn tanh(&self) -> Self {
        Real::new(self.inner.tanh())
    }

    /// Hyperbolic arcsine.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(3.0);
    /// assert_eq!(x.arcsinh().to_radians(), 3.0_f64.asinh());
    /// ```
    pub fn arcsinh(&self) -> Angle {
        Angle::from_radians(self.inner.asinh())
    }

    /// Hyperbolic arccosine.
    /// 
    /// Inputs to arccosh must be between [1, infinity].
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(2.0);
    /// assert_eq!(x.arccosh().unwrap().to_radians(), 2.0_f64.acosh());
    /// ```
    pub fn arccosh(&self) -> Result<Angle, DomainError> {
        if self.inner < 1.0 {
            return Err(DomainError);
        }
        Ok(Angle::from_radians(self.inner.acosh()))
    }

    /// Hyperbolic arctangent.
    ///
    /// Inputs to arctanh must be between [-1, 1]. 
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(0.5);
    /// assert_eq!(x.arctanh().unwrap().to_radians(), 0.5_f64.atanh());
    /// ```
    pub fn arctanh(&self) -> Result<Angle, DomainError> {
        if self.inner <= -1.0 || self.inner >= 1.0 {
            return Err(DomainError);
        }
        Ok(Angle::from_radians(self.inner.atanh()))
    }

    /// e to the power of `self`.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::ZERO;
    /// assert_eq!(x.exp(), Real::ONE);
    /// let y = Real::ONE;
    /// assert_eq!(Real::ONE.exp(), Real::new(std::f64::consts::E));
    /// ```
    pub fn exp(&self) -> Self {
        Real::new(self.inner.exp())
    }

    /// `self` to the power of `power`.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(2.0);
    /// assert_eq!(x, x.pow(Real::ONE));
    /// assert_eq!(x*x, x.pow(Real::new(2.0)));
    /// ```
    pub fn pow(&self, power: Self) -> Self {
        Real::new(self.inner.powf(power.inner))
    }

    /// `self` to the power of `power`.
    /// 
    /// Only accepts non-negative arguments; imaginary values
    /// are not included in the output of powf.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(2.0);
    /// assert_eq!(x, x.powf(1.0).unwrap());
    /// assert_eq!(x*x, x.powf(2.0).unwrap());
    /// ```
    pub fn powf(&self, power: f64) -> Result<Self, DomainError> {
        if self.negative() {
            return Err(DomainError);
        }
        Ok(Real::new(self.inner.powf(power)))
    }

    /// `self` to the power of `power`.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(2.0);
    /// assert_eq!(x, x.powi(1));
    /// assert_eq!(x*x, x.powi(2));
    /// ```
    pub fn powi(&self, power: i64) -> Self {
        Real::new(self.inner.powf(power as f64))
    }

    /// Returns the square of `self`.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(2.0);
    /// assert_eq!(x.squared(), Real::new(4.0));
    /// let y = Real::new(-2.0);
    /// assert_eq!(y.squared(), Real::new(4.0));
    /// ```
    pub fn squared(&self) -> Self {
        self.powi(2)
    }

    /// Returns the square root of `self`.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(4.0);
    /// assert_eq!(x.sqrt().unwrap(), Real::new(2.0));
    /// ```
    pub fn sqrt(&self) -> Result<Self, DomainError> {
        self.powf(0.5)
    }

    /// Natural logarithm.
    ///
    /// Inputs to ln must be greater than zero.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let e = Real::new(std::f64::consts::E);
    /// assert_eq!(e.ln().unwrap(), Real::ONE);
    /// let x = Real::ONE;
    /// assert_eq!(x.ln().unwrap(), Real::new(1.0_f64.ln()));
    /// ```
    pub fn ln(&self) -> Result<Self, DomainError> {
        if self.inner <= 0.0 {
            return Err(DomainError);
        }
        Ok(Real::new(self.inner.ln()))
    }

    /// Logarithm with base 'base'.
    /// 
    /// Inputs for `base` must be greater than zero, and as for
    /// all logarithms, `self` must also be greater than zero.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(5.0);
    /// assert_eq!(x.log(Real::new(5.0)).unwrap(), Real::ONE);
    /// ```
    pub fn log(&self, base: Real) -> Result<Self, DomainError> {
        if self.inner <= 0.0 || base.inner <= 0.0 {
            return Err(DomainError);
        }
        Ok(Real::new(self.inner.log(base.inner)))
    }

    /// Logarithm with base 'base'.
    /// 
    /// Inputs for `base` must be greater than zero, and as for
    /// all logarithms, `self` must also be greater than zero.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(5.0);
    /// assert_eq!(x.logf(5.0).unwrap(), Real::ONE);
    /// ```
    pub fn logf(&self, base: f64) -> Result<Self, DomainError> {
        if self.inner <= 0.0 || base <= 0.0 {
            return Err(DomainError);
        }
        Ok(Real::new(self.inner.log(base)))
    }

    /// Logarithm with base 'base'.
    /// 
    /// Inputs for `base` must be greater than zero, and as for
    /// all logarithms, `self` must also be greater than zero.
    ///
    /// ```
    /// # use hebrides::Real;
    /// let x = Real::new(5.0);
    /// assert_eq!(x.logi(5).unwrap(), Real::ONE);
    /// ```
    pub fn logi(&self, base: i64) -> Result<Self, DomainError> {
        if self.inner <= 0.0 || base <= 0 {
            return Err(DomainError);
        }
        Ok(Real::new(self.inner.log(base as f64)))
    }

    /// Constructs a [`Complex`] from `self`.
    ///
    /// ```
    /// # use hebrides::{Real, Complex};
    /// let x = Real::new(3.0);
    /// assert_eq!(x.to_complex(), Complex::new(3.0, 0.0));
    /// ```
    pub fn to_complex(&self) -> Complex {
        Complex::new(self.inner, 0.0)
    }

}

impl std::fmt::Display for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl PartialEq for Real {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.inner, other.inner)
    }
}

impl Eq for Real {}

impl PartialOrd for Real {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Real {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.inner < other.inner {
            return std::cmp::Ordering::Less;
        }
        if self.inner > other.inner {
            return std::cmp::Ordering::Greater;
        }
        std::cmp::Ordering::Equal
    }
}

impl Add<Self> for Real {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Real::new(self.inner + other.inner)
    }
}

impl Sub<Self> for Real {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Real::new(self.inner - other.inner)
    }
}

impl Mul<Self> for Real {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Real::new(self.inner * other.inner)
    }
}

impl Div<Self> for Real {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Real::new(self.inner / other.inner)
    }
}

impl Neg for Real {
    type Output = Self;
    fn neg(self) -> Self {
        Real::new(-self.inner)
    }
}

macro_rules! impl_real_from_primitive {
    ($t:ty) => {
        impl From<$t> for Real {
            fn from(value: $t) -> Self {
                Real::new(value as f64)
            }
        }
    }
}

impl_real_from_primitive![u8];
impl_real_from_primitive![u16];
impl_real_from_primitive![u32];
impl_real_from_primitive![u64];

impl_real_from_primitive![i8];
impl_real_from_primitive![i16];
impl_real_from_primitive![i32];
impl_real_from_primitive![i64];

impl_real_from_primitive![f32];
impl_real_from_primitive![f64];

/// Representation of complex numbers
#[derive(Copy, Clone, Debug)]
pub struct Complex {
    real: Real,
    imag: Real
}

impl Complex {

    /// shortcut for encoding 0.0 as Complex::new(Real::ZERO, Real::ZERO)
    pub const ZERO: Complex = Complex { real: Real::ZERO, imag: Real::ZERO };

    /// shortcut for encoding 1.0 as Complex::new(Real::ONE, Real::ZERO)
    pub const ONE: Complex = Complex { real: Real::ONE, imag: Real::ZERO };

    /// shortcut for encoding the imaginary unit as Complex::new(Real::ZERO, Real::ONE)
    pub const I: Complex = Complex { real: Real::ZERO, imag: Real::ONE };

    /// Constructs a Complex from two f64 values
    pub fn new(real: f64, imag: f64) -> Self {
        Self {
            real: Real::new(real),
            imag: Real::new(imag)
        }
    }

    /// Returns whether or not `self` is real.
    ///
    /// ```
    /// # use hebrides::Complex;
    /// let z = Complex::new(3.0, 2.0);
    /// assert!(!z.is_real());
    /// let x = Complex::new(1.0, 0.0);
    /// assert!(x.is_real());
    /// let y = Complex::new(0.0, 1.0);
    /// assert!(!y.is_real());
    /// ```
    pub fn is_real(&self) -> bool {
        if approx_eq(self.imag.inner, 0.0) {
            return true;
        }
        false
    }

    /// Returns whether or not `self` is imaginary.
    ///
    /// ```
    /// # use hebrides::Complex;
    /// let z = Complex::new(3.0, 2.0);
    /// assert!(!z.is_imaginary());
    /// let x = Complex::new(23.0, 0.0);
    /// assert!(!x.is_imaginary());
    /// let y = Complex::new(0.0, 12.0);
    /// assert!(y.is_imaginary());
    /// ```
    pub fn is_imaginary(&self) -> bool {
        if approx_eq(self.real.inner, 0.0) {
            return true;
        }
        false
    }

    /// Absolute value (aka. complex norm).
    ///
    /// ```
    /// # use hebrides::Complex;
    /// let z = Complex::new(3.0, 4.0);
    /// assert_eq!(z.abs(), Complex::new(5.0, 0.0));
    /// ```
    pub fn abs(&self) -> Complex {
        let norm = (self.real*self.real + self.imag*self.imag).sqrt();
        Complex::new(norm.unwrap().inner, 0.0)
    }

    /// Complex exponentiation.
    ///
    /// Evaluates e to the power of `self`.
    ///
    /// ```
    /// # use hebrides::Complex;
    /// let e = Complex::new(std::f64::consts::E, 0.0);
    /// let z = Complex::new(1.0, std::f64::consts::FRAC_PI_2);
    /// assert_eq!(z.exp(), e*Complex::I);
    /// let q = Complex::new(1.0, 1.0);
    /// assert_eq!(q.exp(), Complex::new(1.4686939399158851, 2.2873552871788423));
    /// let w = Complex::new(3.0, 7.0);
    /// assert_eq!(w.exp(), Complex::new(15.142531566086864, 13.195928586605717))
    /// ```
    pub fn exp(&self) -> Complex {
        let real_exp = Complex::new(self.real.exp().inner, 0.0);
        let imag_exp = Complex::new(self.imag.cos().inner, self.imag.sin().inner);
        real_exp*imag_exp
    }

    /// Complex sine.
    ///
    /// ```
    /// # use hebrides::Complex;
    /// let z = Complex::new(1.0, std::f64::consts::FRAC_PI_2);
    /// assert_eq!(z.sin(), Complex::new(2.1114008854951747, 1.2433971034084503));
    /// ```
    pub fn sin(&self) -> Complex {
        let power = Complex::I * *self;
        let num = power.exp() - (-power).exp();
        let den = Complex::new(0.0, 2.0);
        num / den
    }

    /// Complex cosine.
    ///
    /// ```
    /// # use hebrides::Complex;
    /// let z = Complex::new(0.0, 0.0);
    /// assert_eq!(z.cos(), Complex::new(1.0, 0.0));
    /// let w = Complex::new(1.0, 2.0);
    /// assert_eq!(w.cos(), Complex::new(2.0327230070196656, -3.0518977991517997));
    /// ```
    pub fn cos(&self) -> Complex {
        let power = Complex::I * *self;
        let num = power.exp() + (-power).exp();
        let den = Complex::new(2.0, 0.0);
        num / den
    }

    /// Complex tangent.
    ///
    /// ```
    /// # use hebrides::Complex;
    /// let z = Complex::new(2.0, 3.0);
    /// assert_eq!(z.tan(), Complex::new(-0.0037640256415040815, 1.0032386273536098));
    /// ```
    pub fn tan(&self) -> Complex {
        self.sin() / self.cos()
    }

    /// Complex arcsin.
    ///
    /// Even if `self` is real, this will return only a [`Complex`] and not an [`Angle`]
    /// as might be expected based off of the behavior of the arcsine implementation on
    /// [`Real`].
    ///
    /// Mathematical justification for the implementation can be found at 
    /// [ProofWiki](https://proofwiki.org/wiki/Definition:Inverse_Sine/Complex).
    ///
    /// ```
    /// # use hebrides::{Real, Complex};
    /// let x = Complex::new(0.8, 0.0);
    /// assert_eq!(x.arcsin(), Real::new(Real::new(0.8).arcsin().unwrap().to_radians()).to_complex());
    /// let z = Complex::new(2.0, 3.0);
    /// assert_eq!(z.arcsin(), Complex::new(1.5413069599437077, -1.3679718444908728));
    /// ```
    pub fn arcsin(&self) -> Complex {
        -Complex::I * (*self*Complex::I + (Complex::ONE - self.squared()).sqrt()).ln()
    }

    /// Complex arccos.
    /// 
    /// Even if `self` is real, this will return only a [`Complex`] and not an [`Angle`]
    /// as might be expected based off of the behavior of the arccosine implementation on
    /// [`Real`].
    ///
    /// Mathematical justification for the implementation can be found at 
    /// [ProofWiki](https://proofwiki.org/wiki/Definition:Inverse_Cosine/Complex/Arccosine).
    ///
    /// ```
    /// # use hebrides::{Real, Complex};
    /// let x = Complex::new(0.8, 0.0);
    /// assert_eq!(x.arccos(), Real::new(Real::new(0.8).arccos().unwrap().to_radians()).to_complex());
    /// let z = Complex::new(2.0, 3.0);
    /// assert_eq!(z.arccos(), Complex::new(1.0001435424737972, -1.9833870299165355));
    /// ```
    pub fn arccos(&self) -> Complex {
        -Complex::I * (*self + (self.squared() - Complex::ONE).sqrt()).ln()
    }

    /// Complex arctan.
    ///
    /// Even if `self` is real, this will return only a [`Complex`] and not an [`Angle`]
    /// as might be expected based off of the behavior of the arctangent implementation on
    /// [`Real`].
    ///
    /// Mathematical justification for the implementation can be found at
    /// [ProofWiki](https://proofwiki.org/wiki/Definition:Inverse_Tangent/Complex).
    ///
    /// ```
    /// # use hebrides::{Real, Complex};
    /// let x = Complex::new(0.8, 0.0);
    /// assert_eq!(x.arctan(), Real::new(Real::new(0.8).arctan().to_radians()).to_complex());
    /// let z = Complex::new(2.0, 3.0);
    /// assert_eq!(z.arctan(), Complex::new(1.4099210495965755, 0.22907268296853875));
    /// ```
    pub fn arctan(&self) -> Complex {
        let coefficient = -Complex::I / Real::new(2.0).to_complex();
        let num = Complex::I - *self;
        let den = Complex::I + *self;
        coefficient * (num / den).ln()
    }

    /// Complex hyperbolic sine.
    ///
    /// ```
    /// # use hebrides::{Real, Complex};
    /// let z = Complex::new(3.0, 2.0);
    /// assert_eq!(z.sinh(), Complex::new(-4.168906959966565, 9.154499146911428));
    /// ```
    pub fn sinh(&self) -> Complex {
        (self.exp() - ((-*self).exp())) / Real::new(2.0).to_complex()
    }

    /// Complex hyperbolic cosine.
    ///
    /// ```
    /// # use hebrides::{Real, Complex};
    /// let z = Complex::new(3.0, 2.0);
    /// assert_eq!(z.cosh(), Complex::new(-4.189625690968807, 9.109227893755337));
    /// ```
    pub fn cosh(&self) -> Complex {
        (self.exp() + ((-*self).exp())) / Real::new(2.0).to_complex()
    }

    /// Complex hyperbolic tangent.
    ///
    /// ```
    /// # use hebrides::{Real, Complex};
    /// let z = Complex::new(3.0, 2.0);
    /// assert_eq!(z.tanh(), Complex::new(1.0032386273536098, -0.0037640256415040815));
    /// ```
    pub fn tanh(&self) -> Complex {
        (self.exp() - (-*self).exp()) / (self.exp() + (-*self).exp())
    }

    /// Complex natural logarithm.
    ///
    /// Computes the natural logarithm of `self` on the principal branch of Ln(x).
    ///
    /// ```
    /// # use hebrides::Complex;
    /// let z = Complex::new(3.0, 4.0);
    /// assert_eq!(z.ln(), Complex::new(5.0_f64.ln(), 0.8_f64.asin()));
    /// ```
    pub fn ln(&self) -> Complex {
        if self.is_real() {
            return Complex::new(self.real.ln().unwrap().inner, 0.0);
        }
        Complex::new(self.norm().ln().unwrap().inner, self.azimuthal().to_radians())
    }

    /// Complex norm.
    ///
    /// Returns the absolute value of `self`. If `self` is interpreted as a vector,
    /// this is its length. The norm is always a positive real number.
    ///
    /// ```
    /// # use hebrides::{Real, Complex};
    /// let z = Complex::new(3.0, 4.0);
    /// assert_eq!(z.norm(), Real::new(5.0));
    /// let w = Complex::new(5.0, 12.0);
    /// assert_eq!(w.norm(), Real::new(13.0));
    /// ```
    pub fn norm(&self) -> Real {
        let length = (self.real.squared() + self.imag.squared()).sqrt();
        Real::new(length.unwrap().inner)
    }

    /// Azimuthal angle.
    ///
    /// Returns the azimuthal angle of `self`. Its range is from [0, 2pi).
    ///
    /// ```
    /// # use hebrides::Complex;
    /// let w = Complex::new(3.0, 4.0);
    /// assert_eq!(w.azimuthal().to_radians(), 0.8_f64.asin());
    /// let v = Complex::new(-3.0, 4.0);
    /// assert_eq!(v.azimuthal().to_radians(), std::f64::consts::PI - 0.8_f64.asin());
    /// ```
    pub fn azimuthal(&self) -> Angle {
        if approx_eq(self.imag.inner, 0.0) {
            if self.real > Real::ZERO {
                return Angle::from_radians(0.0);
            }
            return Angle::from_radians(std::f64::consts::PI);
        }
        if approx_eq(self.real.inner, 0.0) {
            if self.imag > Real::ZERO {
                eprintln!("{}, {}", self.real.inner, approx_eq(self.real.inner, 0.0));
                eprintln!("{}, norm: {}", self, self.norm());
                return Angle::from_radians(std::f64::consts::FRAC_PI_2);
            }
            return Angle::from_radians(3.0*std::f64::consts::FRAC_PI_2);
        }
        // quadrant I
        if self.real > Real::ZERO && self.imag > Real::ZERO {
            
            return (self.imag / self.norm()).arcsin().unwrap();
        }
        // quadrant II
        if self.real < Real::ZERO && self.imag > Real::ZERO {
            let angle = std::f64::consts::PI - (self.imag / self.norm()).arcsin().unwrap().to_radians();
            
            return  Angle::from_radians(angle);
        }
        // quadrant III
        if self.real < Real::ZERO && self.imag < Real::ZERO {
            let angle = std::f64::consts::PI + (-self.imag / self.norm()).arcsin().unwrap().to_radians();
            
            return Angle::from_radians(angle);
        }
        // quadrant IV
        if self.real > Real::ZERO && self.imag < Real::ZERO {
            return (self.imag / self.norm()).arcsin().unwrap();
        }
        return Angle::from_radians(0.0);
    }

    /// Returns `self` squared.
    ///
    /// ```
    /// # use hebrides::Complex;
    /// let z = Complex::new(-2.0, 3.0);
    /// assert_eq!(z.squared(), Complex::new(-5.0, -12.0));
    /// ```
    pub fn squared(&self) -> Complex {
        *self * *self
    }

    /// Returns the square root of `self`.
    ///
    /// ```
    /// # use hebrides::Complex;
    /// let z = Complex::new(2.0, 3.0);
    /// assert_eq!(z.sqrt(), Complex::new(1.67414922803554, 0.8959774761298381));
    /// ```
    pub fn sqrt(&self) -> Complex {
        let root_r = self.norm().sqrt().unwrap().to_complex();
        let theta = self.azimuthal().to_radians();
        root_r * Complex::new(((1.0 + theta.cos()) / 2.0).sqrt(), ((1.0 - theta.cos()) / 2.0).sqrt())
        
    }

}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        let reals_eq = approx_eq(self.real.inner, other.real.inner);
        let imags_eq = approx_eq(self.imag.inner, other.imag.inner);
        reals_eq && imags_eq
    }
}

impl Eq for Complex {}

impl Add<Self> for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let real = (self.real + other.real).inner;
        let imag = (self.imag + other.imag).inner;
        Complex::new(real, imag)
    }
}

impl Sub<Self> for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let real = (self.real - other.real).inner;
        let imag = (self.imag - other.imag).inner;
        Complex::new(real, imag)
    }
}

impl Mul<Self> for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let real_part = self.real*other.real - self.imag*other.imag;
        let imag_part = self.real*other.imag + self.imag*other.real;
        Complex::new(real_part.inner, imag_part.inner)
    }
}

impl Div<Self> for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let real_num = self.real*other.real + self.imag*other.imag;
        let imag_num = self.imag*other.real - self.real*other.imag;
        let den = other.real.squared() + other.imag.squared();
        Complex::new((real_num / den).inner, (imag_num / den).inner)
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Complex::new(-self.real.inner, -self.imag.inner)
    }
}

impl TryFrom<Complex> for Real {
    type Error = ConversionError;
    fn try_from(value: Complex) -> Result<Real, ConversionError> {
        if value.is_real() {
            return Ok(value.real);
        }
        Err(ConversionError)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    mod complex {

        use super::*;

        #[test]
        fn division() {
            assert_eq!(Complex::new(4.0, 2.0) / Complex::new(2.0, 0.0), Complex::new(2.0, 1.0));
            assert_eq!(Complex::new(4.0, 2.0) / Complex::new(0.0, 2.0), Complex::new(1.0, -2.0));
        }

        #[test]
        fn multiplication() {
            assert_eq!(Complex::ONE*Complex::new(12.0, 1.0), Complex::new(12.0, 1.0));
            assert_eq!(Complex::new(1.0, 2.0)*Complex::new(3.0, 4.0), Complex::new(-5.0, 10.0));
        }

    }

}
