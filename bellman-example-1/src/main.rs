use rand::Rng;
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bellman::groth16::{Proof, generate_random_parameters, prepare_verifying_key, verify_proof};
use bellman::Scalar;


struct MyCircuit {
    a: Option<Scalar>,
    b: Option<Scalar>,
    c: Option<Scalar>,
}


impl Circuit<Scalar> for MyCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let a = cs.alloc(|| "a", || self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.alloc(|| "b", || self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let c = cs.alloc(|| "c", || self.c.ok_or(SynthesisError::AssignmentMissing))?;
        
        // Enforce the constraint a * b = c
        cs.enforce(|| "a * b = c", |lc| lc + a, |lc| lc + b, |lc| lc + c);
        
        Ok(())
    }
}


fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();
    
    // Generate some random values for the inputs
    let a_val = Scalar::random(&mut rng);
    let b_val = Scalar::random(&mut rng);
    let c_val = a_val * b_val; // Compute the expected output
    
    // Construct the circuit instance
    let circuit = MyCircuit {
        a: Some(a_val),
        b: Some(b_val),
        c: Some(c_val),
    };
    
    // Generate the proving key and verification key
    let params = generate_random_parameters::<Scalar, _, _>(circuit, &mut rng).unwrap();
    let pvk = prepare_verifying_key(&params.vk);
    
    // Create a proof of correctness
    let proof = {
        let mut prover = match bellman::groth16::create_random_proof(circuit, &params, &mut rng) {
            Ok(proof) => proof,
            Err(_) => panic!("Failed to generate proof"),
        };
        prover.gen_proof()
    };
    
    // Verify the proof
    assert!(verify_proof(&pvk, &proof, &[c_val]).is_ok());
}
