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
//!
//! # Example
//!
//!  #[test]
//!  fn test() {
//!    // BinTest::new() will run 'cargo build' and registers all build binaries
//!    let bintest = BinTest::new();
//!
//!    // BinTest::command() looks up binary by its name and creates a process::Command from it
//!    let command = bintest.command("name");
//!
//!    //WIP: this command can then be used for testing
//!
//!  }
//!
//!
//!
use std::collections::BTreeMap;
use std::env::var_os as env;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
pub use std::process::{Command, Stdio};

use cargo_metadata::{Message, MetadataCommand};

pub struct BinTest {
    build_binaries: BTreeMap<OsString, PathBuf>,
}

impl BinTest {
    pub fn new() -> BinTest {
        //TODO: figure out which profile
        let mut cargo_build = Command::new(env("CARGO").unwrap_or_else(|| OsString::from("cargo")))
            .arg("build")
            .arg("--message-format")
            .arg("json")
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute 'cargo build'");

        let build_binaries = BTreeMap::<OsString, PathBuf>::new();

        let reader = std::io::BufReader::new(cargo_build.stdout.take().unwrap());
        for message in cargo_metadata::Message::parse_stream(reader) {
            if let Message::CompilerArtifact(artifact) = message.unwrap() {
                println!("{:?}", artifact);
            }
        }

        BinTest { build_binaries }
    }

    //PLANNED: pub fn binaries(name: OsStr) -> Iterator {}

    //WIP: pub fn command(name: OsStr) -> Command {}
}
