use std::{
    io::{self, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

// This function is adapted from <https://github.com/rust-lang/rust-bindgen/blob/4f9fa49ca907b831fdc3aecdfaec36b16d03c8d8/bindgen/lib.rs#L986>
pub fn rustfmt(input: &str) -> io::Result<String> {
    let rustfmt = rustfmt_path()?;
    let mut cmd = Command::new(&*rustfmt);

    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    let mut child = cmd.spawn()?;
    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    let source = input.to_string();

    // Write to stdin in a new thread, so that we can read from stdout on this
    // thread. This keeps the child from blocking on writing to its stdout which
    // might block us from writing to its stdin.
    let stdin_handle = ::std::thread::spawn(move || {
        let _ = child_stdin.write_all(source.as_bytes());
        source
    });

    let mut output = vec![];
    io::copy(&mut child_stdout, &mut output)?;

    let status = child.wait()?;
    let source = stdin_handle.join().expect(
        "The thread writing to rustfmt's stdin doesn't do \
         anything that could panic",
    );

    match String::from_utf8(output) {
        Ok(bindings) => match status.code() {
            Some(0) => Ok(bindings),
            Some(2) => Err(io::Error::new(
                io::ErrorKind::Other,
                "Rustfmt parsing errors.".to_string(),
            )),
            Some(3) => {
                eprintln!("Warning: Some lines could not be formatted by rustfmt.");
                Ok(bindings)
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Internal rustfmt error".to_string(),
            )),
        },
        _ => Ok(source),
    }
}

fn rustfmt_path() -> io::Result<PathBuf> {
    if let Ok(rustfmt) = std::env::var("RUSTFMT") {
        return Ok(rustfmt.into());
    }
    match which::which("rustfmt") {
        Ok(p) => Ok(p),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}
