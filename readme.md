# Installation
- Make sure you have Rustup installed, if not install it like so: `winget install Rustlang.Rustup`
- Install Rust Nightly
`rustup install nightly`
- Set nightly as default in this project
`rustup override set nightly`
`rustup component add rust-src --toolchain nightly`
`rustup component add llvm-tools-preview --toolchain nightly`
`rustup component add rust-src`
- Install bootimage
`cargo install cargo-xbuild`
`cargo install bootimage`

- Install QEMU (virtual machine, after compiling it gets called directly)
Win32: https://qemu.weilnetz.de/w32/
Win64: https://qemu.weilnetz.de/w64/
- Add QEMU to the enviroment variables
First find the path, normally this is in `C:\Program Files\qemu`
In powershell you could run the command `[System.Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\qemu", [System.EnvironmentVariableTarget]::Machine)`
Or just open the enviroment variables and add `C:\Program Files\qemu` to `Path`