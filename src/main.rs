
use ark_bn254::Bn254;
use ark_groth16::Groth16;
use ark_snark::{CircuitSpecificSetupSNARK, SNARK};
use ark_relations::{
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
    lc
};
use ark_std::{rand::rngs::OsRng, UniformRand};

struct SquareCircuit {
    pub x: Option<ark_bn254::Fr>,
    pub y: Option<ark_bn254::Fr>,
}

impl ConstraintSynthesizer<ark_bn254::Fr> for SquareCircuit {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ark_bn254::Fr>,
    ) -> Result<(), SynthesisError> {
        let x_var = cs.new_witness_variable(|| self.x.ok_or(SynthesisError::AssignmentMissing))?;
        let y_var = cs.new_input_variable(|| self.y.ok_or(SynthesisError::AssignmentMissing))?;
        
        cs.enforce_constraint(
            lc!() + x_var,
            lc!() + x_var,
            lc!() + y_var,
        )?;
        Ok(())
    }
}

fn main() {
    let mut rng = OsRng;
    let x_val = ark_bn254::Fr::rand(&mut rng);
    let y_val = x_val * x_val;

    let circuit = SquareCircuit {
        x: Some(x_val),
        y: Some(y_val),
    };

    let (pk, vk) = Groth16::<Bn254>::setup(circuit, &mut rng).unwrap();
    let proof = Groth16::<Bn254>::prove(
        &pk, 
        SquareCircuit {
            x: Some(x_val),
            y: Some(y_val),
        }, 
        &mut rng
    ).unwrap();

    // Directly use the VerifyingKey (not PreparedVerifyingKey)
    let verified = Groth16::<Bn254>::verify(
        &vk,  // VerifyingKey
        &[y_val], 
        &proof
    ).unwrap();

    println!("Proof is valid: {}", verified);
}
