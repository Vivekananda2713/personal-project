use halp::cli::CliArgs;
use halp::error::Result;
use pretty_assertions::assert_eq;
use std::path::PathBuf;

/// Test binary.
const BIN: &str = env!("CARGO_BIN_EXE_halp-test");

#[test]
fn get_argument_help() -> Result<()> {
    let args = CliArgs {
        cmd: Some(BIN.to_string()),
        timeout: Some(10),
        config: Some(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("config")
                .join(concat!(env!("CARGO_PKG_NAME"), ".toml")),
        ),
        ..Default::default()
    };
    let mut output = Vec::new();
    halp::run(args, &mut output)?;
    println!("{}", String::from_utf8_lossy(&output));
    assert_eq!(
        r"(Â°ã­Â°)  checking 'test -v'
(Ãï¹Ã)      fail '-v' argument not found.
(Â°ã­Â°)  checking 'test -V'
\(^ã®^)/ success '-V' argument found!
---
halp 0.1.0
---
(Â°ã­Â°)  checking 'test -h'
\(^ã®^)/ success '-h' argument found!
---
Usage: test

Options:
  -h, --help     Print help
  -V, --version  Print version
---",
        String::from_utf8_lossy(&output)
            .replace('\r', "")
            .replace(BIN, "test")
            .replace(env!("CARGO_PKG_VERSION"), "0.1.0")
            .trim()
    );
    Ok(())
}
