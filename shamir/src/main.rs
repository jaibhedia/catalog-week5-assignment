mod algos;
use rand::Rng;

fn main() -> Result<(), algos::sss::ShamirError> {
    let mut rng = rand::thread_rng();

    let secret: u64 = rng.gen_range(1..2003);
    println!("Random secret generated: {}", secret);

    algos::sss::run_shamir_with_secret(secret)?;
    
    algos::vss::run_vss(secret as i128);
    
    Ok(())
}
