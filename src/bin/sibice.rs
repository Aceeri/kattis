use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

fn main() -> Result<(), Box<Error>> {
    let stdin = stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let words = lines[0].split_whitespace().collect::<Vec<&str>>();
    let count = words[0].parse::<usize>()?;
    let rest = &lines[1..=count];

    let width = words[1].parse::<f64>()?;
    let height = words[2].parse::<f64>()?;
    let maximum = (width.powf(2.0) + height.powf(2.0)).sqrt();

    let stdout = stdout();
    let mut stdout_handle = stdout.lock();
    for line in rest {
        let fits = line.trim().parse::<f64>()? <= maximum;
        if fits {
            stdout_handle.write(b"DA\n")?;
        } else {
            stdout_handle.write(b"NE\n")?;
        }
    }

    Ok(())
}
