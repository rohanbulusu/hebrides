//! Implementations for linear algebra.
//!
//! `linal` provides two main structs: Vector and Matrix. These together
//! support a wide array of operations in finite-dimensional space and
//! form the basis of the linear algebra system for `hebrides`.

/// Representation of finite-dimensional vectors
#[derive(Debug)]
pub struct Vector<T> {
	components: Vec<T>
}

impl<T> Vector<T> where T: Clone{

	fn new(components: &[T]) -> Vector<T> {
		Vector {
			components: components.to_vec()
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

	}

}



