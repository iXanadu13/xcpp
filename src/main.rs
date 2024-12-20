use anyhow::Context;
use structopt::StructOpt;
use log::{error, info, warn};
use text_placeholder::Template;

use std::{collections::HashMap, fs::File, io::Write, path::{Path, PathBuf}, process::{exit, Command}, str::FromStr};

use serde_derive::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
struct MyConfig {
    std: String,
    mingw64_path: String,
}

const fn cpp_standards() -> [&'static str; 8] {
    ["c++98", "c++03", "c++11", "c++14", "c++17", "c++20", "c++23", "cfg"]
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(StructOpt)]
enum Cmd {
    /// Create a new cpp project.
    New {
        /// The name of your cpp project.
        name: String,
        /// Cpp standard, will be passed as `--std=<std>` when compile .cpp files.
        #[structopt(long, default_value = "cfg", possible_values = &cpp_standards())]
        std: String,
        /// The path of mingw64, e.g. `E:/Environment/mingw64_14_2_0/bin` in my Windows OS.
        #[structopt(long, default_value = "")]
        path: String,
    },
    /// Store the config to `%appdata%\xcpp\config\config.toml`.
    Store {
        /// Cpp standard, will be passed as `--std=<std>` when compile .cpp files.
        #[structopt(long, possible_values = &cpp_standards())]
        std: String,
        /// The path of mingw64, e.g. `E:/Environment/mingw64_14_2_0/bin` in my Windows OS.
        #[structopt(long, parse(from_os_str))]
        path: std::path::PathBuf,
    },
    /// Delete the config file at `%appdata%\xcpp\config\config.toml`.
    Clear {

    }
}

fn mkdir(path: &Path) {
    // 创建所有必要的父目录
    if let Err(e) = std::fs::create_dir_all(path) {
        panic!("Failed to create directories for {}: {:?}", path.display(), e);
    }
}

fn create_file(path: &Path) -> File {
    if let Some(parent) = path.parent() {
        mkdir(parent);
    }
    match File::create(&path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to create {}: {:?}", path.display(), e),
    }
}

fn create_file_with_content(path: &Path, content: &String) -> File {
    let display = path.display();
    let mut f = create_file(&path);
    match f.write_all(content.as_bytes()) {
        Ok(_) => info!("Successfully wrote to {}", display),
        Err(e) => panic!("Failed to write to {}: {:?}", display, e),
    }
    f
}

fn join_path<'a>(path: &'a PathBuf, sub: &'a str) -> Result<String, Box<dyn std::error::Error>> {
    path.join(sub).as_path().to_str()
        .map(|str| str.to_owned())
        .ok_or(format!("Invalid file path `{}`", path.display()).into())
}

