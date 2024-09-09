//! After you complete the implementation of the R1CS, you can test it by running this file
//! to see if the R1CS is satisfied by a random witness for `x^3 = y` relation.

pub mod finite_field;
pub mod linear_algebra;
pub mod r1cs;

use colored::Colorize;
use finite_field::Fp;
use linear_algebra::{Matrix, Vector};
use r1cs::R1CS;
use rand::Rng;

/// Here, we generate a random witness for our cube root R1CS. Namely, we have
/// the following R1CS:
///
/// - `r1 == x * x`
/// - `r2 == r1 * x - y`
///
/// So our witness consists of values (1, x, y, r1, r2) and we enforce two constraints.
///
/// This function generates random (1, x, y, r1, r2), satisfying the constraints.
fn generate_random_witness() -> Vector<5> {
    let mut rng = rand::thread_rng();

    // Taking random x and calculating y = x^3
    let x: Fp = rng.gen();
    let y = x * x * x;

    // Now, generating intermediate values
    let r1 = x * x;
    let r2 = r1 * x - y;

    // Our witness is (1, x, y, r1, r2)
    Vector::new([Fp::from(1), x, y, r1, r2])
}

/// Main function to test the R1CS. It creates a cube root R1CS and checks if a random witness
/// satisfies the R1CS.
#[allow(unreachable_code, unused_variables)] // To remove the warnings, remove this line after implementing the functions
fn main() {
    println!("{}", "Creating R1CS...".bright_black());

    let zero = Fp::from(0);
    let one = Fp::from(1);

    // NOTE: You might assume that the left matrix is already 
    // correctly(!) implemented for you.
    let left_matrix: Matrix<5, 2> = Matrix::new([
        Vector::new([zero, one, zero, zero, zero]),
        Vector::new([zero, one, zero, zero, zero]),
    ]);
    // TODO: Implement right matrix here!
    let right_matrix: Matrix<5, 2> = {
        unimplemented!("Implement right matrix here!")
    };
    // TODO: Implement output matrix here!
    let output_matrix: Matrix<5, 2> = {
        unimplemented!("Implement right matrix here!")
    };

    let cube_root_r1cs = R1CS::new(left_matrix, right_matrix, output_matrix);
    println!("{}", "R1CS Created Successfully!".bright_black());

    // Now, let us check the R1CS
    let witness = generate_random_witness();
    println!(
        "{}: {:?}",
        "Generated random witness".bright_black(),
        witness
    );

    let is_satisfied = cube_root_r1cs.is_satisfied(&witness);

    if is_satisfied {
        println!(
            "{}",
            "The R1CS is satisfied by the witness! Congratulations!"
                .green()
                .bold()
        );
        return;
    } 

    println!(
        "{}",
        "The R1CS is not satisfied by the witness! You have done something wrong!"
            .red()
            .bold()
    );
}
