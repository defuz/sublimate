# sublimate

[![Gitter](https://badges.gitter.im/defuz/sublimate.svg)](https://gitter.im/defuz/sublimate?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

## Dependencies

You will need `oniguruma` (`libonig-dev`) and `ncurses` (`libncurses5-dev`) libraries.

Installation on Ubuntu Linux: `$ sudo apt-get install libonig-dev libncurses5-dev`

## Compiling

Once you install all dependencies - follow this instructions to compile `sublimate`.

 1. Clone the project `$ git clone https://github.com/defuz/sublimate && cd sublimate`
 2. Build the project `$ cargo build --release` (**NOTE:** There is a large performance differnce when compiling without optimizations, so I recommend alwasy using `--release` to enable to them)
 3. Once complete, the binary will be located at `target/release/sublimate`

## Options

`sublimate` has the following options:

```
USAGE:
    sublimate [FLAGS] <file> --packages <PACKAGES PATH> --project <PROJECT PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --packages <PACKAGES PATH>    Sets packages path
        --project <PROJECT PATH>      Sets path to sublime project

ARGS:
    file    Sets a path to viewing file
```
You can also obtain this help by runing `sublimate` with `-h` flag.

## License

This project is dual-licensed under the terms of the MIT and Apache (version 2.0) licenses.
