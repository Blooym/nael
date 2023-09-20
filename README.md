# Nael

A simplistic CLI-based Dalamud Version Manager inspired by [nvm](https://github.com/nvm-sh/nvm) that allows you to easy install, update and switch between Dalamud versions.

## Installation

### Using a shell script

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Blooym/Nael/releases/download/v0.1.1/nael-installer.sh | sh
```

### Using a PowerShell script

```sh
irm https://github.com/Blooym/Nael/releases/download/v0.1.1/nael-installer.ps1 | iex
```

### Using "cargo install"

To install Nael using Cargo, run the following:

```
cargo install nael
```

### Using "cargo build"

Run the following to build the project and then move the binary to your `PATH`:

```
git clone https://github.com/Blooym/Nael.git && cd Nael && cargo build --release
```

### From a release

Download the latest release from the [releases page](https://github.com/Blooym/Nael/releases) and move the binary for your platform to your `PATH`.

## Usage

Run `nael help` to see a list of commands and their usage.

## Integration with C# Projects

To better integrate Nael with Dalamud C# projects, such as plugins, you should set the `DALAMUD_HOME` environment variable to the path of the "current" folder Nael has created for you. This will allow you to use the `DALAMUD_HOME` environment variable in your project to access the Dalamud installation and switch between versions without having to change your `.csproj` file. 

You can find the path to the "current" folder by running `nael current` in your terminal after installing any version of Dalamud.

## License

This project is licenced under the AGPL-3.0 license. See [LICENCE](LICENSC) for more information.


