use rand::Rng;

const PRIME: u64 = 2147483647;

#[derive(Debug)]
pub enum ShamirError {
    InvalidThreshold,
    InvalidShareCount,
    InsufficientShares,
}

pub fn generate_polynomial(secret: u64, threshold: usize) -> Result<Vec<u64>, ShamirError> {
    if threshold < 2 {
        return Err(ShamirError::InvalidThreshold);
    }

    let mut rng = rand::thread_rng();
    let mut coeffs = vec![secret]; 
    for _ in 1..threshold {
        coeffs.push(rng.gen_range(1..PRIME));
    }
    Ok(coeffs)
}

pub fn evaluate_polynomial(coeffs: &[u64], x: u64) -> u64 {
    let mut result = 0;
    for &coeff in coeffs.iter().rev() {
        result = (result * x + coeff) % PRIME;
    }
    result
}

pub fn generate_shares(
    secret: u64,
    threshold: usize,
    num_shares: usize,
) -> Result<Vec<(u64, u64)>, ShamirError> {
    if num_shares < threshold {
        return Err(ShamirError::InvalidShareCount);
    }

    let coeffs = generate_polynomial(secret, threshold)?;
    let mut shares = Vec::with_capacity(num_shares);
    for x in 1..=num_shares as u64 {
        shares.push((x, evaluate_polynomial(&coeffs, x)));
    }
    Ok(shares)
}

fn mod_inverse(a: u64) -> u64 {
    let mut t: i128 = 0;
    let mut newt: i128 = 1;
    let mut r: i128 = PRIME as i128;
    let mut newr: i128 = a as i128;

    while newr != 0 {
        let quotient = r / newr;
        let temp_t = t;
        t = newt;
        newt = temp_t - quotient * newt;
        let temp_r = r;
        r = newr;
        newr = temp_r - quotient * newr;
    }

    if t < 0 {
        t += PRIME as i128;
    }
    t as u64
}

pub fn reconstruct_secret(shares: &[(u64, u64)], threshold: usize) -> Result<u64, ShamirError> {
    if shares.len() < threshold {
        return Err(ShamirError::InsufficientShares);
    }

    let mut secret: i128 = 0; 
    for i in 0..threshold {
        let (x_i, y_i) = shares[i];
        let mut numerator: i128 = 1;
        let mut denominator: i128 = 1;

        for j in 0..threshold {
            if i != j {
                let (x_j, _) = shares[j];
                numerator = (numerator * ((PRIME as i128) - x_j as i128)) % (PRIME as i128);
                let diff = ((x_i as i128) - (x_j as i128) + (PRIME as i128)) % (PRIME as i128);
                denominator = (denominator * diff) % (PRIME as i128);
            }
        }

        let lagrange_coeff = (numerator * mod_inverse(denominator as u64) as i128) % (PRIME as i128);
        secret = (secret + (y_i as i128 * lagrange_coeff) % (PRIME as i128)) % (PRIME as i128);
    }
    Ok((secret as u64) % PRIME)
}

pub fn run_shamir_with_secret(secret: u64) -> Result<u64, ShamirError> {
    let threshold = 3;
    let num_shares = 5;

    let shares = generate_shares(secret, threshold, num_shares)?;
    println!("(SSS) Generated shares: {:?}", shares);

    let reconstructed = reconstruct_secret(&shares[..threshold], threshold)?;
    println!("(SSS) Successfully reconstructed secret: {}", reconstructed);

    assert_eq!(reconstructed, secret);
    Ok(secret)
}