// RUST_LOG=info ./xcpp.exe new hello_cpp --path=E:/Environment/mingw64_14_2_0/bin --std=c++17
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Cli::from_args();
    match args.cmd {
        Cmd::Clear {  } => {
            let appdata = std::env::var("APPDATA")
                .expect("APPDATA environment variable not found")
                + "\\xcpp\\config\\config.toml";
            info!("Looking for `{appdata}`");
            std::fs::remove_file(Path::new(&appdata))
                .with_context(|| format!("Failed to delete file `config.toml`"))?;
        },
        Cmd::Store { std, path } => {
            let path = path.to_str()
                .with_context(|| format!("Invalid file path `{}`", path.display()))?
                .to_owned()
                .replace("\\", "/");

            let cfg = MyConfig { std, mingw64_path: path };
            info!("Saving {:?} to `%appdata%\\xcpp\\config\\config.toml`", cfg);
            confy::store("xcpp", "config", cfg)?;
        }
        Cmd::New { name, std, path } => {
            let cfg: MyConfig = confy::load("xcpp", "config")?;
            let std: String = match (std, cfg.std) {
                (s1, _) if !matches!(s1.as_str(), "cfg") => s1, // 如果没有指定用配置文件内容，说明是c++??
                (_, s2) => {
                    warn!("Missing argumet: `--std`, using `--std={s2}` in `config.toml`");
                    if cpp_standards().contains(&s2.as_str()) {
                        if matches!(s2.as_str(), "cfg") {
                            panic!("`std=cfg` cannot be used in `config.toml`");
                        }
                        s2
                    } else {
                        Cli::clap().print_help().unwrap();
                        panic!("Invalid argument in config.toml: `std={s2}`");
                    }
                }
            };
            let path = match (path, cfg.mingw64_path) {
                (s1, _) if !s1.is_empty() => s1, // 如果s1非空，说明是经过验证的命令参数
                (_, s2) => {
                    if s2.is_empty() {
                        Cli::clap().print_help().unwrap();
                        panic!("Missing argumet: `--path`, try to pass it or specify it in `config.toml`");
                    }
                    warn!("Missing argumet: `--path`, using `--path={s2}` in `config.toml`");
                    s2
                }
            };
            let path = PathBuf::from_str(path.as_str())
                .with_context(|| format!("Invalid file path `{}`", path))?;

            // 用户指定的mingw64路径不存在
            if !path.exists() {
                error!("{} doesn't exist, setup of mingw64 is required.\n", path.display());
            }
            
            let name: &str = name.clone().leak();
            let work_path = Path::new(name);
            // 要创建的工作路径已经存在，则终止
            if work_path.exists() {
                error!("Destination `{}` already exists", work_path.display());
                exit(-1);
            }
            // 创建项目工作目录
            mkdir(work_path);
            std::env::set_current_dir(work_path)
                .with_context(|| format!("Failed to switch directory"))?;

            let current_dir = std::env::current_dir()
                .with_context(|| format!("Failed to get current directory"))?;
            let current_dir = current_dir
                .to_str()
                .with_context(|| format!("Invalid file path `{}`", current_dir.display()))?
                .to_owned()
                .replace("\\", "/");

            info!("current directory: {current_dir}");
            
            let gpp_path = join_path(&path, "g++.exe")?.replace("\\", "/");
            let gdb_path = join_path(&path, "gdb.exe")?.replace("\\", "/");
            
            let mut table = HashMap::new();
            
            table.insert("project", name);
            table.insert("current_dir", current_dir.as_str());
            table.insert("stdc++", std.as_str());
            info!("using std={std}");
            table.insert("g++", gpp_path.as_str());
            info!("g++ path: `{gpp_path}`");
            table.insert("gdb", gdb_path.as_str());
            info!("gdb path: `{gdb_path}`");

            //exit(-1);
            mkdir(Path::new("target"));

            for file in FILES {
                let template = Template::new(&file.content);
                let content = template.fill_with_hashmap(&table);
                let path = Path::new(file.path);
                create_file_with_content(path, &content);
            }
            create_file(Path::new("data.in"));
            create_file(Path::new("data.out"));
            create_file_with_content(
                Path::new(".gitignore"), 
                &".vscode/\ntarget/\n".to_owned()
            );
            create_file_with_content(
                Path::new("main.cpp"), 
        &
r#"#include <iostream>
using namespace std;

int main(){
    cout << "Hello, World" << '\n';
    return 0;
}
"#.to_owned());

            let output = Command::new("git")
                .arg("init")
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        info!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        error!(
                            "Failed to evaluate: `git init`\nstderr: {}", 
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
                Err(e) => {
                    error!("Failed to evaluate: `git init`: {e}");
                }
            }
        },
    }
    
    // let stdout = io::stdout(); // get the global stdout entity
    // let mut handle = stdout.lock(); // acquire a lock on it
    // writeln!(handle, "foo: {}", 42)?; // add `?` if you care about errors here
    Ok(())
}

struct ConfigFile {
    path: &'static str,
    content: &'static str,
}

