# Build
Requirements:
- Git
- Rust
- Cargo

```console
git clone git@github.com:R3ZV/acrolern.git
cd acrolern
cargo build --release

cp ./target/release/acrolern <destination>
```

# Usage

You have to first specify the options than the command.

```console
# starts the game with description enabled
./acrolern -d true play

# for all options and commands
./acrolern -h 
```

All the acronyms live inside the "acronyms.json" file inside the "src" directory.

You will be tested on the acronyms that you have a lower accuracy rating,
you can stop at any time by typing "quit" and the progress will be saved.

This is the format for "acronyms.json".
To add more acronyms just modify this file according to the format.
```JSON
[
    {
        "acronym": "IP",
        "meaning": "Internet Protocol",
        "description": "It is a communication protocol for relaying datagrams across network boundaries.",
        "score": 0,
        "tags": ["internet", "protocol"]
    }
]
```
