use anyhow::Result;
use dockerfile::{Arg, Cmd, Copy, Dockerfile};
use std::{fs::File, io::Write};

pub fn dockerfile() -> Result<()> {
    let dockerfile = Dockerfile::base("rust:${RUST_VERSION}-slim")
        .push_initial_arg(Arg::new("RUST_VERSION=1.31"))
        .push(Copy::new("/static ./static"))
        .push(Cmd::new("echo 'Hello. Goodbye.'"))
        .finish();

    // Write it out as a string.
    let output = dockerfile.to_string();
    println!("{:#?}", output);

    let mut file = File::create("nginx.Dockerfile")?;
    write!(&mut file, "{}", output)?;
    assert_eq!(
        output,
        r##"ARG RUST_VERSION=1.31
FROM rust:${RUST_VERSION}-slim
COPY /static ./static
CMD echo 'Hello. Goodbye.'
"##
    );
    Ok(())
}
