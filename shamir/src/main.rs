// In shamir/src/main.rs

mod algos;
use rand::Rng;

fn main() -> Result<(), algos::sss::ShamirError> {
    let mut rng = rand::thread_rng();
    // Ensure secret is less than VSS's PRIME (2003)
    let secret: u64 = rng.gen_range(1..2003);
    println!("Random secret generated: {}", secret);

    // Run the SSS demonstration with the generated secret
    algos::sss::run_shamir_with_secret(secret)?;
    
    // Run the VSS demonstration with the same secret (converted to i128)
    algos::vss::run_vss(secret as i128);
    
    Ok(())
}
