use shamir::algos::sss;

#[test]
fn test_generate_polynomial() {
    let secret = 1234;
    let threshold = 3;
    let poly = sss::generate_polynomial(secret, threshold).expect("Failed to generate polynomial");
    assert_eq!(poly[0], secret);
    assert_eq!(poly.len(), threshold);
}

#[test]
fn test_evaluate_polynomial() {
    let coeffs = vec![5, 2, 3];
    let x = 2;
    let result = sss::evaluate_polynomial(&coeffs, x);
    assert_eq!(result, 21);
}

#[test]
fn test_generate_shares() {
    let secret = 9876;
    let threshold = 3;
    let num_shares = 5;
    let shares = sss::generate_shares(secret, threshold, num_shares)
        .expect("Failed to generate shares");
    assert_eq!(shares.len(), num_shares);
}

#[test]
fn test_reconstruct_secret() {
    let secret = 7777;
    let threshold = 3;
    let num_shares = 5;
    let shares = sss::generate_shares(secret, threshold, num_shares)
        .expect("Failed to generate shares");
    let reconstructed = sss::reconstruct_secret(&shares[..threshold], threshold)
        .expect("Failed to reconstruct secret");
    assert_eq!(reconstructed, secret);
}
