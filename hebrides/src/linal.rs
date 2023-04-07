//! Implementations for linear algebra.
//!
//! `linal` provides two main structs: Vector and Matrix. These together
//! support a wide array of operations in finite-dimensional space and
//! form the basis of the linear algebra system for `hebrides`.

use std::ops::{Add, Sub, Mul};

/// Representation of finite-dimensional vectors
#[derive(Debug)]
pub struct Vector<T> {
	components: Vec<T>,
	dim: usize
}

impl<T> Vector<T> where T: Clone {

	fn new(components: &[T]) -> Vector<T> {
		Vector {
			components: components.to_vec(),
			dim: components.len()
		}
	}

}

impl<T> PartialEq for Vector<T> where T: PartialEq {
	fn eq(&self, other: &Vector<T>) -> bool {
		let pairs = self.components.iter().zip(other.components.iter());
		for pair in pairs {
			if pair.0 != pair.1 {
				return false;
			}
		}
		return true;
	}
}

impl<T> Add<Self> for Vector<T> where T: Copy + Clone + Add<Output=T> {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		if self.dim != other.dim {
			panic!("Vectors must be of the same dimension to be summed")
		}
		let mut sum = Vec::with_capacity(self.dim);
		let pairs = self.components.iter().zip(other.components.iter());
		for pair in pairs {
			sum.push(*pair.0 + *pair.1);
		}
		Vector::new(sum.as_slice())
	}
}

impl<T> Sub<Self> for Vector<T> where T: Copy + Clone + Sub<Output=T> {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		if self.dim != other.dim {
			panic!("Vectors must be of the same dimension to be subtracted")
		}
		let mut diff = Vec::with_capacity(self.dim);
		let pairs = self.components.iter().zip(other.components.iter());
		for pair in pairs {
			diff.push(*pair.0 - *pair.1);
		}
		Vector::new(diff.as_slice())
	}
}

/// Implements a dot product between Vectors
impl<T> Mul<Self> for Vector<T> where T: Copy + Mul<Output=T> + std::iter::Sum {
	type Output = T;
	fn mul(self, other: Self) -> T {
		if self.dim != other.dim {
			panic!("Dot products can only be taken between vectors of the same dimension")
		}
		let pairs = self.components.iter().zip(other.components.iter());
		pairs.map(|pair| *pair.0 * *pair.1).sum()
	}
}


#[cfg(test)]
mod test {

	use super::*;

	mod vector {

		use super::*;

		mod equality {

			use super::*;

			#[test]
			fn equality() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[1, 2, 3]);
				assert_eq!(a, b)
			}

			#[test]
			fn normal_inequality() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[4, 5, 6]);
				assert_ne!(a, b)
			}

			#[test]
			fn mixed_inequality() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[3, 2, 1]);
				assert_ne!(a, b)
			}

		}

		mod dimension {

			use super::*;

			#[test]
			fn equal_dimension() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[4, 5, 6]);
				assert_eq!(a.dim, b.dim)
			}

			#[test]
			fn differing_dimension() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[1, 2]);
				assert_ne!(a.dim, b.dim)
			}

		}

		mod addition {

			use super::*;

			#[test]
			fn pos_elements() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[4, 5, 6]);
				assert_eq!(a + b, Vector::new(&[5, 7, 9]))
			}

			#[test]
			fn neg_elements() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[-2, -3, -4]);
				assert_eq!(a + b, Vector::new(&[-1, -1, -1]))
			}

		}

		mod subtraction {

			use super::*;

			#[test]
			fn pos_elements() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[3, 4, 5]);
				assert_eq!(a - b, Vector::new(&[-2, -2, -2]))
			}

			#[test]
			fn neg_elements() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[-1, -2, -3]);
				assert_eq!(a - b, Vector::new(&[2, 4, 6]))
			}

		}

		mod dot_product {

			use super::*;

			#[test]
			fn pos_elements() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[1, 2, 3]);
				assert_eq!(a * b, 14)
			}

			#[test]
			fn neg_elements() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[-1, -2, -3]);
				assert_eq!(a * b, -14)
			}

			#[test]
			fn orthogonal() {
				let a = Vector::new(&[1, 0]);
				let b = Vector::new(&[0, 1]);
				assert_eq!(a * b, 0);
			}

		}

	}

}



