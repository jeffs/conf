fn main() {
    match cl::main() {
        Ok(()) => (),
        Err(cl::Error::Args(args)) => {
            eprintln!("Error: bad arguments: {args:?}");
            eprintln!("Usage: cl [-y|--yesterday]");
            std::process::exit(2);
        }
        Err(cl::Error::Dir(path, err)) => {
            eprintln!("Error: {}: can't chdir: {err}", path.display());
            std::process::exit(1);
        }
    }
}
