# RPG Map
A simple RPG map-maker program.

rpgmap is a simple grid-based RPG map-maker program.

This program currently has 3 implementations: one in C++,
one in Python and one in Rust. I did this mostly as a test
to see how similar the code would be when implementing using
these languages.

Currently, the Rust implementation is the fastest.

Example maps look like this:
![Example halls map](doc/images/halls_60x60.png)

# Install
## Rust
Make sure that you have the Rust compiler and cargo installed.
```
cd rust
cargo build --release
./target/release/rpgmap --help
```

## Python
Make sure that you have Python3 installed on your system.
```
cd python
pip3 install --user .
```

# Usage
For argument descriptions:
```
rpgmap --help
```

Simple example:
```
rpgmap -x 200 -y 200 example.png
```
