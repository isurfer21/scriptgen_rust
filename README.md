# scriptgen_rust

ScriptGen is a script generator tool that simplifies the process of running scripts in any language. 

## Introduction

When a script is executed using an interpreter, the command line statement can become quite long as it includes the interpreter path, script path, and all required arguments. ScriptGen addresses this issue by consolidating the interpreter and script paths into a short command that can be easily accessed from anywhere in the system. This means that when calling the script via command, only the arguments need to be passed.

ScriptGen generates commands for all operating systems. For Windows, it generates batch and PowerShell scripts. For macOS, it generates Shell scripts and PowerShell scripts which can be used if PowerShell is installed. For Linux, it generates Shell scripts. This makes it easy to run scripts on any platform without having to manually create the appropriate command.

## Build

Clone the `scriptgen_rust` repository from GitHub, navigate to the cloned repository, and build the project using Cargo.

```
git clone https://github.com/isurfer21/scriptgen_rust.git
cd scriptgen_rust
cargo build
```

After running these commands, you should have a local copy of the ScriptGen project that is ready to use.

Post building the project, the executable binary can be found in the `.\target\debug\` directory. On Windows, the binary is named `scriptgen.exe`, while on macOS it is simply named `scriptgen`.

To publish crates to [crates.io](https://crates.io/), run this command.

```
cargo publish
```

## Usage

Based on the help menu of _ScriptGen_, here are all the possible commands that can be used with the tool:

Prints the help information for _ScriptGen_.
```
scriptgen -h
``` 
or 
```
scriptgen --help
```

When the above command is executed, _ScriptGen_ displays its help information. This includes a brief description of the tool, its usage instructions, and a list of available options. The help information provides an overview of how to use _ScriptGen_ and the different options that can be used to customize its behavior.

```
ScriptGen 1.0.0
Abhishek Kumar
It generates a shell script for windows and linux to execute interpereted scripts like binary
commands.

USAGE:
    scriptgen [OPTIONS]

OPTIONS:
    -h, --help                         Print help information
    -i, --interpreter <INTERPRETER>    Sets the interpreter to use
    -s, --script-path <SCRIPT_PATH>    Sets the script path to use
    -V, --version                      Print version information
```

Prints the version information for _ScriptGen_.
```
scriptgen -V
``` 
or 
```
scriptgen --version
```

Sets the interpreter and the script path to use when generating the script.
```
scriptgen -i <INTERPRETER> -s <SCRIPT_PATH>
``` 
or 
```
scriptgen --interpreter <INTERPRETER> --script-path <SCRIPT_PATH>
```

Note that `<INTERPRETER>` and `<SCRIPT_PATH>` are placeholders for the actual values that you want to use. 

### Examples

Here are some examples of how to use `scriptgen` with different interpreters and script paths:

To generate a script using Node as the interpreter and a script named `sample.js`:
```
scriptgen -i node -s sample.js
```

To generate a script using Python as the interpreter and a script named `sample.py`:
```
scriptgen -i python -s sample.py
```

To generate a script using Java as the interpreter and a script named `sample.java`:
```
scriptgen -i java -s sample.java
```

In each of these examples, the `-i` option is used to specify the interpreter and the `-s` option is used to specify the script path.

## Installation

Here are a few methods for setting up this tool on your computer.

### Manual installation

Compiled binary versions of _ScriptGen_ are uploaded to GitHub when a release is made. You can install _ScriptGen_ manually by downloading a release, extracting it, and copying the binary to a directory in your `$PATH`. 

On macOS or Linux, this could be `/usr/local/bin`, while on Windows, you could copy the binary to `C:\Windows\system32`.

### Using Cargo

If you already have a Rust environment set up, you can use the `cargo install` command:
```
cargo install scriptgen
```
Cargo will build the _ScriptGen_ binary and place it in `$HOME/.cargo` on macOS or Linux, or `%USERPROFILE%\.cargo\bin` on Windows.