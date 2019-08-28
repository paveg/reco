extern crate scopeguard;

use {
    scopeguard::guard,
    std::process,
};

fn reco() {
    // TODO: Implement this.
}

fn _main() -> i32 {
    let result = 0;

    result
}

fn main() {
    let _g = guard((), |_| {
        eprintln!("Error: {}", "error information.");
        process::exit(1);
    });
    reco();
    process::exit(_main());
}
