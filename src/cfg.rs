pub(crate) struct ConfigFile {
    pub(crate) path: &'static str,
    pub(crate) content: &'static str,
}

pub(crate) const FILES: [ConfigFile; 5] = [
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