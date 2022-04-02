use clap::Parser;

/// Extracts all archives into a flattened directory
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Directory to extract
    #[clap(short, long)]
    dir: String,

    /// Directory to extract
    #[clap(short, long)]
    out: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args.dir);
    println!("{:?}", args.out);
}
