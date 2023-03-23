use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub enum Num {
    USIZE(usize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

macro_rules! is_positive {
    ($value:expr, $t:ty) => {{
        if $value <= (0 as $t) {
            return false;
        }
        true
    }};
}

macro_rules! powi_without_overflow {
    ($value:expr, $n:expr, $num_variant:expr) => {{
        if $n > 0 {
            let (result, overflowed) = $value.overflowing_pow($n as u32);
            if !overflowed {
                return $num_variant(result);
            }
        }
        Num::F64(($value as f64).powi($n))
    }};
}

impl Num {
    pub fn zero() -> Self {
        Num::U8(0)
    }

    pub fn one() -> Self {
        Num::U8(1)
    }

    pub fn abs(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::USIZE(*value),
            Num::U8(value) => Num::U8(*value),
            Num::U16(value) => Num::U16(*value),
            Num::U32(value) => Num::U32(*value),
            Num::U64(value) => Num::U64(*value),
            Num::I8(value) => Num::U8(value.unsigned_abs()),
            Num::I16(value) => Num::U16(value.unsigned_abs()),
            Num::I32(value) => Num::U32(value.unsigned_abs()),
            Num::I64(value) => Num::U64(value.unsigned_abs()),
            Num::F32(value) => Num::F32(value.abs()),
            Num::F64(value) => Num::F64(value.abs()),
        }
    }

    pub fn powi(&self, n: i32) -> Self {
        match self {
            Num::USIZE(value) => {
                if n > 0 {
                    return Num::USIZE(value.pow(n as u32));
                }
                Num::F64((*value as f64).powi(n))
            }
            Num::U8(value) => powi_without_overflow![*value, n, Num::U8],
            Num::U16(value) => powi_without_overflow![*value, n, Num::U16],
            Num::U32(value) => powi_without_overflow![*value, n, Num::U32],
            Num::U64(value) => powi_without_overflow![*value, n, Num::U64],
            Num::I8(value) => powi_without_overflow![*value, n, Num::I8],
            Num::I16(value) => powi_without_overflow![*value, n, Num::I16],
            Num::I32(value) => powi_without_overflow![*value, n, Num::I32],
            Num::I64(value) => powi_without_overflow![*value, n, Num::I64],
            Num::F32(value) => Num::F64((*value as f64).powi(n)),
            Num::F64(value) => Num::F64((*value).powi(n)),
        }
    }

    pub fn squared(&self) -> Self {
        self.powi(2)
    }

    pub fn powf(&self, n: f64) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).powf(n)),
            Num::U8(value) => Num::F64((*value as f64).powf(n)),
            Num::U16(value) => Num::F64((*value as f64).powf(n)),
            Num::U32(value) => Num::F64((*value as f64).powf(n)),
            Num::U64(value) => Num::F64((*value as f64).powf(n)),
            Num::I8(value) => Num::F64((*value as f64).powf(n)),
            Num::I16(value) => Num::F64((*value as f64).powf(n)),
            Num::I32(value) => Num::F64((*value as f64).powf(n)),
            Num::I64(value) => Num::F64((*value as f64).powf(n)),
            Num::F32(value) => Num::F64((*value as f64).powf(n)),
            Num::F64(value) => Num::F64((*value).powf(n)),
        }
    }

    pub fn sqrt(&self) -> Self {
        self.powf(0.5)
    }

    pub fn exp(&self) -> Self {
        let e = 1.0_f64.exp();
        match self {
            Num::USIZE(value) => Num::F64(e.powf(*value as f64)),
            Num::U8(value) => Num::F64(e.powf(*value as f64)),
            Num::U16(value) => Num::F64(e.powf(*value as f64)),
            Num::U32(value) => Num::F64(e.powf(*value as f64)),
            Num::U64(value) => Num::F64(e.powf(*value as f64)),
            Num::I8(value) => Num::F64(e.powf(*value as f64)),
            Num::I16(value) => Num::F64(e.powf(*value as f64)),
            Num::I32(value) => Num::F64(e.powf(*value as f64)),
            Num::I64(value) => Num::F64(e.powf(*value as f64)),
            Num::F32(value) => Num::F64(e.powf(*value as f64)),
            Num::F64(value) => Num::F64(e.powf(*value)),
        }
    }

    pub fn log(&self, base: f64) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).log(base)),
            Num::U8(value) => Num::F64((*value as f64).log(base)),
            Num::U16(value) => Num::F64((*value as f64).log(base)),
            Num::U32(value) => Num::F64((*value as f64).log(base)),
            Num::U64(value) => Num::F64((*value as f64).log(base)),
            Num::I8(value) => Num::F64((*value as f64).log(base)),
            Num::I16(value) => Num::F64((*value as f64).log(base)),
            Num::I32(value) => Num::F64((*value as f64).log(base)),
            Num::I64(value) => Num::F64((*value as f64).log(base)),
            Num::F32(value) => Num::F64((*value as f64).log(base)),
            Num::F64(value) => Num::F64((*value).log(base)),
        }
    }

    pub fn ln(&self) -> Self {
        let e = 1.0_f64.exp();
        self.log(e)
    }

    pub fn log10(&self) -> Self {
        self.log(10.0)
    }

    pub fn sin(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).sin()),
            Num::U8(value) => Num::F64((*value as f64).sin()),
            Num::U16(value) => Num::F64((*value as f64).sin()),
            Num::U32(value) => Num::F64((*value as f64).sin()),
            Num::U64(value) => Num::F64((*value as f64).sin()),
            Num::I8(value) => Num::F64((*value as f64).sin()),
            Num::I16(value) => Num::F64((*value as f64).sin()),
            Num::I32(value) => Num::F64((*value as f64).sin()),
            Num::I64(value) => Num::F64((*value as f64).sin()),
            Num::F32(value) => Num::F64((*value as f64).sin()),
            Num::F64(value) => Num::F64((*value).sin()),
        }
    }

    pub fn cos(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).cos()),
            Num::U8(value) => Num::F64((*value as f64).cos()),
            Num::U16(value) => Num::F64((*value as f64).cos()),
            Num::U32(value) => Num::F64((*value as f64).cos()),
            Num::U64(value) => Num::F64((*value as f64).cos()),
            Num::I8(value) => Num::F64((*value as f64).cos()),
            Num::I16(value) => Num::F64((*value as f64).cos()),
            Num::I32(value) => Num::F64((*value as f64).cos()),
            Num::I64(value) => Num::F64((*value as f64).cos()),
            Num::F32(value) => Num::F64((*value as f64).cos()),
            Num::F64(value) => Num::F64((*value).cos()),
        }
    }

    pub fn tan(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).tan()),
            Num::U8(value) => Num::F64((*value as f64).tan()),
            Num::U16(value) => Num::F64((*value as f64).tan()),
            Num::U32(value) => Num::F64((*value as f64).tan()),
            Num::U64(value) => Num::F64((*value as f64).tan()),
            Num::I8(value) => Num::F64((*value as f64).tan()),
            Num::I16(value) => Num::F64((*value as f64).tan()),
            Num::I32(value) => Num::F64((*value as f64).tan()),
            Num::I64(value) => Num::F64((*value as f64).tan()),
            Num::F32(value) => Num::F64((*value as f64).tan()),
            Num::F64(value) => Num::F64((*value).tan()),
        }
    }

    pub fn arcsin(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).asin()),
            Num::U8(value) => Num::F64((*value as f64).asin()),
            Num::U16(value) => Num::F64((*value as f64).asin()),
            Num::U32(value) => Num::F64((*value as f64).asin()),
            Num::U64(value) => Num::F64((*value as f64).asin()),
            Num::I8(value) => Num::F64((*value as f64).asin()),
            Num::I16(value) => Num::F64((*value as f64).asin()),
            Num::I32(value) => Num::F64((*value as f64).asin()),
            Num::I64(value) => Num::F64((*value as f64).asin()),
            Num::F32(value) => Num::F64((*value as f64).asin()),
            Num::F64(value) => Num::F64((*value).asin()),
        }
    }

    pub fn arccos(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).acos()),
            Num::U8(value) => Num::F64((*value as f64).acos()),
            Num::U16(value) => Num::F64((*value as f64).acos()),
            Num::U32(value) => Num::F64((*value as f64).acos()),
            Num::U64(value) => Num::F64((*value as f64).acos()),
            Num::I8(value) => Num::F64((*value as f64).acos()),
            Num::I16(value) => Num::F64((*value as f64).acos()),
            Num::I32(value) => Num::F64((*value as f64).acos()),
            Num::I64(value) => Num::F64((*value as f64).acos()),
            Num::F32(value) => Num::F64((*value as f64).acos()),
            Num::F64(value) => Num::F64((*value).acos()),
        }
    }

    pub fn arctan(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).atan()),
            Num::U8(value) => Num::F64((*value as f64).atan()),
            Num::U16(value) => Num::F64((*value as f64).atan()),
            Num::U32(value) => Num::F64((*value as f64).atan()),
            Num::U64(value) => Num::F64((*value as f64).atan()),
            Num::I8(value) => Num::F64((*value as f64).atan()),
            Num::I16(value) => Num::F64((*value as f64).atan()),
            Num::I32(value) => Num::F64((*value as f64).atan()),
            Num::I64(value) => Num::F64((*value as f64).atan()),
            Num::F32(value) => Num::F64((*value as f64).atan()),
            Num::F64(value) => Num::F64((*value).atan()),
        }
    }

    pub fn arctan2(_x: Self, _y: Self) -> Self {
        unimplemented!()
    }

    pub fn sinh(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).sinh()),
            Num::U8(value) => Num::F64((*value as f64).sinh()),
            Num::U16(value) => Num::F64((*value as f64).sinh()),
            Num::U32(value) => Num::F64((*value as f64).sinh()),
            Num::U64(value) => Num::F64((*value as f64).sinh()),
            Num::I8(value) => Num::F64((*value as f64).sinh()),
            Num::I16(value) => Num::F64((*value as f64).sinh()),
            Num::I32(value) => Num::F64((*value as f64).sinh()),
            Num::I64(value) => Num::F64((*value as f64).sinh()),
            Num::F32(value) => Num::F64((*value as f64).sinh()),
            Num::F64(value) => Num::F64((*value).sinh()),
        }
    }

    pub fn cosh(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).cosh()),
            Num::U8(value) => Num::F64((*value as f64).cosh()),
            Num::U16(value) => Num::F64((*value as f64).cosh()),
            Num::U32(value) => Num::F64((*value as f64).cosh()),
            Num::U64(value) => Num::F64((*value as f64).cosh()),
            Num::I8(value) => Num::F64((*value as f64).cosh()),
            Num::I16(value) => Num::F64((*value as f64).cosh()),
            Num::I32(value) => Num::F64((*value as f64).cosh()),
            Num::I64(value) => Num::F64((*value as f64).cosh()),
            Num::F32(value) => Num::F64((*value as f64).cosh()),
            Num::F64(value) => Num::F64((*value).cosh()),
        }
    }

    pub fn tanh(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).tanh()),
            Num::U8(value) => Num::F64((*value as f64).tanh()),
            Num::U16(value) => Num::F64((*value as f64).tanh()),
            Num::U32(value) => Num::F64((*value as f64).tanh()),
            Num::U64(value) => Num::F64((*value as f64).tanh()),
            Num::I8(value) => Num::F64((*value as f64).tanh()),
            Num::I16(value) => Num::F64((*value as f64).tanh()),
            Num::I32(value) => Num::F64((*value as f64).tanh()),
            Num::I64(value) => Num::F64((*value as f64).tanh()),
            Num::F32(value) => Num::F64((*value as f64).tanh()),
            Num::F64(value) => Num::F64((*value).tanh()),
        }
    }

    pub fn arcsinh(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).asinh()),
            Num::U8(value) => Num::F64((*value as f64).asinh()),
            Num::U16(value) => Num::F64((*value as f64).asinh()),
            Num::U32(value) => Num::F64((*value as f64).asinh()),
            Num::U64(value) => Num::F64((*value as f64).asinh()),
            Num::I8(value) => Num::F64((*value as f64).asinh()),
            Num::I16(value) => Num::F64((*value as f64).asinh()),
            Num::I32(value) => Num::F64((*value as f64).asinh()),
            Num::I64(value) => Num::F64((*value as f64).asinh()),
            Num::F32(value) => Num::F64((*value as f64).asinh()),
            Num::F64(value) => Num::F64((*value).asinh()),
        }
    }

    pub fn arccosh(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).acosh()),
            Num::U8(value) => Num::F64((*value as f64).acosh()),
            Num::U16(value) => Num::F64((*value as f64).acosh()),
            Num::U32(value) => Num::F64((*value as f64).acosh()),
            Num::U64(value) => Num::F64((*value as f64).acosh()),
            Num::I8(value) => Num::F64((*value as f64).acosh()),
            Num::I16(value) => Num::F64((*value as f64).acosh()),
            Num::I32(value) => Num::F64((*value as f64).acosh()),
            Num::I64(value) => Num::F64((*value as f64).acosh()),
            Num::F32(value) => Num::F64((*value as f64).acosh()),
            Num::F64(value) => Num::F64((*value).acosh()),
        }
    }

    pub fn arctanh(&self) -> Self {
        match self {
            Num::USIZE(value) => Num::F64((*value as f64).atanh()),
            Num::U8(value) => Num::F64((*value as f64).atanh()),
            Num::U16(value) => Num::F64((*value as f64).atanh()),
            Num::U32(value) => Num::F64((*value as f64).atanh()),
            Num::U64(value) => Num::F64((*value as f64).atanh()),
            Num::I8(value) => Num::F64((*value as f64).atanh()),
            Num::I16(value) => Num::F64((*value as f64).atanh()),
            Num::I32(value) => Num::F64((*value as f64).atanh()),
            Num::I64(value) => Num::F64((*value as f64).atanh()),
            Num::F32(value) => Num::F64((*value as f64).atanh()),
            Num::F64(value) => Num::F64((*value).atanh()),
        }
    }

    pub fn is_zero(&self) -> bool {
        *self == Num::zero()
    }

    pub fn positive(&self) -> bool {
        match self {
            Num::USIZE(value) => *value != 0_usize,
            Num::U8(value) => *value != 0_u8,
            Num::U16(value) => *value != 0_u16,
            Num::U32(value) => *value != 0_u32,
            Num::U64(value) => *value != 0_u64,
            Num::I8(value) => is_positive![*value, i8],
            Num::I16(value) => is_positive![*value, i16],
            Num::I32(value) => is_positive![*value, i32],
            Num::I64(value) => is_positive![*value, i64],
            Num::F32(value) => is_positive![*value, f32],
            Num::F64(value) => is_positive![*value, f64],
        }
    }

    pub fn negative(&self) -> bool {
        if self.is_zero() {
            return false;
        }
        !self.positive()
    }
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::USIZE(value) => write!(f, "{}", value),
            Num::U8(value) => write!(f, "{}", value),
            Num::U16(value) => write!(f, "{}", value),
            Num::U32(value) => write!(f, "{}", value),
            Num::U64(value) => write!(f, "{}", value),
            Num::I8(value) => write!(f, "{}", value),
            Num::I16(value) => write!(f, "{}", value),
            Num::I32(value) => write!(f, "{}", value),
            Num::I64(value) => write!(f, "{}", value),
            Num::F32(value) => write!(f, "{}", value),
            Num::F64(value) => write!(f, "{}", value),
        }
    }
}