const FILES: [ConfigFile; 5] = [
    ConfigFile { path: ".vscode/c_cpp_properties.json", content: 
r#"{
    "configurations": [
        {
            "name": "Win32",
            "includePath": [
                "${workspaceFolder}/**"
            ],
            "defines": [
                "_DEBUG",
                "UNICODE",
                "_UNICODE"
            ],
            "cStandard": "c17",
            "cppStandard": "{{stdc++}}",
            "intelliSenseMode": "linux-gcc-x64",
            "compilerPath": "{{g++}}",
            "compilerArgs": [
                "/Zc:__cplusplus"
            ]
        }
    ],
    "version": 4
}"# },
    ConfigFile { path: ".vscode/launch.json", content: 
r#"{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "make: g++.exe Testing",
            "type": "cppdbg",
            "request": "launch",
            "program": "${workspaceFolder}/target/${workspaceFolderBasename}.exe",
            "args": ["<", "data.in", ">", "data.out"],
            "stopAtEntry": false,
            "cwd": "${fileDirname}",
            "environment": [],
            "externalConsole": false,
            "MIMode": "gdb",
            "miDebuggerPath": "{{gdb}}",
            "setupCommands": [
                {
                    "description": "为 gdb 启用整齐打印",
                    "text": "-enable-pretty-printing",
                    "ignoreFailures": true
                },
                {
                    "description": "将反汇编风格设置为 Intel",
                    "text": "-gdb-set disassembly-flavor intel",
                    "ignoreFailures": true
                }
            ],
            "preLaunchTask": "make"
        },
        {
            "name": "C/C++: g++.exe Testing",
            "type": "cppdbg",
            "request": "launch",
            "program": "${workspaceFolder}/target/${workspaceFolderBasename}.exe",
            "args": ["<", "data.in", ">", "data.out"],
            "stopAtEntry": false,
            "cwd": "${fileDirname}",
            "environment": [],
            "externalConsole": false,
            "MIMode": "gdb",
            "miDebuggerPath": "{{gdb}}",
            "setupCommands": [
                {
                    "description": "为 gdb 启用整齐打印",
                    "text": "-enable-pretty-printing",
                    "ignoreFailures": true
                },
                {
                    "description": "将反汇编风格设置为 Intel",
                    "text": "-gdb-set disassembly-flavor intel",
                    "ignoreFailures": true
                }
            ],
            "preLaunchTask": "C++: -O0"
        },
        {
            "name": "C/C++: g++.exe O1",
            "type": "cppdbg",
            "request": "launch",
            "program": "${workspaceFolder}/target/${workspaceFolderBasename}.exe",
            "args": ["<", "data.in", ">", "data.out"],
            "stopAtEntry": false,
            "cwd": "${fileDirname}",
            "environment": [],
            "externalConsole": false,
            "MIMode": "gdb",
            "miDebuggerPath": "{{gdb}}",
            "setupCommands": [
                {
                    "description": "为 gdb 启用整齐打印",
                    "text": "-enable-pretty-printing",
                    "ignoreFailures": true
                },
                {
                    "description": "将反汇编风格设置为 Intel",
                    "text": "-gdb-set disassembly-flavor intel",
                    "ignoreFailures": true
                }
            ],
            "preLaunchTask": "C/C++: -O1"
        },
        {
            "name": "C/C++: g++.exe O2",
            "type": "cppdbg",
            "request": "launch",
            "program": "${workspaceFolder}/target/${workspaceFolderBasename}.exe",
            "args": ["<", "data.in", ">", "data.out"],
            "stopAtEntry": false,
            "cwd": "${fileDirname}",
            "environment": [],
            "externalConsole": false,
            "MIMode": "gdb",
            "miDebuggerPath": "{{gdb}}",
            "setupCommands": [
                {
                    "description": "为 gdb 启用整齐打印",
                    "text": "-enable-pretty-printing",
                    "ignoreFailures": true
                },
                {
                    "description": "将反汇编风格设置为 Intel",
                    "text": "-gdb-set disassembly-flavor intel",
                    "ignoreFailures": true
                }
            ],
            "preLaunchTask": "C/C++: -O2"
        },
        {
            "name": "C/C++: g++.exe O3",
            "type": "cppdbg",
            "request": "launch",
            "program": "${workspaceFolder}/target/${workspaceFolderBasename}.exe",
            "args": ["<", "data.in", ">", "data.out"],
            "stopAtEntry": false,
            "cwd": "${fileDirname}",
            "environment": [],
            "externalConsole": false,
            "MIMode": "gdb",
            "miDebuggerPath": "{{gdb}}",
            "setupCommands": [
                {
                    "description": "为 gdb 启用整齐打印",
                    "text": "-enable-pretty-printing",
                    "ignoreFailures": true
                },
                {
                    "description": "将反汇编风格设置为 Intel",
                    "text": "-gdb-set disassembly-flavor intel",
                    "ignoreFailures": true
                }
            ],
            "preLaunchTask": "C/C++: -O3"
        },
        {
            "name": "外部发行版测试",
            "type": "cppdbg",
            "request": "launch",
            "program": "${fileDirname}/${fileBasenameNoExtension}.exe",
            "args": ["<", "data.in", ">", "data.out"],
            "stopAtEntry": false,
            "cwd": "${fileDirname}",
            "environment": [],
            "externalConsole": true,
            "MIMode": "gdb",
            "miDebuggerPath": "{{gdb}}",
            "setupCommands": [
                {
                    "description": "为 gdb 启用整齐打印",
                    "text": "-enable-pretty-printing",
                    "ignoreFailures": true
                },
                {
                    "description": "将反汇编风格设置为 Intel",
                    "text": "-gdb-set disassembly-flavor intel",
                    "ignoreFailures": true
                }
            ],
            "preLaunchTask": "Release"
        },
    ]
}
"# },
    ConfigFile { path: ".vscode/settings.json", content: 
r#"{
    "files.associations": {
        "iostream": "cpp",
        "*.tcc": "cpp",
        "optional": "cpp",
        "future": "cpp",
        "cmath": "cpp",
        "ostream": "cpp",
        "complex": "cpp",
        "cstdarg": "cpp",
        "cstdint": "cpp",
        "cstdio": "cpp",
        "cstdlib": "cpp",
        "type_traits": "cpp",
        "limits": "cpp",
        "typeinfo": "cpp",
        "bitset": "cpp",
        "algorithm": "cpp",
        "array": "cpp",
        "atomic": "cpp",
        "cctype": "cpp",
        "cfenv": "cpp",
        "charconv": "cpp",
        "chrono": "cpp",
        "cinttypes": "cpp",
        "clocale": "cpp",
        "codecvt": "cpp",
        "condition_variable": "cpp",
        "csetjmp": "cpp",
        "csignal": "cpp",
        "cstddef": "cpp",
        "cstring": "cpp",
        "ctime": "cpp",
        "cuchar": "cpp",
        "cwchar": "cpp",
        "cwctype": "cpp",
        "deque": "cpp",
        "forward_list": "cpp",
        "list": "cpp",
        "unordered_map": "cpp",
        "unordered_set": "cpp",
        "vector": "cpp",
        "exception": "cpp",
        "functional": "cpp",
        "iterator": "cpp",
        "map": "cpp",
        "memory": "cpp",
        "memory_resource": "cpp",
        "numeric": "cpp",
        "random": "cpp",
        "ratio": "cpp",
        "regex": "cpp",
        "set": "cpp",
        "string": "cpp",
        "string_view": "cpp",
        "system_error": "cpp",
        "tuple": "cpp",
        "utility": "cpp",
        "fstream": "cpp",
        "initializer_list": "cpp",
        "iomanip": "cpp",
        "iosfwd": "cpp",
        "istream": "cpp",
        "mutex": "cpp",
        "new": "cpp",
        "scoped_allocator": "cpp",
        "shared_mutex": "cpp",
        "sstream": "cpp",
        "stdexcept": "cpp",
        "streambuf": "cpp",
        "thread": "cpp",
        "typeindex": "cpp",
        "valarray": "cpp",
        "cassert": "cpp",
        "ccomplex": "cpp",
        "cerrno": "cpp",
        "cfloat": "cpp",
        "ciso646": "cpp",
        "climits": "cpp",
        "cstdalign": "cpp",
        "cstdbool": "cpp",
        "ctgmath": "cpp",
        "filesystem": "cpp",
        "ios": "cpp",
        "locale": "cpp",
        "queue": "cpp",
        "stack": "cpp",
        "any": "cpp",
        "barrier": "cpp",
        "bit": "cpp",
        "compare": "cpp",
        "concepts": "cpp",
        "coroutine": "cpp",
        "expected": "cpp",
        "source_location": "cpp",
        "latch": "cpp",
        "numbers": "cpp",
        "ranges": "cpp",
        "semaphore": "cpp",
        "span": "cpp",
        "spanstream": "cpp",
        "stacktrace": "cpp",
        "stop_token": "cpp",
        "syncstream": "cpp",
        "variant": "cpp",
        "nfafragmentstack.h": "c",
        "posttonfa.h": "c",
        "regexptopost.h": "c",
        "format": "cpp",
        "generator": "cpp",
        "print": "cpp",
        "stdfloat": "cpp",
        "text_encoding": "cpp"
    },
    "cmake.configureOnOpen": false,
    "C_Cpp.errorSquiggles": "enabled",
    "editor.formatOnPaste": false,
    "editor.formatOnSaveMode": "modifications"
}"# },
    ConfigFile { path: ".vscode/tasks.json", content: 
r#"{
    "version": "2.0.0",
    "tasks": [
        {
            "type": "cppbuild",
            "label": "make",
            "command": "make",
            "args": [],
            "options": {
                "cwd": "${fileDirname}"
            },
            "problemMatcher": [
                "$gcc"
            ],
            "group": "build",
            "detail": "wow, cmake"
        },
        {
            "type": "cppbuild",
            "label": "C++: -O0",
            "command": "{{g++}}",
            "args": [
                "-fdiagnostics-color=always",
                "-std={{stdc++}}",
                "-g",
                "-Wall",
                "-DLOCAL",
                "${fileDirname}/*.cpp",
                // "${fileDirname}/*.c",
                "-o",
                "${workspaceFolder}/target/${workspaceFolderBasename}.exe"
            ],
            "options": {
                "cwd": "${fileDirname}"
            },
            "problemMatcher": [
                "$gcc"
            ],
            "group": "build",
            "detail": "g++ -std={{stdc++}} -g -O0"
        },
        {
            "type": "cppbuild",
            "label": "C/C++: -O1",
            "command": "{{g++}}",
            "args": [
                "-fdiagnostics-color=always",
                "-std={{stdc++}}",
                "-g",
                "-DLOCAL",
                "-Wall",
                "-O1",
                "${fileDirname}/*.cpp",
                "-o",
                "${workspaceFolder}/target/${workspaceFolderBasename}.exe"
            ],
            "options": {
                "cwd": "${fileDirname}"
            },
            "problemMatcher": [
                "$gcc"
            ],
            "group": "build",
            "detail": "g++ -std={{stdc++}} -g -O1"
        },
        {
            "type": "cppbuild",
            "label": "C/C++: -O2",
            "command": "{{g++}}",
            "args": [
                "-fdiagnostics-color=always",
                "-std={{stdc++}}",
                "-g",
                "-DLOCAL",
                "-Wall",
                "-O2",
                "${fileDirname}/*.cpp",
                "-o",
                "${workspaceFolder}/target/${workspaceFolderBasename}.exe"
            ],
            "options": {
                "cwd": "${fileDirname}"
            },
            "problemMatcher": [
                "$gcc"
            ],
            "group": "build",
            "detail": "g++ -std={{stdc++}} -g -O2"
        },
        {
            "type": "cppbuild",
            "label": "C/C++: -O3",
            "command": "{{g++}}",
            "args": [
                "-fdiagnostics-color=always",
                "-std={{stdc++}}",
                "-g",
                "-DLOCAL",
                "-Wall",
                "-O3",
                //"-march=native", 生成的binary将与本地机器相关，不建议使用
                // "-Wl,--stack=536870912", 扩栈
                // "${file}",
                "${fileDirname}/*.cpp",
                // "${fileDirname}/*.c",
                "-o",
                "${workspaceFolder}/target/${workspaceFolderBasename}.exe"
            ],
            "options": {
                "cwd": "${fileDirname}"
            },
            "problemMatcher": [
                "$gcc"
            ],
            "group": "build",
            "detail": "g++ -std={{stdc++}} -g -O3"
        },
        {
            "type": "cppbuild",
            "label": "Release",
            "command": "{{g++}}",
            "args": [
                "-fdiagnostics-color=always",
                "-std={{stdc++}}",
                "-static",
                // "-g",
                "-Wall",
                "-O3",
                // "${file}",
                "${fileDirname}/*.cpp",
                // "${fileDirname}/*.c",
                "-o",
                "${workspaceFolder}/target/${workspaceFolderBasename}.exe"
            ],
            "options": {
                "cwd": "${fileDirname}"
            },
            "problemMatcher": [
                "$gcc"
            ],
            "group": "build",
            "detail": "g++ -std={{stdc++}} -static -O3"
        },
    ]
}
"# },
    ConfigFile { path: "makefile", content: 
r#"TARPATH = {{current_dir}}/target/
CXX = {{g++}}
EXEC = {{project}}
SRC = $(wildcard *.cpp)
OBJ = $(patsubst %.cpp, $(TARPATH)%.o, $(SRC))

CFLAGS = -c -fdiagnostics-color=always -std={{stdc++}} -g -O1 -Wall -DLOCAL

$(TARPATH)$(EXEC): $(OBJ)
	$(CXX) -o $@ $^

$(TARPATH)%.o: %.cpp
	$(CXX) $(CFLAGS) $< -o $@


.PHONY: clean
TARPATH2 = $(subst /,\,$(TARPATH))
clean:
	del $(TARPATH2)*.o $(TARPATH2)$(EXEC).exe

"# },
];