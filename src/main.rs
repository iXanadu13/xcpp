use std::{collections::HashMap, fs::File, io::Write, path::{Path, PathBuf}, process::{exit, Command}, str::FromStr};

use anyhow::Context;
use structopt::StructOpt;
use log::{error, info, warn};
use text_placeholder::Template;
use serde_derive::{Serialize, Deserialize};

mod cfg;

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

// RUST_LOG=info ./xcpp.exe new hello_cpp --path E:/Environment/mingw64_14_2_0/bin --std=c++17
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
            let config: MyConfig = confy::load("xcpp", "config")?;
            let std: String = match (std, config.std) {
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
            let path = match (path, config.mingw64_path) {
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
                error!("{} doesn't exist, setup of mingw64 is required first.\nYou can download it at https://github.com/niXman/mingw-builds-binaries/releases",
                    path.display()
                );
                exit(-1);
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

            for file in cfg::FILES {
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
    
    Ok(())
}
