use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    let path = "test.txt";
    let result = std::fs::read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading, `{}`: {}", path, err)))?;

    println!("{}", result);
    Ok(())
}
