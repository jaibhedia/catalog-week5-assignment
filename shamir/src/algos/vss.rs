use rand::Rng;

const SECRET: i128 = 1234;
const PRIME: i128 = 2003; // A nearby prime number
const THRESHOLD: usize = 3; // t
const SHARES_COUNT: usize = 5; // n
const g: i32 = 3; // Generator for the finite field

fn mod_pow(base: i128, exponent: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }

    let mut result: i128 = 1;
    let mut base = base % modulus;
    let mut exp = exponent;

    if exp < 0 {
        // For negative exponents, we'd need modular multiplicative inverse
        panic!("Negative exponents not supported");
    }

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    result
}

// Calculate modular multiplicative inverse using extended Euclidean algorithm
fn mod_inverse(a: i128, m: i128) -> i128 {
    // Ensure a is positive
    let a_pos = if a < 0 { a + m } else { a } % m;

    let mut t = 0;
    let mut newt = 1;
    let mut r = m;
    let mut newr = a_pos;

    while newr != 0 {
        let quotient = r / newr;
        (t, newt) = (newt, t - quotient * newt);
        (r, newr) = (newr, r - quotient * newr);
    }

    if r > 1 {
        panic!(
            "Modular inverse does not exist for {} mod {}, gcd={}",
            a, m, r
        );
    }

    if t < 0 {
        t += m;
    }

    t
}

fn generate_polynomial(coeffs: &mut Vec<i128>) {
    let mut rng = rand::thread_rng(); // Fixed method name
                                      // First push in the secret that's the constant value
    coeffs.push(SECRET);

    // Then push the remaining constants (positive values only)
    for _ in 1..THRESHOLD {
        let rand_val: i128 = rng.gen_range(0..PRIME); // Fixed method name
        coeffs.push(rand_val);
    }
}

// Generate shares from polynomial
fn generate_shares(coeffs: &Vec<i128>, shares: &mut Vec<(i128, i128)>) {
    for x in 1..SHARES_COUNT + 1 {
        let x_i128: i128 = x as i128;
        let mut fx: i128 = 0;

        // Evaluate polynomial at point x
        for (power, coeff) in coeffs.iter().enumerate() {
            fx = (fx + coeff * mod_pow(x_i128, power as i128, PRIME)) % PRIME;
        }

        shares.push((x_i128, fx));
    }
}

// Generate commitments for each coefficient
fn generate_commitments(commitments: &mut Vec<i128>, coeffs: &Vec<i128>) {
    // formula used is Cx = g^coeff[x] mod PRIME
    for c in coeffs {
        let cmt = mod_pow(g as i128, *c, PRIME);
        commitments.push(cmt);
    }
}

// Verify shares using commitments
fn verify_shares(commitments: &Vec<i128>, shares: &Vec<(i128, i128)>) -> bool {
    let mut all_verified = true;

    for (x, y) in shares {
        // LHS: g^y mod PRIME
        let lhs = mod_pow(g.into(), *y, PRIME);

        // RHS: Compute C_0 * C_1^x * C_2^(x^2) * ... * C_t-1^(x^(t-1)) mod PRIME
        let mut rhs = 1;
        for j in 0..commitments.len() {
            let power = mod_pow(*x, j as i128, PRIME);
            let term = mod_pow(commitments[j], power, PRIME);
            rhs = (rhs * term) % PRIME;
        }

        println!(
            "For point ({}, {}): LHS = {}, RHS = {}, verified: {}",
            x,
            y,
            lhs,
            rhs,
            lhs == rhs
        );

        if lhs != rhs {
            all_verified = false;
        }
    }

    all_verified
}

