fn main() {
    let mut args = std::env::args();
    let program_name = args.next().unwrap();
    match aoc2021::parse_args(args) {
        Ok(app_params) => {
            if let Err(message) = aoc2021::run(app_params) {
                eprintln!("{}", message);
            }
        }
        Err(message) => {
            eprintln!("{}\nUsage: {} [options]", message, program_name);
        }
    };
}
