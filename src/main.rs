use std::{
    error::Error,
    fs::{self, File},
    io::Read,
    path::PathBuf,
    process::Command,
};

use clap::Parser;
use regex::{Captures, Regex};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: PathBuf,
    #[clap(short, long)]
    inplace: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input = &mut File::open(&args.path)?;
    let mut input_content = String::with_capacity(input.metadata()?.len() as usize);
    input.read_to_string(&mut input_content)?;

    let regex = Regex::new(
        "```sh run\n\
        > (.*)\n\
        (?:[\\S\\s]*?\n)?\
        ```",
    )
    .expect("regex should be valid");

    let output_content = regex.replace_all(&input_content, replace_output);

    if args.inplace {
        fs::write(&args.path, output_content.as_ref())?;
    } else {
        print!("{}", output_content);
    }

    Ok(())
}

fn replace_output(captures: &Captures) -> String {
    let (_, [command]) = captures.extract();

    let output = Command::new("sh").arg("-c").arg(command).output();

    let output = match &output {
        Ok(output) => {
            let stdout = output.stdout.as_slice();
            match std::str::from_utf8(stdout) {
                Ok(output) => output,
                Err(_) => todo!(),
            }
        }
        Err(_) => todo!(),
    };
    let output = output.trim_end();

    return format!(
        "```sh run\n\
        > {command}\n\
        {output}\n\
        ```"
    );
}
