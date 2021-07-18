#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(pith =>
        (version: "1.0.0")
        (about: "The interpreter for the pith esolang")
        (@arg INPUT: +required "The input file to be run")
        (@arg STDIN: "STDIN for the program")
        (@arg numbers: -n "STDIN is a sequence of space-seperated numbers, instead of a sequence of characters")
    )
    .get_matches();

    let file = matches.value_of("INPUT").unwrap();
    let stdin = matches.value_of("STDIN").unwrap_or_default();
    let stdin = if matches.is_present("numbers") {
        stdin
            .split(' ')
            .map(|n| n.parse().expect("Invalid number(s) passed to STDIN"))
            .collect()
    } else {
        stdin.as_bytes().to_owned()
    };
    if let Ok(program) = std::fs::read_to_string(file) {
        pith_lib::parse(&program, &stdin);
    } else {
        panic!("File {} not found", file);
    }
}
