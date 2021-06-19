Testing the build binaries of a bin crate.


# Description

'cargo' tests by default have no support for running tests on a build binary. This crate
solves this.


# How It Works

There are some problems to overcome the cargo limitations.

1. Running cargo tests does not depend on the binary build, by default they are not
   compiled at test time.
2. There are no standard facilities to locate and execute the build binaries
   in a test.

BinTest solve these problems by running 'cargo build' at test time, parsing its output for
identifying and locating the build binaries. On request it creates a std::process::Command
for the binary which can be used for any further testing.
