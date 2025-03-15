use std::process::Command;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("tput failure")]
    Tput,
}

type Result<T> = std::result::Result<T, Error>;

fn tput_cols() -> Result<usize> {
    let output = Command::new("tput")
        .arg("cols")
        .output()
        .map_err(|_| Error::Tput)?;
    if !output.status.success() {
        return Err(Error::Tput);
    }
    String::from_utf8_lossy(&output.stdout)
        .trim_ascii()
        .parse()
        .map_err(|_| Error::Tput)
}

fn main() -> Result<()> {
    let cols = tput_cols()?;

    println!("{}", "-".repeat(cols));
    println!("{}", "-".repeat(cols));

    // readonly width=$(($(tput cols) - 40))

    // echo '┌────────────┬──────────────────────┬────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐'
    // echo '│ Date       │ Name                 │ Subject                                                                                                                                                │'
    // echo '│────────────┼──────────────────────┼────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤'
    // git --no-pager log --first-parent --format="│ %as │ %<(20)%an │ %<($width)%s │" "$@"
    // echo '└────────────┴──────────────────────┴────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘'
    Ok(())
}
