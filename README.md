# vpp-api-transport

This is a Rust library for interfacing with the VPP API. 

WARNING: quite likely the interfaces may change. This version
(until at least 0.2.0) is to test things out and encourage the feedback.

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


# Running the examples

Same prerequisites as for cargo test, and then:

```

$ cargo run --release --example vpp-api-transport-test -- -h

...

    Finished release [optimized] target(s) in 0.04s
     Running `target/release/examples/vpp-api-transport-test -h`
vpp-api-transport version heads/main-0-gaca41b6
Andrew Yourtchenko <ayourtch@gmail.com>
This program is a minimum test of vpp-api-transport crate To make it somewhat useful, it can also
bench the cli_inband API execution time for various commands

USAGE:
    vpp-api-transport-test [FLAGS] [OPTIONS]

FLAGS:
    -h, --help           Prints help information
    -n, --nonblocking    set non-blocking mode for the connection
    -v, --verbose        A level of verbosity, and can be used multiple times
    -V, --version        Prints version information

OPTIONS:
    -c, --command <command>
            Run the bench using this CLI, else use "show version"

    -o, --options-override <options-override>    Override options from this yaml/json file
    -r, --repeat-count <repeat-count>            repeat count for the command [default: 100000]
    -s, --socket-path <socket-path>
            Use AF_UNIX socket if this path is mentioned, else use shared memory transport

$
$ cargo run --release --example vpp-api-transport-test -- -s /run/vpp/api.sock -r 1000000

...

    Finished release [optimized] target(s) in 0.04s
     Running `target/release/examples/vpp-api-transport-test -s /run/vpp/api.sock -r 1000000`
Open success!
Table: MsgSockClntCreateReplyHdr { _vl_msg_id: 16, client_index: 0, context: 124, response: 0, index: 2147483649, count: 1563 }
Connect result: 0
Starting 1000000 requests of 'show version'
Still running... 270492 iterations in 5.000010229s: 54098.2893257197 per second
Still running... 539286 iterations in 10.000028579s: 53928.445877894526 per second
Still running... 854847 iterations in 15.0000489s: 56989.61421385766 per second
Ran 1000000 operations in 17.455633583s : 57288.09528712252 per second
$
$ ./target/release/examples/vpp-api-transport-test -s /run/vpp/api.sock -c "show clock" -v -v -v -r 3
Open success!
Table: MsgSockClntCreateReplyHdr { _vl_msg_id: 16, client_index: 0, context: 124, response: 0, index: 2147483649, count: 1563 }
Connect result: 0
Starting 3 requests of 'show clock'
Result:
Time now 45666.272925, Sun, 21 Feb 2021 22:20:42 GMT

Result:
Time now 45666.272982, Sun, 21 Feb 2021 22:20:42 GMT

Result:
Time now 45666.273032, Sun, 21 Feb 2021 22:20:42 GMT

Ran 3 operations in 173.171µs : 17323.9168221007 per second
$
```

