/// This is the main entry point for the jld-common application.
fn main() {
    // Call the `run()` function from the `jld-common` module.
    if let Err(err) = jld_common::run() {
        eprintln!("Error running jld-common: {}", err);
        std::process::exit(1);
    }
}
