# Nael

A simplistic CLI-based Dalamud Version Manager inspired by [nvm](https://github.com/nvm-sh/nvm) that allows you to easy install, update and switch between Dalamud versions.

## Installation

### Pre-built releases

Go to the [releases page](https://github.com/Blooym/Nael/releases) and manually download a pre-built binary for your device add place it somewhere in your system `PATH`. 

### Using Cargo

To build & install Nael using Cargo, run the following:

```
cargo install nael
```

## Usage

Run `nael help` to see a list of commands and their usage.

## Integration with C# Projects

To better integrate Nael with Dalamud C# projects, such as plugins, you should set the `DALAMUD_HOME` environment variable to the path of the "current" folder Nael has created for you. You should then add the following to your `csproj` file to replace your current DalamudLibPath property:

```xml
<PropertyGroup>
  <DalamudLibPath Condition="$([MSBuild]::IsOSPlatform('Windows'))">$(appdata)\XIVLauncher\addon\Hooks\dev\</DalamudLibPath>
  <DalamudLibPath Condition="$([MSBuild]::IsOSPlatform('Linux'))">$(HOME)/.xlcore/dalamud/Hooks/dev/</DalamudLibPath>
  <DalamudLibPath Condition="$([MSBuild]::IsOSPlatform('OSX'))">$(HOME)/Library/Application Support/XIV on Mac/dalamud/Hooks/dev/</DalamudLibPath>
  <DalamudLibPath Condition="$(DALAMUD_HOME) != ''">$(DALAMUD_HOME)/</DalamudLibPath>
</PropertyGroup>
```

This will enable you to use the `DALAMUD_HOME` environment variable to override the default DalamudLibPath and use Nael to manage your Dalamud version instead whilst still maintaining compatibility for other users who may not be using Nael.

You can find the path to the "current" folder by running `nael current` in your terminal after installing & selecting any version of Dalamud.

## License

This project is licenced under the AGPL-3.0 license. See [LICENCE](LICENSC) for more information.


