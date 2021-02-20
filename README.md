# vpp-api-transport

This is a Rust library for interfacing with the VPP API. 

WARNING: quite likely the interfaces may change. This version is
to test things out and encourage the feedback.

The idea is to have entity that implements Read and Write traits,
as well as eventually AsRawFd. This should allow to use this
in place where you would use a regular socket in your code.

# Prerequisites

## Rust and OS dependencies

### Rust

As per https://www.rust-lang.org/tools/install:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### OS dependencies

On Ubuntu:

```
sudo apt-get install build-essential libclang-10-dev
```

### VPP client libraries

## Option 1 - your local development VPP tree at arbitrary location

Define the environment variable *VPP_LIB_DIR* to point to the folder where
the file *libvppapiclient.so* is located. You might also have to set the
*LD_LIBRARY_PATH* to the same location as well.


## Option 2 - VPP client from packagecloud.io:

Setup the repository from https://packagecloud.io/fdio/master or the
branch-specific repositories as per instructions, then install
the package *python3-vpp-api*, cargo build will find the library there.

# Usage

See the src/vpp-api-transport-test.rs, more and better docs will
come as we go :-) There is no point to document much here yet,
as the interfaces will change.

# Running *cargo test*

```
sudo apt-get install vpp vpp-plugin-core python3-vpp-api
```


