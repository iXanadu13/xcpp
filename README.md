[English](README.md) | [中文](README-zh.md)

# xcpp

Create a pre-configured VSCode C++ project with one command in Windows.

The generated multi-files C++ project uses g++ for compilation, and gdb for debugging. It also generates a Makefile and initializes `Git`.

## Preparation

### Configure mingw64

Download from: [https://github.com/niXman/mingw-builds-binaries/releases](https://github.com/niXman/mingw-builds-binaries/releases)

I chose `x86_64-14.2.0-release-posix-seh-ucrt-rt_v12-rev0`. You can Google the differences between versions if needed.

After downloading, extract the files and place them in a path without Chinese characters or spaces. My mingw64 path is `E:/Environment/mingw64_14_2_0/bin`.

Lastly, you may choose to add the mingw64 path (`E:/Environment/mingw64_14_2_0/bin`) to the system's Path environment variable.

## Setup

Download the latest version of xcpp from the [GitHub release page](https://github.com/iXanadu13/xcpp/releases/latest), extract it, and add the path to `xcpp.exe` to your environment variables.

Run the following command in the command prompt (an example mingw64 path: `E:/Environment/mingw64_14_2_0/bin`). The default configuration will be written to `%appdata%\xcpp\config\config.toml`:

```
xcpp store --std=c++20 --path <MINGW64_PATH>
```

You can then create a new C++ project directly by running `xcpp new project_name`.

**Do not use paths with Chinese characters or spaces, and avoid using Chinese project names, as this may prevent proper debugging in VSCode.**

You can also pass specific parameters (e.g., `--std`, `--path`) when creating a new C++ project. Command-line arguments take priority over the configuration file.

```
xcpp new hello_cpp --std=c++20 --path <MINGW64_PATH>
```

After successfully creating the project, you need to install the following C/C++ extensions in VSCode:
- `C/C++`
- `C/C++ Extension Pack`
- `C/C++ Themes`

**If you see an error about missing tasks, simply restart VSCode.**

If you run with `RUST_LOG=info`, you will see the log output. For example, in `Git Bash`:

```bash
RUST_LOG=info ./xcpp.exe new test1 --std=c++17 --path E:/Environment/mingw64_14_2_0/bin
```

Make sure to replace `./xcpp.exe` and `E:/Environment/mingw64_14_2_0/bin` with the correct paths.

[Usage Example](example.gif) (with `mingw64` and `xcpp` PATH already configured)

## Usage

Use `xcpp.exe --help` or `xcpp.exe <SUBCOMMAND> --help` to display help information.

```
USAGE:
    xcpp.exe <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    clear    Delete the config file at `%appdata%\xcpp\config\config.toml`
    help     Prints this message or the help of the given subcommand(s)
    new      Create a new cpp project
    store    Store the config to `%appdata%\xcpp\config\config.toml`
```
