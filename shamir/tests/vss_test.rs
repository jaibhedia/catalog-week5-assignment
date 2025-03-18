use rand::thread_rng;
use shamir::algos::vss;

#[test]
fn test_verify_shares() {
    let secret = 1234;
    let mut rng = thread_rng();
    let coeffs = vss::generate_polynomial(secret, vss::THRESHOLD, &mut rng);
    let shares = vss::generate_shares(&coeffs);
    let commitments = vss::generate_commitments(&coeffs);
    for share in shares {
        assert!(
            vss::verify_share(share, &commitments),
            "Share {:?} failed verification",
            share
        );
    }
}

#[test]
fn test_reconstruct_secret() {
    let secret = 1234;
    let mut rng = thread_rng();
    let coeffs = vss::generate_polynomial(secret, vss::THRESHOLD, &mut rng);
    let shares = vss::generate_shares(&coeffs);
    let recovered = vss::reconstruct_secret(&shares[0..vss::THRESHOLD]);
    assert_eq!(recovered, secret, "Reconstructed secret did not match original");
}
