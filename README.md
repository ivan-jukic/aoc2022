# Advent of Code 2022

Here we go again, into the madness!

## Running the code

To run the code this repo depends on `Nix` to set up the environment. It provides the binaries required for building and running solutions.

Repo can handle dev shell via flakes `nix develop` or the standard `nix-shell` commands.

There's also a simple way to test and the code. `.cargo/config.toml` defines a couple of aliases to facilitate that. To test the code one can run:

```bash
cargo tb dayXY
```

Where `XY` is the number of the day, zero prefixed for values less than 10; e.g. 01, 02, 03 ... 24, 25. And to run the _release_ version of the code:

```bash
cargo rb dayXY
```

Release should be a bit faster to run than _debug_; might make a difference for some calculation intesive solutions.

## Setting up a new day solution

Made slightly easier with a `bash` script `bin/new.sh`, which reads the folder names in the `src` directory, and figures out which day solution it must add next. Then adds all the required files to `src` and a new binary target to `Cargo.toml`.

To do this, one needs to simply run:

```bash
make add
```

## About things...

Not sure if there's anything else worth mentioning, but depending on the time I'll have available, lets see how far I get this year.
