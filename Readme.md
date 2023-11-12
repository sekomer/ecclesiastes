<center>

## Ecclesiastes

Ecclesiastes is a book in the Old Testament of the Bible, traditionally attributed to King Solomon. It is a philosophical and reflective work that explores the meaning of life, the pursuit of wisdom, and the challenges of human existence.

What I like about it is the section 1:9, which says: "What has been will be again, what has been done will be done again; there is nothing new under the sun.". This phrase is always a reminder to me that we are not as important as we think we are. We are just monkeys who have learned to grab a stick and kill another.

That's why I chose this name for my project. I'm not the first to write a port scanner and I won't be the last.

Enough philosophy, let's get to the project!

<br />

### Description

This is a lightweight port scanner that can be used to scan for open ports on a given IP address. Its written in Rust and its only dependency is the argparse crate.

### Installation

Clone the repository and run `cargo build --release` to compile the program. The binary will be located in `./target/release/ecclesiastes`.

### Usage

```bash
ecclesiastes [OPTIONS] <IP_ADDRESS> <PORT_RANGE>
```

### Options

- -j, --jobs <JOBS> Number of threads to run in parallel. Defaults to 1024.

### Arguments

- `<IP_ADDRESS>` The IP address to scan. This argument is required.
- `<PORT_RANGE>` The range of ports to scan. Must be in the format of `START_PORT:END_PORT`. This argument is optional and defaults to `1:65535`.

```bash
ecclesiastes 172.20.10.1
ecclesiastes -j 2000 192.168.1.1 1:65535
```

### Binary Size

This is not related to project, but I will show you how to reduce the binary size when you are using compiled languages. The binary size is 692K on my arm64 machine. We can reduce it to 212K by using the [UPX](https://upx.github.io/) packer.

```bash
upx --best --lzma target/release/ecclesiastes
```

Voila! The binary size is now 212K.

### Author Note

This is my first Rust project and I'm sure there are many ways to improve the code. Feel free to correct me if you have any suggestions. I also plan to invest more time into Rust and hopefully create more projects in the future.

</center>
```
