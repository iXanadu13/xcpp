![English](README.md) | ![中文](README-zh.md)

# xcpp

在Windows中，使用一条指令创建配置好的VSCode C++项目。

生成的C++项目支持多文件，使用g++编译，gdb调试。项目中同时生成了makefile、git。

## Preparation

### step 1: 配置mingw64环境

下载：[https://github.com/niXman/mingw-builds-binaries/releases](https://github.com/niXman/mingw-builds-binaries/releases)

我选的是`x86_64-14.2.0-release-posix-seh-ucrt-rt_v12-rev0`，欲知不同版本的区别请自行google。

解压缩后，尽量放置在无中文、空格的路径中。我的mingw64路径为`E:/Environment/mingw64_14_2_0/bin`

最后，可以选择把mingw64路径(`E:/Environment/mingw64_14_2_0/bin`)添加到环境变量Path中。

### step 2: 配置make

其实mingw64里有自带make，位于`bin/mingw32-make.exe`。

为了方便，我把它单独复制出来放在`E:/Environment/mingw32-make`文件夹中，重命名为make.exe，并把`E:/Environment/mingw32-make`也添加进环境变量。（这样就可以在cmd中输入make来使用）

注意：生成的C++项目中配置的任务有make版本，如果没有完成这一步，也可以使用其他的任务。make会判断哪些文件未修改，并在下一次编译时跳过它们，从而提升编译速度。

## Setup

从[Github release](https://github.com/iXanadu13/xcpp/releases/latest)下载最新版xcpp，解压后获得xcpp.exe，将其路径添加到环境变量。

在cmd中运行以下指令（一个可能的`mingw64路径`示例：`E:/Environment/mingw64_14_2_0/bin`），默认配置将写入`%appdata%\xcpp\config\config.toml`：

```
xcpp store --std=c++20 --path mingw64路径
```

之后可以通过`xcpp new 项目名`直接生成C++项目。

**请不要在包含中文、空格的路径下使用，不要使用中文项目名，否则可能导致VSCode中无法正常调试。**

当然，你也可以在每次新建C++项目时，传入指定参数（`--std`、`--path`等），命令行传入的参数优先级高于配置文件。

```
xcpp new hello_cpp --std=c++20 --path mingw64路径
```

成功创建项目后，需要在VSCode中安装C/C++拓展插件：
- `C/C++`
- `C/C++ Extension Pack`
- `C/C++ Themes`

**如果提示找不到任务，重启VSCode即可。**

若指定`RUST_LOG=info`运行，能看到打印出的log。比如在`Git Bash`中运行：

```bash
RUST_LOG=info ./xcpp.exe new test1 --std=c++17 --path E:/Environment/mingw64_14_2_0/bin
```

你需要替换`./xcpp.exe`、`E:/Environment/mingw64_14_2_0/bin`为正确的路径。

使用示例（已经配置好mingw64、make、xcpp环境变量）：

![](example.gif)

## Usage

使用`xcpp.exe --help`或者`xcpp.exe <SUBCOMMAND> --help`即可显示帮助信息。

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
