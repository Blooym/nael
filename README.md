# Nael

> [!WARNING]  
> Nael is not yet considered stable and is subject to breaking changes. If you're using it in CI or scripts it you should pin to a specific version to avoid breakage. This includes both the `nael_core` crate and the `nael` binary.

A Dalamud version manager that makes working with different releases (branches) of Dalamud simple, inspired by [nvm](https://github.com/nvm-sh/nvm).

## Features
- Quick installs and updates from official release distribution source.
- Instant switching between multiple locally-installed branches.
- Able to check version information from a remote source to only update when out of date.
- Compliant with all major operating system storage standards.
- No configuration files, everything is completely filesystem based.
- All functionality implemented through a project-agnostic crate.

Check the [examples](#examples) section to see Nael in use. 

## Installing

<details>
<Summary>With Cargo</summary>

You can get Nael from Crates.io using Cargo, to do so simply run:

```
cargo install nael
```

</details>

<details>
<Summary>From a prebuilt binary</summary>

Pre-built binaries are currently available for `Windows x64`, `Linux x64`, `macOS Apple Silicon` and `macOS Intel`. Head to the [GitHub releases page](https://github.com/Blooym/Nael/releases) and download the right release asset for your machine add place it somewhere in your system `$PATH`.

</details>

<details>
<Summary>From a package manager</summary>

Nael is not available from any package manager at this time.

</details>

## Integrating with Dalamud C# Projects.

> [!NOTE]  
> Testing is required here. PRs are welcome to help improve this section!

To integrate Nael with Dalamud C# projects, such as plugins, do the following:

1. Install and set a branch as active by running `nael install <branch>` followed by `nael use <branch>`.

2. 
   - **If your solution works with Symlinks**: set the `$DALAMUD_HOME` environment variable to point to the symlink of the active branch. You can achieve this by setting the environment variable to point to the return value of `nael active --empty-if-none --format symlink-path` (note: the path will be empty if no branch is active).
   - **If your solution does not work with Symlinks** set the `$DALAMUD_HOME` environment variable to point to the absolute path of the active branch. You can achieve this by setting the environment variable to point to the return value of `nael active --empty-if-none --format real-path` (note: the path will be empty if no branch is active).

3. Add the following to your `.csproj` file, replacing any existing definitions of the `DalamudLibPath` property.
    ```xml
    <PropertyGroup>
      <DalamudLibPath Condition="$([MSBuild]::IsOSPlatform('Windows'))">$(appdata)\XIVLauncher\addon\Hooks\dev\</DalamudLibPath>
      <DalamudLibPath Condition="$([MSBuild]::IsOSPlatform('Linux'))">$(HOME)/.xlcore/dalamud/Hooks/dev/</DalamudLibPath>
      <DalamudLibPath Condition="$([MSBuild]::IsOSPlatform('OSX'))">$(HOME)/Library/Application Support/XIV on Mac/dalamud/Hooks/dev/</DalamudLibPath>
      <DalamudLibPath Condition="$(DALAMUD_HOME) != ''">$(DALAMUD_HOME)/</DalamudLibPath>
    </PropertyGroup>
    ```

This will enable you to use the `DALAMUD_HOME` environment variable to override the default DalamudLibPath and use Nael to manage your Dalamud version instead while still maintaining compatibility for other users who want to build your project.

## Examples

**Show all command line options:**
```
> nael help
A Dalamud version manager that makes working with different releases of Dalamud simple.

Usage: nael <COMMAND>

Commands:
  install  Install a Dalamud release from the specified branch
  update   Update a local branch to the latest version
  remove   Remove a branch from this system
  list     List all installed branches
  use      Switch the currently active branch
  active   Get information about the active branch
  info     Show information about the specified branch
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

**Installing a Dalamud release from the `latest` branch:**
```
> nael install latest
Successfully installed branch 'latest' with version '9.0.0.17'.
```

**Updating the `latest` branch**
```
> nael update latest
Branch is already up to date.
```

**Setting the active branch to `latest`:**
```
> nael use latest
Successfully set branch 'latest' as active.
```

**Listing all installed branches**
```
> nael list
Installed branches:
 - stg
 - v9
 * latest
```

**Getting information about the active version:**
```
> nael active
latest
> nael active --format symlink-path
/home/example/.config/nael/active
> nael active --format real-path
/home/example/.local/share/nael/dalamud-branches/latest
```

**Getting local information about the `latest` branch:**   
Optionally, see remote information instead with the `--remote` flag.
```
> nael info latest
Local version information for branch latest:
- Version: 9.0.0.17
- Git Sha: unknown
- Revision: unknown
- Key: N/A
- Supported GameVer: 2023.03.24.0000.0000
- Runtime version: 7.0.0
- Runtime required: true
```

## License

This project is dual-licensed under both the MIT License and the Apache License (Version 2.0). See [LICENSE-MIT](./LICENSE-MIT) and [LICENSE-APACHE](./LICENSE-APACHE) for more details.