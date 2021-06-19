//! Testing the build binaries of a bin crate.
//!
//!
//! # Description
//!
//! 'cargo' tests by default have no support for running tests on a build binary. This crate
//! solves this.
//!
//!
//! # How It Works
//!
//! There are some problems to overcome the cargo limitations.
//!
//! 1. Running cargo tests does not depend on the binary build, by default they are not
//!    compiled at test time.
//! 2. There are no standard facilities to locate and execute the build binaries
//!    in a test.
//!
//! BinTest solve these problems by running 'cargo build' at test time, parsing its output for
//! identifying and locating the build binaries. On request it creates a std::process::Command
//! for the binary which can be used for any further testing.
//!
//! BinTest will panic on any error and not offer any error handling. This is deliberate to
//! make things simple.
//!
//!
//! # Example
//!
//! ```rust
//! #[test]
//! fn test() {
//!   // BinTest::new() will run 'cargo build' and registers all build binaries
//!   let bintest = BinTest::new();
//!
//!   // List the executables build
//!   for (k,v) in bintest.list_binaries() {
//!     println!("{} @ {}", k, v);
//!   }
//!
//!   // BinTest::command() looks up binary by its name and creates a process::Command from it
//!   let command = bintest.command("name");
//!
//!   // this command can then be used for testing
//!   command.arg("help").spawn();
//!
//! }
//! ```
//!
use std::collections::BTreeMap;
use std::env::var_os as env;
use std::ffi::OsString;

/// re-exported for convinience
pub use std::process::{Command, Stdio};

use cargo_metadata::{camino::Utf8PathBuf, Message};

/// Access to binaries build by 'cargo build'
pub struct BinTest {
    build_binaries: BTreeMap<String, Utf8PathBuf>,
}

impl BinTest {
    /// Runs 'cargo build' and stores all build binaries
    pub fn new() -> BinTest {
        //PLANNED: figure out which profile
        let mut cargo_build = Command::new(env("CARGO").unwrap_or_else(|| OsString::from("cargo")))
            .arg("build")
            .arg("--message-format")
            .arg("json")
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute 'cargo build'");

        let mut build_binaries = BTreeMap::new();

        let reader = std::io::BufReader::new(cargo_build.stdout.take().unwrap());
        for message in cargo_metadata::Message::parse_stream(reader) {
            if let Message::CompilerArtifact(artifact) = message.unwrap() {
                if let Some(executable) = artifact.executable {
                    build_binaries.insert(
                        String::from(executable.file_name().expect("Missing filename")),
                        executable.to_path_buf(),
                    );
                }
            }
        }

        BinTest { build_binaries }
    }

    /// Gives an `(name, path)` iterator over all binaries found
    pub fn list_binaries(&self) -> std::collections::btree_map::Iter<'_, String, Utf8PathBuf> {
        self.build_binaries.iter()
    }

    /// Constructs a 'std::process::Command' for the given binary name
    pub fn command(&self, name: &str) -> Command {
        Command::new(
            self.build_binaries
                .get(name)
                .unwrap_or_else(|| panic!("no such binary <<{}>>", name)),
        )
    }
}