macro_rules! impl_num_from_primitive {
    ($primitive:ty, $variant:expr) => {
        impl From<$primitive> for Num {
            fn from(value: $primitive) -> Self {
                $variant(value)
            }
        }
    };
}

impl_num_from_primitive![usize, Num::USIZE];

impl_num_from_primitive![u8, Num::U8];
impl_num_from_primitive![u16, Num::U16];
impl_num_from_primitive![u32, Num::U32];
impl_num_from_primitive![u64, Num::U64];

impl_num_from_primitive![i8, Num::I8];
impl_num_from_primitive![i16, Num::I16];
impl_num_from_primitive![i32, Num::I32];
impl_num_from_primitive![i64, Num::I64];

impl_num_from_primitive![f32, Num::F32];
impl_num_from_primitive![f64, Num::F64];

macro_rules! assert_almost_equal {
    ($a:expr, $b:expr, $t:ty) => {{
        if $a as $t == $b as $t {
            return true;
        }
        if ($a as $t - $b as $t).abs() < <$t>::EPSILON {
            return true;
        }
        false
    }};
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Num::USIZE(self_value) => match other {
                Num::USIZE(other_value) => *self_value == *other_value,
                Num::U8(other_value) => *self_value == *other_value as usize,
                Num::U16(other_value) => *self_value == *other_value as usize,
                Num::U32(other_value) => *self_value == *other_value as usize,
                Num::U64(other_value) => *self_value == *other_value as usize,
                Num::I8(other_value) => *self_value as i64 == *other_value as i64,
                Num::I16(other_value) => *self_value as i64 == *other_value as i64,
                Num::I32(other_value) => *self_value as i64 == *other_value as i64,
                Num::I64(other_value) => *self_value as i64 == *other_value,
                Num::F32(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::F64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
            },
            Num::U8(self_value) => match other {
                Num::USIZE(other_value) => *self_value as usize == *other_value,
                Num::U8(other_value) => *self_value == *other_value,
                Num::U16(other_value) => *self_value as u16 == *other_value,
                Num::U32(other_value) => *self_value as u32 == *other_value,
                Num::U64(other_value) => *self_value as u64 == *other_value,
                Num::I8(other_value) => *self_value as i8 == *other_value,
                Num::I16(other_value) => *self_value as i16 == *other_value,
                Num::I32(other_value) => *self_value as i32 == *other_value,
                Num::I64(other_value) => *self_value as i64 == *other_value,
                Num::F32(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::F64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
            },
            Num::U16(self_value) => match other {
                Num::USIZE(other_value) => *self_value as usize == *other_value,
                Num::U8(other_value) => *self_value == *other_value as u16,
                Num::U16(other_value) => *self_value == *other_value,
                Num::U32(other_value) => *self_value as u32 == *other_value,
                Num::U64(other_value) => *self_value as u64 == *other_value,
                Num::I8(other_value) => *self_value as i16 == *other_value as i16,
                Num::I16(other_value) => *self_value as i16 == *other_value,
                Num::I32(other_value) => *self_value as i32 == *other_value,
                Num::I64(other_value) => *self_value as i64 == *other_value,
                Num::F32(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::F64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
            },
            Num::U32(self_value) => match other {
                Num::USIZE(other_value) => *self_value as usize == *other_value,
                Num::U8(other_value) => *self_value == *other_value as u32,
                Num::U16(other_value) => *self_value == *other_value as u32,
                Num::U32(other_value) => *self_value == *other_value,
                Num::U64(other_value) => *self_value as u64 == *other_value,
                Num::I8(other_value) => *self_value as i32 == *other_value as i32,
                Num::I16(other_value) => *self_value as i32 == *other_value as i32,
                Num::I32(other_value) => *self_value as i32 == *other_value,
                Num::I64(other_value) => *self_value as i64 == *other_value,
                Num::F32(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::F64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
            },
            Num::U64(self_value) => match other {
                Num::USIZE(other_value) => *self_value as usize == *other_value,
                Num::U8(other_value) => *self_value == *other_value as u64,
                Num::U16(other_value) => *self_value == *other_value as u64,
                Num::U32(other_value) => *self_value == *other_value as u64,
                Num::U64(other_value) => *self_value == *other_value,
                Num::I8(other_value) => *self_value as i64 == *other_value as i64,
                Num::I16(other_value) => *self_value as i64 == *other_value as i64,
                Num::I32(other_value) => *self_value as i64 == *other_value as i64,
                Num::I64(other_value) => *self_value as i64 == *other_value,
                Num::F32(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::F64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
            },
            Num::I8(self_value) => match other {
                Num::USIZE(other_value) => *self_value as i64 == *other_value as i64,
                Num::U8(other_value) => *self_value == *other_value as i8,
                Num::U16(other_value) => *self_value as i16 == *other_value as i16,
                Num::U32(other_value) => *self_value as i32 == *other_value as i32,
                Num::U64(other_value) => *self_value as i64 == *other_value as i64,
                Num::I8(other_value) => *self_value == *other_value,
                Num::I16(other_value) => *self_value as i16 == *other_value,
                Num::I32(other_value) => *self_value as i32 == *other_value,
                Num::I64(other_value) => *self_value as i64 == *other_value,
                Num::F32(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::F64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
            },
            Num::I16(self_value) => match other {
                Num::USIZE(other_value) => *self_value as i64 == *other_value as i64,
                Num::U8(other_value) => *self_value == *other_value as i16,
                Num::U16(other_value) => *self_value == *other_value as i16,
                Num::U32(other_value) => *self_value as i32 == *other_value as i32,
                Num::U64(other_value) => *self_value as i64 == *other_value as i64,
                Num::I8(other_value) => *self_value == *other_value as i16,
                Num::I16(other_value) => *self_value == *other_value,
                Num::I32(other_value) => *self_value as i32 == *other_value,
                Num::I64(other_value) => *self_value as i64 == *other_value,
                Num::F32(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::F64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
            },
            Num::I32(self_value) => match other {
                Num::USIZE(other_value) => *self_value as i64 == *other_value as i64,
                Num::U8(other_value) => *self_value == *other_value as i32,
                Num::U16(other_value) => *self_value == *other_value as i32,
                Num::U32(other_value) => *self_value == *other_value as i32,
                Num::U64(other_value) => *self_value as i64 == *other_value as i64,
                Num::I8(other_value) => *self_value == *other_value as i32,
                Num::I16(other_value) => *self_value == *other_value as i32,
                Num::I32(other_value) => *self_value == *other_value,
                Num::I64(other_value) => *self_value as i64 == *other_value,
                Num::F32(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::F64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
            },
            Num::I64(self_value) => match other {
                Num::USIZE(other_value) => *self_value == *other_value as i64,
                Num::U8(other_value) => *self_value == *other_value as i64,
                Num::U16(other_value) => *self_value == *other_value as i64,
                Num::U32(other_value) => *self_value == *other_value as i64,
                Num::U64(other_value) => *self_value == *other_value as i64,
                Num::I8(other_value) => *self_value == *other_value as i64,
                Num::I16(other_value) => *self_value == *other_value as i64,
                Num::I32(other_value) => *self_value == *other_value as i64,
                Num::I64(other_value) => *self_value == *other_value,
                Num::F32(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::F64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
            },
            Num::F32(self_value) => match other {
                Num::USIZE(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::U8(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::U16(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::U32(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::U64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::I8(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::I16(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::I32(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::I64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::F32(other_value) => assert_almost_equal![*self_value, *other_value, f32],
                Num::F64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
            },
            Num::F64(self_value) => match other {
                Num::USIZE(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::U8(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::U16(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::U32(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::U64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::I8(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::I16(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::I32(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::I64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::F32(other_value) => assert_almost_equal![*self_value, *other_value, f64],
                Num::F64(other_value) => assert_almost_equal![*self_value, *other_value, f64],
            },
        }
    }
}

impl Eq for Num {}

impl Add<Self> for Num {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Num::USIZE(self_value) => match other {
                Num::USIZE(other_value) => Num::U64(self_value as u64 + other_value as u64),
                Num::U8(other_value) => Num::U64(self_value as u64 + other_value as u64),
                Num::U16(other_value) => Num::U64(self_value as u64 + other_value as u64),
                Num::U32(other_value) => Num::U64(self_value as u64 + other_value as u64),
                Num::U64(other_value) => Num::U64(self_value as u64 + other_value),
                Num::I8(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I16(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I32(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 + other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 + other_value),
            },
            Num::U8(self_value) => match other {
                Num::USIZE(other_value) => Num::U64(self_value as u64 + other_value as u64),
                Num::U8(other_value) => Num::U16(self_value as u16 + other_value as u16),
                Num::U16(other_value) => Num::U32(self_value as u32 + other_value as u32),
                Num::U32(other_value) => Num::U64(self_value as u64 + other_value as u64),
                Num::U64(other_value) => Num::U64(self_value as u64 + other_value),
                Num::I8(other_value) => Num::I16(self_value as i16 + other_value as i16),
                Num::I16(other_value) => Num::I32(self_value as i32 + other_value as i32),
                Num::I32(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 + other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 + other_value),
            },
            Num::U16(self_value) => match other {
                Num::USIZE(other_value) => Num::U64(self_value as u64 + other_value as u64),
                Num::U8(other_value) => Num::U32(self_value as u32 + other_value as u32),
                Num::U16(other_value) => Num::U32(self_value as u32 + other_value as u32),
                Num::U32(other_value) => Num::U64(self_value as u64 + other_value as u64),
                Num::U64(other_value) => Num::U64(self_value as u64 + other_value),
                Num::I8(other_value) => Num::I32(self_value as i32 + other_value as i32),
                Num::I16(other_value) => Num::I32(self_value as i32 + other_value as i32),
                Num::I32(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 + other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 + other_value),
            },
            Num::U32(self_value) => match other {
                Num::USIZE(other_value) => Num::U64(self_value as u64 + other_value as u64),
                Num::U8(other_value) => Num::U64(self_value as u64 + other_value as u64),
                Num::U16(other_value) => Num::U64(self_value as u64 + other_value as u64),
                Num::U32(other_value) => Num::U64(self_value as u64 + other_value as u64),
                Num::U64(other_value) => Num::U64(self_value as u64 + other_value),
                Num::I8(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I16(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I32(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 + other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 + other_value),
            },
            Num::U64(self_value) => match other {
                Num::USIZE(other_value) => Num::U64(self_value + other_value as u64),
                Num::U8(other_value) => Num::U64(self_value + other_value as u64),
                Num::U16(other_value) => Num::U64(self_value + other_value as u64),
                Num::U32(other_value) => Num::U64(self_value + other_value as u64),
                Num::U64(other_value) => Num::U64(self_value + other_value),
                Num::I8(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I16(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I32(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 + other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 + other_value),
            },
            Num::I8(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::U8(other_value) => Num::I16(self_value as i16 + other_value as i16),
                Num::U16(other_value) => Num::I32(self_value as i32 + other_value as i32),
                Num::U32(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::U64(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I8(other_value) => Num::I16(self_value as i16 + other_value as i16),
                Num::I16(other_value) => Num::I32(self_value as i32 + other_value as i32),
                Num::I32(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 + other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 + other_value),
            },
            Num::I16(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::U8(other_value) => Num::I32(self_value as i32 + other_value as i32),
                Num::U16(other_value) => Num::I32(self_value as i32 + other_value as i32),
                Num::U32(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::U64(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I8(other_value) => Num::I32(self_value as i32 + other_value as i32),
                Num::I16(other_value) => Num::I32(self_value as i32 + other_value as i32),
                Num::I32(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 + other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 + other_value),
            },
            Num::I32(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::U8(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::U16(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::U32(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::U64(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I8(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I16(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I32(other_value) => Num::I64(self_value as i64 + other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 + other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 + other_value),
            },
            Num::I64(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value + other_value as i64),
                Num::U8(other_value) => Num::I64(self_value + other_value as i64),
                Num::U16(other_value) => Num::I64(self_value + other_value as i64),
                Num::U32(other_value) => Num::I64(self_value + other_value as i64),
                Num::U64(other_value) => Num::I64(self_value + other_value as i64),
                Num::I8(other_value) => Num::I64(self_value + other_value as i64),
                Num::I16(other_value) => Num::I64(self_value + other_value as i64),
                Num::I32(other_value) => Num::I64(self_value + other_value as i64),
                Num::I64(other_value) => Num::I64(self_value + other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 + other_value),
            },
            Num::F32(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::U8(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::U16(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::U32(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::U64(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::I8(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::I16(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::I32(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::I64(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::F32(other_value) => Num::F64(self_value as f64 + other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 + other_value),
            },
            Num::F64(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value + other_value as f64),
                Num::U8(other_value) => Num::F64(self_value + other_value as f64),
                Num::U16(other_value) => Num::F64(self_value + other_value as f64),
                Num::U32(other_value) => Num::F64(self_value + other_value as f64),
                Num::U64(other_value) => Num::F64(self_value + other_value as f64),
                Num::I8(other_value) => Num::F64(self_value + other_value as f64),
                Num::I16(other_value) => Num::F64(self_value + other_value as f64),
                Num::I32(other_value) => Num::F64(self_value + other_value as f64),
                Num::I64(other_value) => Num::F64(self_value + other_value as f64),
                Num::F32(other_value) => Num::F64(self_value + other_value as f64),
                Num::F64(other_value) => Num::F64(self_value + other_value),
            },
        }
    }
}

impl Sub<Self> for Num {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match self {
            Num::USIZE(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U8(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U16(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U32(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U64(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I8(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I16(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I32(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 - other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 - other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 - other_value),
            },
            Num::U8(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U8(other_value) => Num::I8(self_value as i8 - other_value as i8),
                Num::U16(other_value) => Num::I16(self_value as i16 - other_value as i16),
                Num::U32(other_value) => Num::I32(self_value as i32 - other_value as i32),
                Num::U64(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I8(other_value) => Num::I8(self_value as i8 - other_value),
                Num::I16(other_value) => Num::I16(self_value as i16 - other_value),
                Num::I32(other_value) => Num::I32(self_value as i32 - other_value),
                Num::I64(other_value) => Num::I64(self_value as i64 - other_value),
                Num::F32(other_value) => Num::F32(self_value as f32 - other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 - other_value),
            },
            Num::U16(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U8(other_value) => Num::I16(self_value as i16 - other_value as i16),
                Num::U16(other_value) => Num::I16(self_value as i16 - other_value as i16),
                Num::U32(other_value) => Num::I32(self_value as i32 - other_value as i32),
                Num::U64(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I8(other_value) => Num::I16(self_value as i16 - other_value as i16),
                Num::I16(other_value) => Num::I16(self_value as i16 - other_value),
                Num::I32(other_value) => Num::I32(self_value as i32 - other_value),
                Num::I64(other_value) => Num::I64(self_value as i64 - other_value),
                Num::F32(other_value) => Num::F32(self_value as f32 - other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 - other_value),
            },
            Num::U32(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U8(other_value) => Num::I32(self_value as i32 - other_value as i32),
                Num::U16(other_value) => Num::I32(self_value as i32 - other_value as i32),
                Num::U32(other_value) => Num::I32(self_value as i32 - other_value as i32),
                Num::U64(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I8(other_value) => Num::I32(self_value as i32 - other_value as i32),
                Num::I16(other_value) => Num::I32(self_value as i32 - other_value as i32),
                Num::I32(other_value) => Num::I32(self_value as i32 - other_value),
                Num::I64(other_value) => Num::I64(self_value as i64 - other_value),
                Num::F32(other_value) => Num::F32(self_value as f32 - other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 - other_value),
            },
            Num::U64(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U8(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U16(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U32(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U64(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I8(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I16(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I32(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 - other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 - other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 - other_value),
            },
            Num::I8(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U8(other_value) => Num::I8(self_value - other_value as i8),
                Num::U16(other_value) => Num::I16(self_value as i16 - other_value as i16),
                Num::U32(other_value) => Num::I32(self_value as i32 - other_value as i32),
                Num::U64(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I8(other_value) => Num::I8(self_value - other_value),
                Num::I16(other_value) => Num::I16(self_value as i16 - other_value),
                Num::I32(other_value) => Num::I32(self_value as i32 - other_value),
                Num::I64(other_value) => Num::I64(self_value as i64 - other_value),
                Num::F32(other_value) => Num::F32(self_value as f32 - other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 - other_value),
            },
            Num::I16(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U8(other_value) => Num::I16(self_value - other_value as i16),
                Num::U16(other_value) => Num::I16(self_value - other_value as i16),
                Num::U32(other_value) => Num::I32(self_value as i32 - other_value as i32),
                Num::U64(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I8(other_value) => Num::I16(self_value - other_value as i16),
                Num::I16(other_value) => Num::I16(self_value - other_value),
                Num::I32(other_value) => Num::I32(self_value as i32 - other_value),
                Num::I64(other_value) => Num::I64(self_value as i64 - other_value),
                Num::F32(other_value) => Num::F32(self_value as f32 - other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 - other_value),
            },
            Num::I32(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::U8(other_value) => Num::I32(self_value - other_value as i32),
                Num::U16(other_value) => Num::I32(self_value - other_value as i32),
                Num::U32(other_value) => Num::I32(self_value - other_value as i32),
                Num::U64(other_value) => Num::I64(self_value as i64 - other_value as i64),
                Num::I8(other_value) => Num::I32(self_value - other_value as i32),
                Num::I16(other_value) => Num::I32(self_value - other_value as i32),
                Num::I32(other_value) => Num::I32(self_value - other_value),
                Num::I64(other_value) => Num::I64(self_value as i64 - other_value),
                Num::F32(other_value) => Num::F32(self_value as f32 - other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 - other_value),
            },
            Num::I64(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value - other_value as i64),
                Num::U8(other_value) => Num::I64(self_value - other_value as i64),
                Num::U16(other_value) => Num::I64(self_value - other_value as i64),
                Num::U32(other_value) => Num::I64(self_value - other_value as i64),
                Num::U64(other_value) => Num::I64(self_value - other_value as i64),
                Num::I8(other_value) => Num::I64(self_value - other_value as i64),
                Num::I16(other_value) => Num::I64(self_value - other_value as i64),
                Num::I32(other_value) => Num::I64(self_value - other_value as i64),
                Num::I64(other_value) => Num::I64(self_value - other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 - other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 - other_value),
            },
            Num::F32(self_value) => match other {
                Num::USIZE(other_value) => Num::F32(self_value - other_value as f32),
                Num::U8(other_value) => Num::F32(self_value - other_value as f32),
                Num::U16(other_value) => Num::F32(self_value - other_value as f32),
                Num::U32(other_value) => Num::F32(self_value - other_value as f32),
                Num::U64(other_value) => Num::F32(self_value - other_value as f32),
                Num::I8(other_value) => Num::F32(self_value - other_value as f32),
                Num::I16(other_value) => Num::F32(self_value - other_value as f32),
                Num::I32(other_value) => Num::F32(self_value - other_value as f32),
                Num::I64(other_value) => Num::F32(self_value - other_value as f32),
                Num::F32(other_value) => Num::F32(self_value - other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 - other_value),
            },
            Num::F64(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value - other_value as f64),
                Num::U8(other_value) => Num::F64(self_value - other_value as f64),
                Num::U16(other_value) => Num::F64(self_value - other_value as f64),
                Num::U32(other_value) => Num::F64(self_value - other_value as f64),
                Num::U64(other_value) => Num::F64(self_value - other_value as f64),
                Num::I8(other_value) => Num::F64(self_value - other_value as f64),
                Num::I16(other_value) => Num::F64(self_value - other_value as f64),
                Num::I32(other_value) => Num::F64(self_value - other_value as f64),
                Num::I64(other_value) => Num::F64(self_value - other_value as f64),
                Num::F32(other_value) => Num::F64(self_value - other_value as f64),
                Num::F64(other_value) => Num::F64(self_value - other_value),
            },
        }
    }
}

impl Mul<Self> for Num {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match self {
            Num::USIZE(self_value) => match other {
                Num::USIZE(other_value) => Num::U64(self_value as u64 * other_value as u64),
                Num::U8(other_value) => Num::U64(self_value as u64 * other_value as u64),
                Num::U16(other_value) => Num::U64(self_value as u64 * other_value as u64),
                Num::U32(other_value) => Num::U64(self_value as u64 * other_value as u64),
                Num::U64(other_value) => Num::U64(self_value as u64 * other_value),
                Num::I8(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I16(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I32(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 * other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 * other_value),
            },
            Num::U8(self_value) => match other {
                Num::USIZE(other_value) => Num::U64(self_value as u64 * other_value as u64),
                Num::U8(other_value) => Num::U16(self_value as u16 * other_value as u16),
                Num::U16(other_value) => Num::U32(self_value as u32 * other_value as u32),
                Num::U32(other_value) => Num::U64(self_value as u64 * other_value as u64),
                Num::U64(other_value) => Num::U64(self_value as u64 * other_value),
                Num::I8(other_value) => Num::I16(self_value as i16 * other_value as i16),
                Num::I16(other_value) => Num::I32(self_value as i32 * other_value as i32),
                Num::I32(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 * other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 * other_value),
            },
            Num::U16(self_value) => match other {
                Num::USIZE(other_value) => Num::U64(self_value as u64 * other_value as u64),
                Num::U8(other_value) => Num::U32(self_value as u32 * other_value as u32),
                Num::U16(other_value) => Num::U32(self_value as u32 * other_value as u32),
                Num::U32(other_value) => Num::U64(self_value as u64 * other_value as u64),
                Num::U64(other_value) => Num::U64(self_value as u64 * other_value),
                Num::I8(other_value) => Num::I32(self_value as i32 * other_value as i32),
                Num::I16(other_value) => Num::I32(self_value as i32 * other_value as i32),
                Num::I32(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 * other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 * other_value),
            },
            Num::U32(self_value) => match other {
                Num::USIZE(other_value) => Num::U64(self_value as u64 * other_value as u64),
                Num::U8(other_value) => Num::U64(self_value as u64 * other_value as u64),
                Num::U16(other_value) => Num::U64(self_value as u64 * other_value as u64),
                Num::U32(other_value) => Num::U64(self_value as u64 * other_value as u64),
                Num::U64(other_value) => Num::U64(self_value as u64 * other_value),
                Num::I8(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I16(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I32(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 * other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 * other_value),
            },
            Num::U64(self_value) => match other {
                Num::USIZE(other_value) => Num::U64(self_value * other_value as u64),
                Num::U8(other_value) => Num::U64(self_value * other_value as u64),
                Num::U16(other_value) => Num::U64(self_value * other_value as u64),
                Num::U32(other_value) => Num::U64(self_value * other_value as u64),
                Num::U64(other_value) => Num::U64(self_value * other_value),
                Num::I8(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I16(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I32(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 * other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 * other_value),
            },
            Num::I8(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::U8(other_value) => Num::I16(self_value as i16 * other_value as i16),
                Num::U16(other_value) => Num::I32(self_value as i32 * other_value as i32),
                Num::U32(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::U64(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I8(other_value) => Num::I16(self_value as i16 * other_value as i16),
                Num::I16(other_value) => Num::I32(self_value as i32 * other_value as i32),
                Num::I32(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 * other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 * other_value),
            },
            Num::I16(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::U8(other_value) => Num::I32(self_value as i32 * other_value as i32),
                Num::U16(other_value) => Num::I32(self_value as i32 * other_value as i32),
                Num::U32(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::U64(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I8(other_value) => Num::I32(self_value as i32 * other_value as i32),
                Num::I16(other_value) => Num::I32(self_value as i32 * other_value as i32),
                Num::I32(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 * other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 * other_value),
            },
            Num::I32(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::U8(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::U16(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::U32(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::U64(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I8(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I16(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I32(other_value) => Num::I64(self_value as i64 * other_value as i64),
                Num::I64(other_value) => Num::I64(self_value as i64 * other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 * other_value),
            },
            Num::I64(self_value) => match other {
                Num::USIZE(other_value) => Num::I64(self_value * other_value as i64),
                Num::U8(other_value) => Num::I64(self_value * other_value as i64),
                Num::U16(other_value) => Num::I64(self_value * other_value as i64),
                Num::U32(other_value) => Num::I64(self_value * other_value as i64),
                Num::U64(other_value) => Num::I64(self_value * other_value as i64),
                Num::I8(other_value) => Num::I64(self_value * other_value as i64),
                Num::I16(other_value) => Num::I64(self_value * other_value as i64),
                Num::I32(other_value) => Num::I64(self_value * other_value as i64),
                Num::I64(other_value) => Num::I64(self_value * other_value),
                Num::F32(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 * other_value),
            },
            Num::F32(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::U8(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::U16(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::U32(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::U64(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::I8(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::I16(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::I32(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::I64(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::F32(other_value) => Num::F64(self_value as f64 * other_value as f64),
                Num::F64(other_value) => Num::F64(self_value as f64 * other_value),
            },
            Num::F64(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value * other_value as f64),
                Num::U8(other_value) => Num::F64(self_value * other_value as f64),
                Num::U16(other_value) => Num::F64(self_value * other_value as f64),
                Num::U32(other_value) => Num::F64(self_value * other_value as f64),
                Num::U64(other_value) => Num::F64(self_value * other_value as f64),
                Num::I8(other_value) => Num::F64(self_value * other_value as f64),
                Num::I16(other_value) => Num::F64(self_value * other_value as f64),
                Num::I32(other_value) => Num::F64(self_value * other_value as f64),
                Num::I64(other_value) => Num::F64(self_value * other_value as f64),
                Num::F32(other_value) => Num::F64(self_value * other_value as f64),
                Num::F64(other_value) => Num::F64(self_value * other_value),
            },
        }
    }
}

impl Div<Self> for Num {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match self {
            Num::USIZE(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::U8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::I8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::F32(other_value) => Num::F32(self_value as f32 / other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 / other_value),
            },
            Num::U8(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::U8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::I8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::F32(other_value) => Num::F32(self_value as f32 / other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 / other_value),
            },
            Num::U16(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::U8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::I8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::F32(other_value) => Num::F32(self_value as f32 / other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 / other_value),
            },
            Num::U32(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::U8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::I8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::F32(other_value) => Num::F32(self_value as f32 / other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 / other_value),
            },
            Num::U64(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::U8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::I8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::F32(other_value) => Num::F32(self_value as f32 / other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 / other_value),
            },
            Num::I8(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::U8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::I8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::F32(other_value) => Num::F32(self_value as f32 / other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 / other_value),
            },
            Num::I16(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::U8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::I8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::F32(other_value) => Num::F32(self_value as f32 / other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 / other_value),
            },
            Num::I32(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::U8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::I8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::F32(other_value) => Num::F32(self_value as f32 / other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 / other_value),
            },
            Num::I64(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::U8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::I8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::F32(other_value) => Num::F32(self_value as f32 / other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 / other_value),
            },
            Num::F32(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::U8(other_value) => Num::F32(self_value / other_value as f32),
                Num::U16(other_value) => Num::F32(self_value / other_value as f32),
                Num::U32(other_value) => Num::F32(self_value / other_value as f32),
                Num::U64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::I8(other_value) => Num::F32(self_value / other_value as f32),
                Num::I16(other_value) => Num::F32(self_value / other_value as f32),
                Num::I32(other_value) => Num::F32(self_value / other_value as f32),
                Num::I64(other_value) => Num::F64(self_value as f64 / other_value as f64),
                Num::F32(other_value) => Num::F32(self_value / other_value),
                Num::F64(other_value) => Num::F64(self_value as f64 / other_value),
            },
            Num::F64(self_value) => match other {
                Num::USIZE(other_value) => Num::F64(self_value / other_value as f64),
                Num::U8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::U64(other_value) => Num::F64(self_value / other_value as f64),
                Num::I8(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I16(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I32(other_value) => Num::F32(self_value as f32 / other_value as f32),
                Num::I64(other_value) => Num::F64(self_value / other_value as f64),
                Num::F32(other_value) => Num::F32(self_value as f32 / other_value),
                Num::F64(other_value) => Num::F64(self_value / other_value),
            },
        }
    }
}

impl Neg for Num {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Num::USIZE(value) => Num::I64(-(value as i64)),
            Num::U8(value) => Num::I8(-(value as i8)),
            Num::U16(value) => Num::I16(-(value as i16)),
            Num::U32(value) => Num::I32(-(value as i32)),
            Num::U64(value) => Num::I64(-(value as i64)),
            Num::I8(value) => Num::I8(-value),
            Num::I16(value) => Num::I16(-value),
            Num::I32(value) => Num::I32(-value),
            Num::I64(value) => Num::I64(-value),
            Num::F32(value) => Num::F32(-value),
            Num::F64(value) => Num::F64(-value),
        }
    }
}

fn cmp_numeric<U: PartialOrd>(a: U, b: U) -> Option<std::cmp::Ordering> {
    if a < b {
        return Some(std::cmp::Ordering::Less);
    }
    if a > b {
        return Some(std::cmp::Ordering::Greater);
    }
    if a == b {
        return Some(std::cmp::Ordering::Equal);
    }
    None
}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Num::USIZE(self_value) => match other {
                Num::USIZE(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U8(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U16(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U32(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U64(other_value) => cmp_numeric(*self_value as u64, *other_value),
                Num::I8(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I16(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I32(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I64(other_value) => cmp_numeric(*self_value as i64, *other_value),
                Num::F32(other_value) => (*other_value).partial_cmp(&(*self_value as f32)),
                Num::F64(other_value) => (*other_value).partial_cmp(&(*self_value as f64)),
            },
            Num::U8(self_value) => match other {
                Num::USIZE(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U8(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U16(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U32(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U64(other_value) => cmp_numeric(*self_value as u64, *other_value),
                Num::I8(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I16(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I32(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I64(other_value) => cmp_numeric(*self_value as i64, *other_value),
                Num::F32(other_value) => (*other_value).partial_cmp(&(*self_value as f32)),
                Num::F64(other_value) => (*other_value).partial_cmp(&(*self_value as f64)),
            },
            Num::U16(self_value) => match other {
                Num::USIZE(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U8(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U16(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U32(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U64(other_value) => cmp_numeric(*self_value as u64, *other_value),
                Num::I8(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I16(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I32(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I64(other_value) => cmp_numeric(*self_value as i64, *other_value),
                Num::F32(other_value) => (*other_value).partial_cmp(&(*self_value as f32)),
                Num::F64(other_value) => (*other_value).partial_cmp(&(*self_value as f64)),
            },
            Num::U32(self_value) => match other {
                Num::USIZE(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U8(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U16(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U32(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U64(other_value) => cmp_numeric(*self_value as u64, *other_value),
                Num::I8(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I16(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I32(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I64(other_value) => cmp_numeric(*self_value as i64, *other_value),
                Num::F32(other_value) => (*other_value).partial_cmp(&(*self_value as f32)),
                Num::F64(other_value) => (*other_value).partial_cmp(&(*self_value as f64)),
            },
            Num::U64(self_value) => match other {
                Num::USIZE(other_value) => cmp_numeric(*self_value, *other_value as u64),
                Num::U8(other_value) => cmp_numeric(*self_value, *other_value as u64),
                Num::U16(other_value) => cmp_numeric(*self_value, *other_value as u64),
                Num::U32(other_value) => cmp_numeric(*self_value, *other_value as u64),
                Num::U64(other_value) => cmp_numeric(*self_value, *other_value),
                Num::I8(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I16(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I32(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I64(other_value) => cmp_numeric(*self_value as i64, *other_value),
                Num::F32(other_value) => (*other_value).partial_cmp(&(*self_value as f32)),
                Num::F64(other_value) => (*other_value).partial_cmp(&(*self_value as f64)),
            },
            Num::I8(self_value) => match other {
                Num::USIZE(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U8(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U16(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U32(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U64(other_value) => cmp_numeric(*self_value as u64, *other_value),
                Num::I8(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I16(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I32(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I64(other_value) => cmp_numeric(*self_value as i64, *other_value),
                Num::F32(other_value) => (*other_value).partial_cmp(&(*self_value as f32)),
                Num::F64(other_value) => (*other_value).partial_cmp(&(*self_value as f64)),
            },
            Num::I16(self_value) => match other {
                Num::USIZE(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U8(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U16(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U32(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U64(other_value) => cmp_numeric(*self_value as u64, *other_value),
                Num::I8(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I16(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I32(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I64(other_value) => cmp_numeric(*self_value as i64, *other_value),
                Num::F32(other_value) => (*other_value).partial_cmp(&(*self_value as f32)),
                Num::F64(other_value) => (*other_value).partial_cmp(&(*self_value as f64)),
            },
            Num::I32(self_value) => match other {
                Num::USIZE(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U8(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U16(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U32(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U64(other_value) => cmp_numeric(*self_value as u64, *other_value),
                Num::I8(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I16(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I32(other_value) => cmp_numeric(*self_value as i64, *other_value as i64),
                Num::I64(other_value) => cmp_numeric(*self_value as i64, *other_value),
                Num::F32(other_value) => (*other_value).partial_cmp(&(*self_value as f32)),
                Num::F64(other_value) => (*other_value).partial_cmp(&(*self_value as f64)),
            },
            Num::I64(self_value) => match other {
                Num::USIZE(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U8(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U16(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U32(other_value) => cmp_numeric(*self_value as u64, *other_value as u64),
                Num::U64(other_value) => cmp_numeric(*self_value as u64, *other_value),
                Num::I8(other_value) => cmp_numeric(*self_value, *other_value as i64),
                Num::I16(other_value) => cmp_numeric(*self_value, *other_value as i64),
                Num::I32(other_value) => cmp_numeric(*self_value, *other_value as i64),
                Num::I64(other_value) => cmp_numeric(*self_value, *other_value),
                Num::F32(other_value) => (*other_value).partial_cmp(&(*self_value as f32)),
                Num::F64(other_value) => (*other_value).partial_cmp(&(*self_value as f64)),
            },
            Num::F32(self_value) => match other {
                Num::USIZE(other_value) => cmp_numeric(*self_value as f64, *other_value as f64),
                Num::U8(other_value) => cmp_numeric(*self_value as f64, *other_value as f64),
                Num::U16(other_value) => cmp_numeric(*self_value as f64, *other_value as f64),
                Num::U32(other_value) => cmp_numeric(*self_value as f64, *other_value as f64),
                Num::U64(other_value) => cmp_numeric(*self_value as f64, *other_value as f64),
                Num::I8(other_value) => cmp_numeric(*self_value as f64, *other_value as f64),
                Num::I16(other_value) => cmp_numeric(*self_value as f64, *other_value as f64),
                Num::I32(other_value) => cmp_numeric(*self_value as f64, *other_value as f64),
                Num::I64(other_value) => cmp_numeric(*self_value as f64, *other_value as f64),
                Num::F32(other_value) => (*other_value).partial_cmp(self_value),
                Num::F64(other_value) => (*other_value).partial_cmp(&(*self_value as f64)),
            },
            Num::F64(self_value) => match other {
                Num::USIZE(other_value) => cmp_numeric(*self_value, *other_value as f64),
                Num::U8(other_value) => cmp_numeric(*self_value, *other_value as f64),
                Num::U16(other_value) => cmp_numeric(*self_value, *other_value as f64),
                Num::U32(other_value) => cmp_numeric(*self_value, *other_value as f64),
                Num::U64(other_value) => cmp_numeric(*self_value, *other_value as f64),
                Num::I8(other_value) => cmp_numeric(*self_value, *other_value as f64),
                Num::I16(other_value) => cmp_numeric(*self_value, *other_value as f64),
                Num::I32(other_value) => cmp_numeric(*self_value, *other_value as f64),
                Num::I64(other_value) => cmp_numeric(*self_value, *other_value as f64),
                Num::F32(other_value) => (*other_value as f64).partial_cmp(self_value),
                Num::F64(other_value) => (*other_value).partial_cmp(self_value),
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Real {
    value: Num,
}

impl Real {
    pub fn new(value: Num) -> Self {
        Self { value }
    }

    pub fn zero() -> Self {
        Real::new(Num::zero())
    }

    pub fn one() -> Self {
        Real::new(Num::F64(1.0))
    }

    pub fn min_err() -> Self {
        Real::from(f64::EPSILON)
    }

    pub fn abs(&self) -> Self {
        Real::new(self.value.abs())
    }

    pub fn powi(&self, n: i32) -> Self {
        Real::new(self.value.powi(n))
    }

    pub fn squared(&self) -> Self {
        Real::new(self.value.squared())
    }

    pub fn powf(&self, n: f64) -> Self {
        Real::new(self.value.powf(n))
    }

    pub fn sqrt(&self) -> Self {
        Real::new(self.value.powf(0.5))
    }

    pub fn exp(&self) -> Self {
        Real::new(self.value.exp())
    }

    pub fn neg_exp(&self) -> Self {
        Real::new(self.value.powf(-std::f64::consts::E))
    }

    pub fn log(&self, base: f64) -> Self {
        if base == 0.0 {
            return Real::new(Num::U8(1));
        }
        if base < 0.0 {
            unimplemented!()
        }
        Real::new(self.value.log(base))
    }

    pub fn log10(&self) -> Self {
        self.log(10.0)
    }

    pub fn log2(&self) -> Self {
        self.log(2.0)
    }

    pub fn ln(&self) -> Self {
        self.log(1.0_f64.exp())
    }

    pub fn sin(&self) -> Self {
        Real::new(self.value.sin())
    }

    pub fn cos(&self) -> Self {
        Real::new(self.value.cos())
    }

    pub fn tan(&self) -> Self {
        Real::new(self.value.tan())
    }

    pub fn arcsin(&self) -> Self {
        Real::new(self.value.arcsin())
    }

    pub fn arccos(&self) -> Self {
        Real::new(self.value.arccos())
    }

    pub fn arctan(&self) -> Self {
        Real::new(self.value.arctan())
    }

    pub fn sinh(&self) -> Self {
        Real::new(self.value.sinh())
    }

    pub fn cosh(&self) -> Self {
        Real::new(self.value.cosh())
    }

    pub fn tanh(&self) -> Self {
        Real::new(self.value.tanh())
    }

    pub fn arcsinh(&self) -> Self {
        Real::new(self.value.arcsinh())
    }

    pub fn arccosh(&self) -> Self {
        Real::new(self.value.arccosh())
    }

    pub fn arctanh(&self) -> Self {
        Real::new(self.value.arccosh())
    }

    pub fn arctan2(_x: Real, _y: Real) -> Self {
        unimplemented!()
    }

    pub fn is_zero(&self) -> bool {
        *self == Real::zero()
    }

    pub fn positive(&self) -> bool {
        self.value.positive()
    }

    pub fn negative(&self) -> bool {
        self.value.negative()
    }
}

impl std::fmt::Display for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

macro_rules! impl_real_from_primitive {
    ($primitive:ty, $num_variant:expr) => {
        impl From<$primitive> for Real {
            fn from(other: $primitive) -> Self {
                Real::new($num_variant(other))
            }
        }
    };
}

impl_real_from_primitive![u8, Num::U8];
impl_real_from_primitive![u16, Num::U16];
impl_real_from_primitive![u32, Num::U32];
impl_real_from_primitive![u64, Num::U64];
impl_real_from_primitive![i8, Num::I8];
impl_real_from_primitive![i16, Num::I16];
impl_real_from_primitive![i32, Num::I32];
impl_real_from_primitive![i64, Num::I64];
impl_real_from_primitive![f32, Num::F32];
impl_real_from_primitive![f64, Num::F64];

impl PartialEq for Real {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Real {}

impl PartialOrd for Real {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Real {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.value.partial_cmp(&other.value) {
            Some(ordering) => ordering,
            None => panic!("Item in comparison is not a real number."),
        }
    }
}

impl Add<Self> for Real {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Real::new(self.value + other.value)
    }
}

impl Sub<Self> for Real {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Real::new(self.value - other.value)
    }
}

impl Mul<Self> for Real {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Real::new(self.value * other.value)
    }
}

impl Div<Self> for Real {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Real::new(self.value / other.value)
    }
}

impl Neg for Real {
    type Output = Self;
    fn neg(self) -> Self {
        Real::new(-(self.value))
    }
}

pub enum DegreesType {
    Degrees,
    Radians,
}

impl DegreesType {
    pub fn to_degrees(value: f64) -> f64 {
        value * 180.0_f64 / std::f64::consts::PI
    }

    pub fn to_radians(value: f64) -> f64 {
        value * std::f64::consts::PI / 180.0_f64
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Complex {
    real: Num,
    imag: Num,
}

impl Complex {
    pub fn new(real: Num, imag: Num) -> Self {
        Self { real, imag }
    }

    pub fn from_real(value: Num) -> Self {
        Complex::new(value, Num::zero())
    }

    pub fn from_imag(value: Num) -> Self {
        Complex::new(Num::zero(), value)
    }

    fn azimuthal_in_degrees(z: &Complex) -> Complex {
        if z.real.is_zero() {
            if z.imag > Num::zero() {
                return Complex::from_real(Num::F64(90.0));
            }
            return Complex::from_real(Num::F64(270.0));
        }
        if z.imag.is_zero() {
            if z.real > Num::zero() {
                return Complex::from_real(Num::zero());
            }
            return Complex::from_real(Num::F64(180.0));
        }
        if z.imag > Num::zero() {
            return Complex::to_degrees(&Complex::from_real((z.real / z.abs().real).arccos()));
        }
        if z.imag < Num::zero() {
            return Complex::to_degrees(&Complex::from_real(
                (z.real / z.abs().real).arccos() + Num::F64(std::f64::consts::PI),
            ));
        }
        Complex::from_real(Num::zero())
    }

    fn azimuthal_in_radians(z: &Complex) -> Complex {
        if z.real.is_zero() {
            if z.imag > Num::zero() {
                return Complex::from_real(Num::F64(std::f64::consts::FRAC_PI_2));
            }
            return Complex::from_real(Num::F64(3.0 * std::f64::consts::FRAC_PI_2));
        }
        if z.imag.is_zero() {
            if z.real > Num::zero() {
                return Complex::from_real(Num::zero());
            }
            return Complex::from_real(Num::F64(std::f64::consts::PI));
        }
        if z.imag > Num::zero() {
            return Complex::from_real((z.real / z.abs().real).arccos());
        }
        if z.imag < Num::zero() {
            return Complex::from_real(
                (z.real / z.abs().real).arccos() + Num::F64(std::f64::consts::PI),
            );
        }
        Complex::from_real(Num::zero())
    }

    pub fn azimuthal(&self, units: DegreesType) -> Self {
        match units {
            DegreesType::Degrees => Complex::azimuthal_in_degrees(self),
            DegreesType::Radians => Complex::azimuthal_in_radians(self),
        }
    }

    pub fn zero() -> Self {
        Complex::new(Num::zero(), Num::zero())
    }

    pub fn one() -> Self {
        Complex::new(Num::one(), Num::zero())
    }

    pub fn i() -> Self {
        Complex::new(Num::zero(), Num::one())
    }

    pub fn min_err() -> Self {
        Complex::from_real(Num::F64(f64::EPSILON))
    }

    pub fn to_degrees(&self) -> Self {
        if self.imag != Num::zero() {
            panic!("Cannot convert imaginary value to degrees")
        }
        *self * Complex::from_real(Num::U16(180))
            / Complex::from_real(Num::F64(std::f64::consts::PI))
    }

    pub fn to_radians(&self) -> Self {
        if self.imag != Num::zero() {
            panic!("Cannot convert imaginary value to radians")
        }
        *self * Complex::from_real(Num::F64(std::f64::consts::PI))
            / Complex::from_real(Num::U16(180))
    }

    pub fn abs(&self) -> Self {
        Complex::from_real((self.real.squared() + self.imag.squared()).sqrt())
    }

    pub fn powi(&self, n: i32) -> Self {
        self.powf(n as f64)
    }

    pub fn squared(&self) -> Self {
        self.powi(2)
    }

    pub fn powf(&self, n: f64) -> Self {
        if self.imag == Num::zero() {
            return Complex::from_real(self.real.powf(n));
        }
        let complex_n = Complex::from_real(Num::F64(n));
        let sine = (complex_n * self.azimuthal(DegreesType::Radians)).sin();
        let cosine = (complex_n * self.azimuthal(DegreesType::Radians)).cos();
        self.abs().powf(n) * (cosine + Complex::i() * sine)
    }

    pub fn sqrt(&self) -> Self {
        self.powf(0.5)
    }

    pub fn exp(&self) -> Self {
        let real_exp = Complex::from_real(self.real.exp());
        let imag_exp = Complex::from_real(self.imag.cos())
            + Complex::from_real(self.imag).sin() * Complex::i();
        real_exp + imag_exp
    }

    pub fn neg_exp(&self) -> Self {
        Complex::one() / self.exp()
    }

    pub fn log(&self, base: f64) -> Self {
        self.ln() / Complex::from_real(Num::F64(base)).ln()
    }

    pub fn log10(&self) -> Self {
        self.log(10.0)
    }

    pub fn log2(&self) -> Self {
        self.log(2.0)
    }

    #[allow(unconditional_recursion)]
    pub fn ln(&self) -> Self {
        let real_part = Complex::from_real(self.real).ln();
        let imag_part = self.azimuthal(DegreesType::Degrees) * Complex::i();
        real_part + imag_part
    }

    pub fn sin(&self) -> Self {
        let num = (*self * Complex::i()).exp() - (-*self * Complex::i()).exp();
        let den = Complex::from_real(Num::U8(2)) * Complex::i();
        num / den
    }

    pub fn cos(&self) -> Self {
        let num = (*self * Complex::i()).exp() + (-*self * Complex::i()).exp();
        let den = Complex::from_real(Num::U8(2));
        num / den
    }

    pub fn tan(&self) -> Self {
        self.sin() / self.cos()
    }

    pub fn is_zero(&self) -> bool {
        self.real == Num::zero() && self.imag == Num::zero()
    }

    pub fn in_half_plane(&self) -> bool {
        self.imag > Num::zero()
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}

impl Eq for Complex {}

macro_rules! impl_complex_from_primitives {
    ($t:ty, $num_variant:expr) => {
        impl From<($t, $t)> for Complex {
            fn from(value: ($t, $t)) -> Self {
                Complex::new($num_variant(value.0), $num_variant(value.1))
            }
        }
    };
    ($left_t:ty, $left_variant:expr, $right_t:ty, $right_variant:expr) => {
        impl From<($left_t, $right_t)> for Complex {
            fn from(value: ($left_t, $right_t)) -> Self {
                Complex::new($left_variant(value.0), $right_variant(value.1))
            }
        }
    };
}

impl_complex_from_primitives![u8, Num::U8];
impl_complex_from_primitives![u8, Num::U8, u16, Num::U16];
impl_complex_from_primitives![u8, Num::U8, u32, Num::U32];
impl_complex_from_primitives![u8, Num::U8, u64, Num::U64];
impl_complex_from_primitives![u8, Num::U8, i8, Num::I8];
impl_complex_from_primitives![u8, Num::U8, i16, Num::I16];
impl_complex_from_primitives![u8, Num::U8, i32, Num::I32];
impl_complex_from_primitives![u8, Num::U8, i64, Num::I64];
impl_complex_from_primitives![u8, Num::U8, f32, Num::F32];
impl_complex_from_primitives![u8, Num::U8, f64, Num::F64];

impl_complex_from_primitives![u16, Num::U16];
impl_complex_from_primitives![u16, Num::U16, u8, Num::U8];
impl_complex_from_primitives![u16, Num::U16, u32, Num::U32];
impl_complex_from_primitives![u16, Num::U16, u64, Num::U64];
impl_complex_from_primitives![u16, Num::U16, i8, Num::I8];
impl_complex_from_primitives![u16, Num::U16, i16, Num::I16];
impl_complex_from_primitives![u16, Num::U16, i32, Num::I32];
impl_complex_from_primitives![u16, Num::U16, i64, Num::I64];
impl_complex_from_primitives![u16, Num::U16, f32, Num::F32];
impl_complex_from_primitives![u16, Num::U16, f64, Num::F64];

impl_complex_from_primitives![u32, Num::U32];
impl_complex_from_primitives![u32, Num::U32, u8, Num::U8];
impl_complex_from_primitives![u32, Num::U32, u16, Num::U16];
impl_complex_from_primitives![u32, Num::U32, u64, Num::U64];
impl_complex_from_primitives![u32, Num::U32, i8, Num::I8];
impl_complex_from_primitives![u32, Num::U32, i16, Num::I16];
impl_complex_from_primitives![u32, Num::U32, i32, Num::I32];
impl_complex_from_primitives![u32, Num::U32, i64, Num::I64];
impl_complex_from_primitives![u32, Num::U32, f32, Num::F32];
impl_complex_from_primitives![u32, Num::U32, f64, Num::F64];

impl_complex_from_primitives![u64, Num::U64];
impl_complex_from_primitives![u64, Num::U64, u8, Num::U8];
impl_complex_from_primitives![u64, Num::U64, u16, Num::U16];
impl_complex_from_primitives![u64, Num::U64, u32, Num::U32];
impl_complex_from_primitives![u64, Num::U64, i8, Num::I8];
impl_complex_from_primitives![u64, Num::U64, i16, Num::I16];
impl_complex_from_primitives![u64, Num::U64, i32, Num::I32];
impl_complex_from_primitives![u64, Num::U64, i64, Num::I64];
impl_complex_from_primitives![u64, Num::U64, f32, Num::F32];
impl_complex_from_primitives![u64, Num::U64, f64, Num::F64];

impl_complex_from_primitives![i8, Num::I8];
impl_complex_from_primitives![i8, Num::I8, u8, Num::U8];
impl_complex_from_primitives![i8, Num::I8, u16, Num::U16];
impl_complex_from_primitives![i8, Num::I8, u32, Num::U32];
impl_complex_from_primitives![i8, Num::I8, u64, Num::U64];
impl_complex_from_primitives![i8, Num::I8, i16, Num::I16];
impl_complex_from_primitives![i8, Num::I8, i32, Num::I32];
impl_complex_from_primitives![i8, Num::I8, i64, Num::I64];
impl_complex_from_primitives![i8, Num::I8, f32, Num::F32];
impl_complex_from_primitives![i8, Num::I8, f64, Num::F64];

impl_complex_from_primitives![i16, Num::I16];
impl_complex_from_primitives![i16, Num::I16, u8, Num::U8];
impl_complex_from_primitives![i16, Num::I16, u16, Num::U16];
impl_complex_from_primitives![i16, Num::I16, u32, Num::U32];
impl_complex_from_primitives![i16, Num::I16, u64, Num::U64];
impl_complex_from_primitives![i16, Num::I16, i8, Num::I8];
impl_complex_from_primitives![i16, Num::I16, i32, Num::I32];
impl_complex_from_primitives![i16, Num::I16, i64, Num::I64];
impl_complex_from_primitives![i16, Num::I16, f32, Num::F32];
impl_complex_from_primitives![i16, Num::I16, f64, Num::F64];

impl_complex_from_primitives![i32, Num::I32];
impl_complex_from_primitives![i32, Num::I32, u8, Num::U8];
impl_complex_from_primitives![i32, Num::I32, u16, Num::U16];
impl_complex_from_primitives![i32, Num::I32, u32, Num::U32];
impl_complex_from_primitives![i32, Num::I32, u64, Num::U64];
impl_complex_from_primitives![i32, Num::I32, i8, Num::I8];
impl_complex_from_primitives![i32, Num::I32, i16, Num::I16];
impl_complex_from_primitives![i32, Num::I32, i64, Num::I64];
impl_complex_from_primitives![i32, Num::I32, f32, Num::F32];
impl_complex_from_primitives![i32, Num::I32, f64, Num::F64];

impl_complex_from_primitives![i64, Num::I64];
impl_complex_from_primitives![i64, Num::I64, u8, Num::U8];
impl_complex_from_primitives![i64, Num::I64, u16, Num::U16];
impl_complex_from_primitives![i64, Num::I64, u32, Num::U32];
impl_complex_from_primitives![i64, Num::I64, u64, Num::U64];
impl_complex_from_primitives![i64, Num::I64, i8, Num::I8];
impl_complex_from_primitives![i64, Num::I64, i16, Num::I16];
impl_complex_from_primitives![i64, Num::I64, i32, Num::I32];
impl_complex_from_primitives![i64, Num::I64, f32, Num::F32];
impl_complex_from_primitives![i64, Num::I64, f64, Num::F64];

impl_complex_from_primitives![f32, Num::F32];
impl_complex_from_primitives![f32, Num::F32, u8, Num::U8];
impl_complex_from_primitives![f32, Num::F32, u16, Num::U16];
impl_complex_from_primitives![f32, Num::F32, u32, Num::U32];
impl_complex_from_primitives![f32, Num::F32, u64, Num::U64];
impl_complex_from_primitives![f32, Num::F32, i8, Num::I8];
impl_complex_from_primitives![f32, Num::F32, i16, Num::I16];
impl_complex_from_primitives![f32, Num::F32, i32, Num::I32];
impl_complex_from_primitives![f32, Num::F32, i64, Num::I64];
impl_complex_from_primitives![f32, Num::F32, f64, Num::F64];

impl_complex_from_primitives![f64, Num::F64];
impl_complex_from_primitives![f64, Num::F64, u8, Num::U8];
impl_complex_from_primitives![f64, Num::F64, u16, Num::U16];
impl_complex_from_primitives![f64, Num::F64, u32, Num::U32];
impl_complex_from_primitives![f64, Num::F64, u64, Num::U64];
impl_complex_from_primitives![f64, Num::F64, i8, Num::I8];
impl_complex_from_primitives![f64, Num::F64, i16, Num::I16];
impl_complex_from_primitives![f64, Num::F64, i32, Num::I32];
impl_complex_from_primitives![f64, Num::F64, i64, Num::I64];
impl_complex_from_primitives![f64, Num::F64, f32, Num::F32];

impl Add<Self> for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let real_sum = self.real + other.real;
        let imag_sum = self.imag + other.imag;
        Complex::new(real_sum, imag_sum)
    }
}

impl Sub<Self> for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let real_diff = self.real - other.real;
        let imag_diff = self.imag - other.imag;
        Complex::new(real_diff, imag_diff)
    }
}

impl Mul<Self> for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let real_part = self.real * other.real - self.imag * other.imag;
        let imag_part = self.real * other.imag + self.imag * other.real;
        Complex::new(real_part, imag_part)
    }
}

impl Div<Self> for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let den = other.real.squared() + other.imag.squared();
        let real_part = (self.real * other.real + self.imag * other.imag) / den;
        let imag_part = (self.imag * other.real - self.real * other.imag) / den;
        Complex::new(real_part, imag_part)
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Complex::new(-self.real, -self.imag)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    mod test_real {

        use super::*;

        mod test_abs {

            use super::*;

            macro_rules! unsigned_abs_test {
                ($name:ident, $num_variant:expr, $value:expr) => {
                    #[test]
                    fn $name() {
                        let x = Real::new($num_variant($value));
                        assert_eq!(x, x.abs());
                    }
                };
            }

            unsigned_abs_test! {usize, Num::USIZE, 12000}
            unsigned_abs_test! {u8, Num::U8, 12}
            unsigned_abs_test! {u16, Num::U16, 120}
            unsigned_abs_test! {u32, Num::U32, 1200}
            unsigned_abs_test! {u64, Num::U64, 12000}

            macro_rules! signed_abs_test {
                ($name:ident, $num_variant:expr, $value:expr) => {
                    #[test]
                    fn $name() {
                        let x = Real::new($num_variant($value));
                        assert_eq!(x, x.abs());
                        let y = Real::new($num_variant(-$value));
                        assert_eq!(y, -y.abs());
                    }
                };
            }

            signed_abs_test! {i8, Num::I8, 12}
            signed_abs_test! {i16, Num::I16, 120}
            signed_abs_test! {i32, Num::I32, 1200}
            signed_abs_test! {i64, Num::I64, 12000}
            signed_abs_test! {f32, Num::F32, 1200.0}
            signed_abs_test! {f64, Num::F64, 12000.0}
        }

        mod test_exponentials {

            use super::*;

            #[test]
            fn exp() {
                let e = Real::from(std::f64::consts::E);
                assert!(e == Real::one().exp());
                assert!(e.powi(2) == e * e);
                assert!(e.powi(0) == Real::one());
            }

            mod test_logs {

                use super::*;

                #[test]
                fn ln() {
                    let e = Real::from(std::f64::consts::E);
                    assert!(e.ln() == Real::one());
                    assert!((e * e).ln() == Real::from(2));
                }

                #[test]
                fn log10() {
                    assert!(Real::from(10).log10() == Real::one());
                    assert!(Real::from(100).log10() == Real::from(2));
                }

                #[test]
                fn log2() {
                    assert!(Real::from(2).log2() == Real::one());
                    assert!(Real::from(8).log2() == Real::from(3));
                }
            }
        }

        mod test_trig {

            use super::*;

            mod test_circular {

                use super::*;

                #[test]
                fn sin() {
                    assert!(Real::from(std::f64::consts::PI).sin() == Real::zero());
                    assert!(Real::from(std::f64::consts::FRAC_PI_2).sin() == Real::one());
                    assert!((-Real::from(std::f64::consts::FRAC_PI_2)).sin() == -Real::one());
                }

                #[test]
                fn cos() {
                    assert!(Real::from(std::f64::consts::PI).cos() == -Real::one());
                    assert!(Real::from(std::f64::consts::FRAC_PI_2).cos() == Real::zero());
                    assert!((-Real::from(std::f64::consts::FRAC_PI_2)).cos() == Real::zero());
                }

                #[test]
                fn tan() {
                    assert!(Real::from(std::f64::consts::PI).tan() == Real::zero());
                    assert!(Real::from(std::f64::consts::FRAC_PI_4).tan() == Real::one());
                    assert!((-Real::from(std::f64::consts::FRAC_PI_4)).tan() == -Real::one());
                }

                #[test]
                fn arcsin() {
                    assert!(Real::zero().arcsin() == Real::zero());
                    assert!(Real::one().arcsin() == Real::from(std::f64::consts::FRAC_PI_2));
                    assert!((-Real::one()).arcsin() == -Real::from(std::f64::consts::FRAC_PI_2));
                }

                #[test]
                fn arccos() {
                    assert!(Real::zero().arccos() == Real::from(std::f64::consts::FRAC_PI_2));
                    assert!(Real::one().arccos() == Real::zero());
                    assert!((-Real::one()).arccos() == Real::from(std::f64::consts::PI))
                }

                #[test]
                fn arctan() {
                    assert!(Real::zero().arctan() == Real::zero());
                    assert!(Real::one().arctan() == Real::from(std::f64::consts::FRAC_PI_4));
                    assert!((-Real::one()).arctan() == -Real::from(std::f64::consts::FRAC_PI_4));
                }
            }

            mod test_hyperbolic {

                use super::*;

                #[test]
                fn sinh() {
                    assert!(Real::from(1.0_f64.sinh()) == Real::from(1.1752011936438014));
                }

                #[test]
                fn cosh() {
                    assert!(Real::from(1.0_f64.cosh()) == Real::from(1.5430806348152437));
                }

                #[test]
                fn tanh() {
                    assert!(Real::from(1.0_f64.tanh()) == Real::from(0.7615941559557649));
                }

                fn arcsinh_substitution(value: Real) -> Real {
                    (value + (value.powi(2) + Real::one()).sqrt()).ln()
                }

                #[test]
                fn arcsinh() {
                    assert_eq!(
                        arcsinh_substitution(Real::one()),
                        Real::from(1.0_f64).arcsinh()
                    );
                    assert_eq!(
                        arcsinh_substitution(Real::from(0.5)),
                        Real::from(0.5_f64).arcsinh()
                    );
                }

                fn arccosh_substitution(value: Real) -> Real {
                    (value + (value.powi(2) - Real::one()).sqrt()).ln()
                }

                #[test]
                fn arccosh() {
                    assert_eq!(
                        arccosh_substitution(Real::one()),
                        Real::from(1.0_f64).arccosh()
                    );
                    assert_eq!(
                        arccosh_substitution(Real::from(1.5)),
                        Real::from(1.5_f64).arccosh()
                    );
                }

                fn arctanh_substitution(value: Real) -> Real {
                    Real::from(0.5) * ((Real::one() + value) / (Real::one() - value)).ln()
                }

                #[test]
                fn arctanh() {
                    assert_eq!(arctanh_substitution(Real::zero()), Real::zero());
                    assert_eq!(
                        arctanh_substitution(Real::from(0.5_f64)),
                        Real::from(0.5493061443340549)
                    );
                }
            }
        }

        #[test]
        fn is_positive() {
            assert!(Real::from(2).positive());
            assert!(Real::from(2.0).positive());
            assert!(Real::from(1.5).positive());
            assert!(!Real::from(0).positive());
            assert!(!Real::from(0.0).positive());
            assert!(!Real::from(-0.0).positive());
            assert!(!Real::from(-0).positive());
            assert!(!Real::from(-1.5).positive());
            assert!(!Real::from(-2.0).positive());
            assert!(!Real::from(-2).positive());
        }

        #[test]
        fn is_negative() {
            assert!(!Real::from(2).negative());
            assert!(!Real::from(2.0).negative());
            assert!(!Real::from(1.5).negative());
            assert!(!Real::from(0).negative());
            assert!(!Real::from(0.0).negative());
            assert!(!Real::from(-0.0).negative());
            assert!(!Real::from(-0).negative());
            assert!(Real::from(-1.5).negative());
            assert!(Real::from(-2.0).negative());
            assert!(Real::from(-2).negative());
        }

        #[test]
        fn is_zero() {
            assert!(!Real::from(2).is_zero());
            assert!(!Real::from(1.5).is_zero());
            assert!(Real::from(0).is_zero());
            assert!(Real::from(0.0).is_zero());
            assert!(Real::from(-0.0).is_zero());
            assert!(Real::from(-0).is_zero());
            assert!(!Real::from(-1.5).is_zero());
            assert!(!Real::from(-2.0).is_zero());
            assert!(!Real::from(-2).is_zero());
        }
    }
}
