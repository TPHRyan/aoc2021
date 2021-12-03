fn main() {
    match aoc2021::parse_args(std::env::args()) {
        Ok(app_params) => {
            if let Err(message) = aoc2021::run(app_params) {
                eprintln!("{}", message);
            }
        }
        Err(message) => eprintln!("{}", message),
    };
}
