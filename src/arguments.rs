use clap::Parser;

const LONGEST_EN_WORD: usize = 45;

#[derive(Parser)]
struct Args {
    #[arg(required = true)]
    pub characters: String
}

pub fn parse_args() -> String {
    let args = Args::parse();

    if args.characters.len() > LONGEST_EN_WORD {
        println!("Too many input characters, MAX is 45 characters!");
        std::process::exit(0);
    }

    args.characters
}
