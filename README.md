# Nael

A simplistic CLI-based Dalamud Version Manager inspired by [nvm](https://github.com/nvm-sh/nvm) that allows you to easy install, update and switch between Dalamud versions.

## Installation

As of now, the only installation method available is manually compiling this repository either with `cargo install --git https://github.com/Blooym/Nael` or by cloning the repository and running `cargo build --release` in the root folder and then copying the binary to a folder in your `PATH` environment variable. 

In the future a more streamlined installation method will be available.

## Integration with C# Projects

To better integrate Nael with Dalamud C# projects, such as plugins, you should set the `DALAMUD_HOME` environment variable to the path of the "current" folder Nael has created for you. This will allow you to use the `DALAMUD_HOME` environment variable in your project to access the Dalamud installation and switch between versions without having to change your `.csproj` file. 

You can find the path to the "current" folder by running `nael current` in your terminal after installing any version of Dalamud.

## License

This project is licenced under the AGPL-3.0 license. See [LICENCE](LICENSC) for more information.


