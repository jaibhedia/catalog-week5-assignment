use rand::Rng;

const Q: i128 = 2003;  
const P: i128 = 4007;  
const THRESHOLD: usize = 3; 
const SHARES_COUNT: usize = 5;  
const G: i128 = 2;  

fn mod_norm(a: i128, m: i128) -> i128 {
    let r = a % m;
    if r < 0 { r + m } else { r }
}


fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    let mut result = 1;
    base = mod_norm(base, modulus);
    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_norm(result * base, modulus);
        }
        exp /= 2;
        base = mod_norm(base * base, modulus);
    }
    result
}

fn mod_inverse(a: i128, m: i128) -> i128 {
    let (mut t, mut new_t) = (0, 1);
    let (mut r, mut new_r) = (m, mod_norm(a, m));
    while new_r != 0 {
        let quotient = r / new_r;
        let temp_t = t;
        t = new_t;
        new_t = temp_t - quotient * new_t;
        let temp_r = r;
        r = new_r;
        new_r = temp_r - quotient * new_r;
    }
    if r != 1 {
        panic!("Modular inverse does not exist for {} mod {}", a, m);
    }
    mod_norm(t, m)
}


fn generate_polynomial(secret: i128, threshold: usize, rng: &mut impl Rng) -> Vec<i128> {
    let mut coeffs = Vec::with_capacity(threshold);
    coeffs.push(mod_norm(secret, Q));
    for _ in 1..threshold {
        coeffs.push(rng.gen_range(0..Q));
    }
    coeffs
}


fn eval_polynomial(coeffs: &[i128], x: i128) -> i128 {
    let mut sum = 0;
    for (i, &coeff) in coeffs.iter().enumerate() {
        let term = mod_norm(coeff * mod_pow(x, i as i128, Q), Q);
        sum = mod_norm(sum + term, Q);
    }
    sum
}


fn generate_shares(coeffs: &[i128]) -> Vec<(i128, i128)> {
    (1..=SHARES_COUNT as i128)
        .map(|x| (x, eval_polynomial(coeffs, x)))
        .collect()
}


fn generate_commitments(coeffs: &[i128]) -> Vec<i128> {
    coeffs.iter()
        .map(|&a| mod_pow(G, mod_norm(a, Q), P))
        .collect()
}


fn verify_share(share: (i128, i128), commitments: &[i128]) -> bool {
    let (x, y) = share;
    let lhs = mod_pow(G, y, P);
    let mut rhs = 1;
    for (i, &commitment) in commitments.iter().enumerate() {
    
        let exponent = mod_pow(x, i as i128, Q);
        rhs = mod_norm(rhs * mod_pow(commitment, exponent, P), P);
    }
    lhs == rhs
}

fn reconstruct_secret(shares: &[(i128, i128)]) -> i128 {
    let mut secret = 0;
    for (j, &(xj, yj)) in shares.iter().enumerate() {
        let mut num = 1;
        let mut den = 1;
        for (m, &(xm, _)) in shares.iter().enumerate() {
            if m != j {
                num = mod_norm(num * mod_norm(-xm, Q), Q);
                let diff = mod_norm(xj - xm, Q);
                den = mod_norm(den * diff, Q);
            }
        }
        let inv_den = mod_inverse(den, Q);
        let lambda = mod_norm(num * inv_den, Q);
        secret = mod_norm(secret + mod_norm(yj * lambda, Q), Q);
    }
    secret
}

pub fn run_vss(secret: i128) {
    println!("--- Feldman VSS Demonstration ---");
    let mut rng = rand::thread_rng();

    let coeffs = generate_polynomial(secret, THRESHOLD, &mut rng);
    println!("Polynomial coefficients: {:?}", coeffs);

    let shares = generate_shares(&coeffs);
    println!("Shares: {:?}", shares);

    let commitments = generate_commitments(&coeffs);
    println!("Commitments: {:?}", commitments);

    for share in &shares {
        let valid = verify_share(*share, &commitments);
        println!("Share {:?} valid: {}", share, valid);
    }

    let recovered = reconstruct_secret(&shares[0..THRESHOLD]);
    println!("Reconstructed secret (from first {} shares): {}", THRESHOLD, recovered);
}

