# Nael

A simplistic CLI-based Dalamud Version Manager inspired by [nvm](https://github.com/nvm-sh/nvm) that allows you to easy install, update and switch between Dalamud versions.

## Installation

As of now, the only installation methods available involve manually compiling this repository. In the future a more streamlined installation process will be available.

### Cargo Install

To install Nael using Cargo, run the following:

```
cargo install --git https://github.com/Blooym/Nael`
```

### Cargo Build

Run the following to build the project and then move the binary to your `PATH`:

```
git clone https://github.com/Blooym/Nael.git && cd Nael && cargo build --release
```

## Integration with C# Projects

To better integrate Nael with Dalamud C# projects, such as plugins, you should set the `DALAMUD_HOME` environment variable to the path of the "current" folder Nael has created for you. This will allow you to use the `DALAMUD_HOME` environment variable in your project to access the Dalamud installation and switch between versions without having to change your `.csproj` file. 

You can find the path to the "current" folder by running `nael current` in your terminal after installing any version of Dalamud.

## License

This project is licenced under the AGPL-3.0 license. See [LICENCE](LICENSC) for more information.


