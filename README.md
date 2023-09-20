# Nael

A simplistic CLI-based Dalamud Version Manager inspired by [nvm](https://github.com/nvm-sh/nvm) that allows you to easy install, update and switch between Dalamud versions.

## Installation

### From a release

Go to the [releases page](https://github.com/Blooym/Nael/releases) and either use the script for your platform to automatically download a pre-built binary and add it to your `PATH` or manually download the binary for your platform and set it up yourself.

### Using "cargo install"

To build & install Nael using Cargo, run the following:

```
cargo install nael
```

## Usage

Run `nael help` to see a list of commands and their usage.

## Integration with C# Projects

To better integrate Nael with Dalamud C# projects, such as plugins, you should set the `DALAMUD_HOME` environment variable to the path of the "current" folder Nael has created for you. This will allow you to use the `DALAMUD_HOME` environment variable in your project to access the Dalamud installation and switch between versions without having to change your `.csproj` file. 

You can find the path to the "current" folder by running `nael current` in your terminal after installing any version of Dalamud.

## License

This project is licenced under the AGPL-3.0 license. See [LICENCE](LICENSC) for more information.


