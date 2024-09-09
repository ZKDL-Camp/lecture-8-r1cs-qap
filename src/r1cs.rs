//! This module contains the implementation of Rank-1 Constraint System (R1CS).
//! It consists of three matrices: left, right, and output, each of which is a vector of vectors.
//! The R1CS is satisfied if the Hadamard product of left and right matrices times solution witness 
//! is equal to the output matrix times solution witness.
//! 
//! Here, you need to implement the function checking whether the R1CS is satisfied by the witness.

use crate::{finite_field::Fp, linear_algebra::{Matrix, Vector}};

/// Structure representing general R1CS. It consists of three matrices:
/// left, right, and output, each of which is a vector of vectors
pub struct R1CS<const WITNESS_SIZE: usize, const CONSTRAINT_NUM: usize> {
    left_matrix: Matrix<WITNESS_SIZE, CONSTRAINT_NUM>,
    right_matrix: Matrix<WITNESS_SIZE, CONSTRAINT_NUM>,
    output_matrix: Matrix<WITNESS_SIZE, CONSTRAINT_NUM>,
}

impl<const WITNESS_SIZE: usize, const CONSTRAINT_NUM: usize> R1CS<WITNESS_SIZE, CONSTRAINT_NUM> {
    /// Creates a new R1CS instance from three matrices
    pub fn new(
        left_matrix: Matrix<WITNESS_SIZE, CONSTRAINT_NUM>,
        right_matrix: Matrix<WITNESS_SIZE, CONSTRAINT_NUM>,
        output_matrix: Matrix<WITNESS_SIZE, CONSTRAINT_NUM>,
    ) -> Self {
        R1CS {
            left_matrix,
            right_matrix,
            output_matrix,
        }
    }

    /// Checks whether specified witness satisfies the R1CS
    pub fn is_satisfied(&self, witness: &Vector<WITNESS_SIZE>) -> bool {
        assert_eq!(witness.get(0), Fp::from(1), "first element of witness must be 1!");

        // TODO: Implement R1CS satisfaction check here!
        unimplemented!("Implement R1CS satisfaction check!")
    }

    /// Checks whether the j-th R1CS constraint is satisfied by the witness
    pub fn is_constraint_satisfied(&self, witness: &Vector<WITNESS_SIZE>, constraint_id: usize) -> bool {
        assert_eq!(witness.get(0), Fp::from(1), "first element of witness must be 1!");
        assert!(constraint_id < CONSTRAINT_NUM, "invalid constraint id!");

        // TODO: Implement R1CS constraint satisfaction check here!
        unimplemented!("Implement R1CS constraint satisfaction check!")
    }
}

/// Tests below create a toy R1CS to check the circuit satisfiability
/// C(x1, x2, x3) = x1*x2*x3 + (1-x1)(x2+x3) for x2, x3 in Fp and x1 in {0,1}.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::finite_field::Fp;

    const WITNESS_SIZE: usize = 7;
    const CONSTRAINT_NUM: usize = 4;
    
    fn create_toy_r1cs() -> R1CS<WITNESS_SIZE, CONSTRAINT_NUM> {
        let zero = Fp::from(0);
        let one = Fp::from(1);

        let left_matrix = Matrix::new([
            Vector::new([zero, zero, one, zero, zero, zero, zero]),
            Vector::new([zero, zero, zero, one, zero, zero, zero]),
            Vector::new([zero, zero, one, zero, zero, zero, zero]),
            Vector::new([one, zero, -one, zero, zero, zero, zero]),
        ]);
        let right_matrix = Matrix::new([
            Vector::new([zero, zero, one, zero, zero, zero, zero]),
            Vector::new([zero, zero, zero, zero, one, zero, zero]),
            Vector::new([zero, zero, zero, zero, zero, one, zero]),
            Vector::new([zero, zero, zero, one, one, zero, zero]),
        ]);
        let output_matrix = Matrix::new([
            Vector::new([zero, zero, one, zero, zero, zero, zero]),
            Vector::new([zero, zero, zero, zero, zero, one, zero]),
            Vector::new([zero, zero, zero, zero, zero, zero, one]),
            Vector::new([zero, one, zero, zero, zero, zero, -one]),
        ]);

        R1CS::new(left_matrix, right_matrix, output_matrix)
    }

    #[test]
    fn test_toy_r1cs_valid_witness_1() {
        let toy_r1cs = create_toy_r1cs();

        let valid_witness = Vector::new([
            Fp::from(1),
            Fp::from(12),
            Fp::from(1),
            Fp::from(3),
            Fp::from(4),
            Fp::from(12),
            Fp::from(12),
        ]);

        assert!(
            toy_r1cs.is_satisfied(&valid_witness),
            "valid witness failed!"
        );
    }

    #[test]
    fn test_toy_r1cs_valid_witness_2() {
        let toy_r1cs = create_toy_r1cs();

        let valid_witness = Vector::new([
            Fp::from(1),
            Fp::from(7),
            Fp::from(0),
            Fp::from(3),
            Fp::from(4),
            Fp::from(12),
            Fp::from(0),
        ]);

        assert!(
            toy_r1cs.is_satisfied(&valid_witness),
            "valid witness failed!"
        );
    }

    #[test]
    fn test_toy_r1cs_invalid_witness() {
        let toy_r1cs = create_toy_r1cs();

        let invalid_witness = Vector::new([
            Fp::from(1),
            Fp::from(12),
            Fp::from(0),
            Fp::from(3),
            Fp::from(4),
            Fp::from(12),
            Fp::from(12),
        ]);

        assert!(
            !toy_r1cs.is_satisfied(&invalid_witness),
            "invalid witness passed!"
        );
    }

    #[test]
    fn test_toy_r1cs_constraints() {
        let toy_r1cs = create_toy_r1cs();

        let invalid_witness = Vector::new([
            Fp::from(1),
            Fp::from(12),
            Fp::from(0),
            Fp::from(3),
            Fp::from(4),
            Fp::from(12),
            Fp::from(12),
        ]);

        // Here, the first two constraints should be satisfied
        // while the last two should not be satisfied
        assert!(
            toy_r1cs.is_constraint_satisfied(&invalid_witness, 0),
            "correct constraint is not satisfied!"
        );
        assert!(
            toy_r1cs.is_constraint_satisfied(&invalid_witness, 1),
            "correct constraint is not satisfied!"
        );
        assert!(
            !toy_r1cs.is_constraint_satisfied(&invalid_witness, 2),
            "incorrect constraint is satisfied!"
        );
        assert!(
            !toy_r1cs.is_constraint_satisfied(&invalid_witness, 3),
            "incorrect constraint is satisfied!"
        );
    }
}
