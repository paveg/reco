extern crate scopeguard;

mod reco;

use {
    scopeguard::guard,
    std::process,
};

fn _main() -> i32 {
    let result = 0;

    result
}

fn main() {
    // Go-like defer
    let _g = guard((), |_| {
        eprintln!("Error: {}", "error information.");
        process::exit(1);
    });

    // actual execution
    println!("{:#?}", reco::Reco {});

    process::exit(_main());
}
