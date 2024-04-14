use cmdline_rs::cat;

fn main() {
    let config = cat::get_args()
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        })
        .validate_config();
    if let Err(e) = cat::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
