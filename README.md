<h1 align="center">rust-c-compiler</h1>

This repository contains a C compiler written in Rust (rcc). It is a toy compiler built as a hobby project. It is not fully functional, it can only handle a subset of C.

# Setup

Run the following command to setup the directory structure:

```sh
make setup-dirs
```

# Testing

There are scripts to compile the testing C source files. To compile all source files:

```sh
./compile-all.sh rcc
```

Then to verify that the binaries return the correct result:

```sh
./verify.sh rust-binaries
```

To verify the status of a single binary:

```sh
./verify.sh rust-binaries return-ten 10
```

To verify that the test assertions are correct, compile the source files with gcc and then verify the binaries:

```sh
./compile-all.sh gcc && ./verify.sh gcc-binaries
```

