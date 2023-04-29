// Here is a Rust program that uses the `clap` library to parse command line options. It takes two command line arguments: `--interpreter` (or `-i`) and `--script-path` (or `-s`). It then extracts the script name from the script path and creates three files with the same name but different extensions: `.cmd`, `.ps1`, and `.sh`. The content of these files is based on a template where the interpreter and script path values are replaced.
// You can run this program with the `--help` (or `-h`) option to see the help menu and with the `--version` (or `-V`) option to see the version information.

use std::fs;
use std::path::Path;
use clap::{Arg, App};

fn main() {
    let matches = App::new("ScriptGen")
        .version("1.0.0")
        .author("Abhishek Kumar")
        .about("It generates a shell script for windows and linux to execute interpereted scripts like binary commands.")
        .arg(Arg::with_name("interpreter")
            .short('i')
            .long("interpreter")
            .value_name("INTERPRETER")
            .help("Sets the interpreter to use")
            .takes_value(true))
        .arg(Arg::with_name("script-path")
            .short('s')
            .long("script-path")
            .value_name("SCRIPT_PATH")
            .help("Sets the script path to use")
            .takes_value(true))
        .get_matches();

    let interpreter = matches.value_of("interpreter").unwrap();
    let script_path = matches.value_of("script-path").unwrap();
    let script_name = Path::new(script_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    let cmd_content = format!("@ECHO off
SETLOCAL
SET dp0=%~dp0

SET \"_prog={}\"
SET PATHEXT=%PATHEXT:;.{};=;%

ENDLOCAL & GOTO #_undefined_# 2>NUL || title %COMSPEC% & \"%_prog%\"  \"%dp0%\\{}\" %*
", interpreter, interpreter, script_path);

    let ps1_content = format!("#!/usr/bin/env pwsh
$basedir=Split-Path $MyInvocation.MyCommand.Definition -Parent

$exe=\"\"
if ($PSVersionTable.PSVersion -lt \"6.0\" -or $IsWindows) {{
  # Fix case when both the Windows and Linux builds of Node
  # are installed in the same directory
  $exe=\".exe\"
}}
$ret=0

# Support pipeline input
if ($MyInvocation.ExpectingInput) {{
  $input | & \"{}$exe\"  \"$basedir/{}\" $args
}} else {{
  & \"{}$exe\"  \"$basedir/{}\" $args
}}
$ret=$LASTEXITCODE

exit $ret
", interpreter, script_path, interpreter, script_path);

    let sh_content = format!("#!/bin/sh
basedir=$(dirname \"$(echo \"$0\" | sed -e 's,\\,/,g')\")

case `uname` in
    *CYGWIN*|*MINGW*|*MSYS*) basedir=`cygpath -w \"$basedir\"`;;
esac

exec {}  \"$basedir/{}\" \"$@\"
", interpreter, script_path);

    fs::write(format!("{}.cmd", script_name), cmd_content).expect("Unable to write file");
    fs::write(format!("{}.ps1", script_name), ps1_content).expect("Unable to write file");
    fs::write(format!("{}.sh", script_name), sh_content).expect("Unable to write file");
}
