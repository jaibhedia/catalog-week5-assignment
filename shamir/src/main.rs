mod algos;

fn main() -> Result<(), algos::sss::ShamirError> {
    algos::sss::run_shamir()
}

