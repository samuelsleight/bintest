//! Testing the executables build by a bin crate.
//!
//!
//! # Description
//!
//! 'cargo' tests by default have no support for running tests on a build executable.
//! This crate solves this.
//!
//!
//! # How It Works
//!
//! There are some problems to overcome the cargo limitations.
//!
//! 1. Running cargo tests does not depend on the executables build, by default they are not
//!    compiled at test time.
//! 2. There are no standard facilities to locate and execute them in a test.
//!
//! BinTest solve these problems by running 'cargo build' at test time, parsing its output for
//! identifying and locating the build executables. On request it creates a std::process::Command
//! for the executable which can be used for any further testing.
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
//!   // BinTest::new() will run 'cargo build' and registers all build executables
//!   let executables = BinTest::new();
//!
//!   // List the executables build
//!   for (k,v) in bintest.list_executables() {
//!     println!("{} @ {}", k, v);
//!   }
//!
//!   // BinTest::command() looks up executable by its name and creates a process::Command from it
//!   let command = executables.command("name");
//!
//!   // this command can then be used for testing
//!   command.arg("help").spawn();
//!
//! }
//! ```
//!
//!
//! # See Also
//!
//! The testcall crate uses this to build tests and assertions on top of the commands created by
//! bintest.
//!
use std::collections::BTreeMap;
use std::env::var_os as env;
use std::ffi::OsString;

pub use std::process::{Command, Stdio};

pub use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::Message;

/// Access to binaries build by 'cargo build'
pub struct BinTest {
    build_executables: BTreeMap<String, Utf8PathBuf>,
}

//PLANNED: needs some better way to figure out what profile is active
#[cfg(not(debug_assertions))]
const RELEASE_BUILD: bool = true;

#[cfg(debug_assertions)]
const RELEASE_BUILD: bool = false;

impl BinTest {
    /// Runs 'cargo build' and register all build executables.
    /// Executables are identified by their name, without path and filename extension.
    pub fn new() -> BinTest {
        let mut cargo_build = Command::new(env("CARGO").unwrap_or_else(|| OsString::from("cargo")));

        cargo_build
            .args(["build", "--message-format", "json"])
            .stdout(Stdio::piped());

        if RELEASE_BUILD {
            cargo_build.arg("--release");
        }

        let mut cargo_result = cargo_build.spawn().expect("'cargo build' success");

        let mut build_executables = BTreeMap::new();

        let reader = std::io::BufReader::new(cargo_result.stdout.take().unwrap());
        for message in cargo_metadata::Message::parse_stream(reader) {
            if let Message::CompilerArtifact(artifact) = message.unwrap() {
                if let Some(executable) = artifact.executable {
                    build_executables.insert(
                        String::from(executable.file_stem().expect("filename")),
                        executable.to_path_buf(),
                    );
                }
            }
        }

        BinTest { build_executables }
    }

    /// Gives an `(name, path)` iterator over all executables found
    pub fn list_executables(&self) -> std::collections::btree_map::Iter<'_, String, Utf8PathBuf> {
        self.build_executables.iter()
    }

    /// Constructs a 'std::process::Command' for the given executable name
    pub fn command(&self, name: &str) -> Command {
        Command::new(
            self.build_executables
                .get(name)
                .unwrap_or_else(|| panic!("no such executable <<{}>>", name)),
        )
    }
}

impl Default for BinTest {
    fn default() -> Self {
        Self::new()
    }
}

