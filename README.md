Testing the build executables of a bin crate.


# Description

'cargo' tests by default have no support for running tests on a build excecutable.
This crate solves this.


# How It Works

There are some problems to overcome the cargo limitations.

1. Running cargo tests does not depend on the executables build, by default they are not
   compiled at test time.
2. There are no standard facilities to locate and execute them in a test.

BinTest solve these problems by running 'cargo build' at test time, parsing its output for
identifying and locating the build executables. On request it creates a std::process::Command
for the binary which can be used for any further testing.

# See Also

The 'testcall' crate uses this to build tests and assertions on top of the commands created by
bintest. The 'testpath' crate lets you run test in specially created temporary directories to
provide an filesystem environment for tests.