// Calculate lagrange basis polynomial for interpolation
fn lagrange_basis(x: i128, x_values: &[i128], j: usize) -> i128 {
    let x_j = x_values[j];
    let mut numerator = 1;
    let mut denominator = 1;

    // First compute the entire numerator and denominator separately
    for (m, x_m) in x_values.iter().enumerate() {
        if m != j {
            // Numerator: (x - x_m) mod PRIME
            let mut factor = (x - x_m) % PRIME;
            if factor < 0 {
                factor += PRIME;
            }
            numerator = (numerator * factor) % PRIME;

            // Denominator: (x_j - x_m) mod PRIME
            let mut diff = (x_j - x_m) % PRIME;
            if diff < 0 {
                diff += PRIME;
            }
            denominator = (denominator * diff) % PRIME;
        }
    }

    // Now calculate the modular inverse of the denominator
    let denominator_inv = mod_inverse(denominator, PRIME);

    // Calculate the final result
    (numerator * denominator_inv) % PRIME
}

// Reconstruct the secret from shares
fn reconstruct_secret(shares: &[(i128, i128)], threshold: usize) -> i128 {
    if shares.len() < threshold {
        panic!("Not enough shares to reconstruct the secret!");
    }

    // Extract x and y values from shares
    let selected_shares = &shares[0..threshold];
    let x_values: Vec<i128> = selected_shares.iter().map(|(x, _)| *x).collect();
    let y_values: Vec<i128> = selected_shares.iter().map(|(_, y)| *y).collect();

    let mut secret = 0;
    for j in 0..threshold {
        println!(
            "Computing lagrange basis for point {} with x_values {:?}, j={}",
            0, x_values, j
        );
        let basis = lagrange_basis(0, &x_values, j);
        let term = (y_values[j] * basis) % PRIME;
        secret = (secret + term) % PRIME;
    }

    // Ensure the result is positive
    if secret < 0 {
        secret += PRIME;
    }

    secret
}

pub fn main() {
    println!("=== Verifiable Secret Sharing (VSS) Demonstration ===");
    println!(
        "Secret: {}, Threshold: {}, Total Shares: {}\n",
        SECRET, THRESHOLD, SHARES_COUNT
    );

    let mut coeffs = Vec::<i128>::with_capacity(THRESHOLD);
    let mut commitments = Vec::<i128>::with_capacity(THRESHOLD);
    let mut shares = Vec::<(i128, i128)>::with_capacity(SHARES_COUNT);

    // 1. Generate polynomial
    generate_polynomial(&mut coeffs);
    println!("Polynomial coefficients: {:?}", coeffs);

    // 2. Compute shares
    generate_shares(&coeffs, &mut shares);
    println!("Generated shares: {:?}\n", shares);

    // 3. Generate commitments
    generate_commitments(&mut commitments, &coeffs);
    println!("Generated commitments: {:?}\n", commitments);

    // 4. Verify shares
    println!("=== Verification of Shares ===");
    let all_verified = verify_shares(&commitments, &shares);
    println!("All shares verified: {}\n", all_verified);

    // 5. Reconstruct secret from shares
    println!("=== Secret Reconstruction ===");
    println!("Using first {} shares for reconstruction:", THRESHOLD);

    println!("Direct check: secret = {}", coeffs[0]);

    // Try to reconstruct with just the threshold number of shares
    let reconstructed_secret = reconstruct_secret(&shares[0..THRESHOLD], THRESHOLD);
    println!("Reconstructed secret: {}", reconstructed_secret);
    println!(
        "Original secret matched: {}\n",
        reconstructed_secret == SECRET
    );

    // Try different combinations of shares
    println!("Using different combinations of shares:");
    let combinations = vec![
        vec![0, 1, 2], // First three shares
        vec![2, 3, 4], // Last three shares
        vec![0, 2, 4], // Mixed shares
    ];

    for (i, combo) in combinations.iter().enumerate() {
        let selected_shares: Vec<(i128, i128)> = combo.iter().map(|&idx| shares[idx]).collect();

        let reconstructed = reconstruct_secret(&selected_shares, THRESHOLD);
        println!(
            "Combination {}: Shares {:?} -> Secret = {} (Matched: {})",
            i + 1,
            combo,
            reconstructed,
            reconstructed == SECRET
        );
    }
}