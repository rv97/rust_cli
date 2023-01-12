use clap::{Parser, Subcommand, Args};

mod api;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "stringer - a simple cli to transform and inspect strings", long_about = "stringer is a super fancy CLI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Reverse(Reverse),

    Inspect(Inspect),
}

#[derive(Args)]
struct Reverse {
    string: Option<String>,
}

#[derive(Args)]
struct Inspect {
    string: Option<String>,
    #[arg(short = 'd', long = "digits")]
    only_digits: bool,
}


fn main () {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Reverse(name)) => {
            match name.string {
                Some(ref _name) => {
                    let reverse = api::stringer::reverse(_name);
                    println!("{}", reverse);
                }
                None => {
                    println!("Please provice a string to reverse");
                }
            }
        }
        Some(Commands::Inspect(name)) => {
            match name.string {
                Some(ref _name) => {
                    let (res, kind) = api::stringer::inspect(_name, name.only_digits);

                    let mut plural_s = "s";
                    if res == 1 {
                        plural_s = "";
                    }
                    println!("{:?} has {} {}{}", _name, res, kind, plural_s);
                }
                None => {
                    println!("Please enter a string to inspect!");
                }
            }
        }
        None => {}
    }
}
