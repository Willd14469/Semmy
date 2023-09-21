# Semmy
A small utility to automatically alter a semantic version in a local file to be used however you see fit. Useful when you do not want to depend solely on a CI/CD pipeline tool but want to automatically track builds.

# Usage

```bash
$ semmy
```

```
Manage a local version file that follows a variation of the Semantic Version spec

Usage: semmy [OPTIONS] <COMMAND>

Commands:
  major    Increment the major version and reset minor, patch and build
  minor    Increment the Minor version and reset patch and build
  patch    Increment the Patch version and reset build
  build    Increment the Build version, does not reset the alpha/beta tags
  alpha    Set the alpha flag
  beta     Set the beta flag
  release  Remove the alpha/beta flags
  init     Initialize a default version file starting at 0.1.0
  get      Print the version in the current working directory - returns non-zero error code if not initialized
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose   Include verbose output from the process
  -c, --coloured  Allow coloured output
  -h, --help      Print help
  -V, --version   Print version

```

## Init a version if a file folder

```bash
$ semmy init
```

Creates a local `.sem_ver` file that will contain `0.1.0` as the initial version.

## Bumping the version

```bash
$ semmy <major|minor|patch|build>
```

Will bump the version in the file by the corresponding amount

## Pre-release flags

```bash
$ semmy <alpha|beta>
```

Will add the corresponding pre-release flags to the version

```bash
$ semmy release
```

Will clear the pre-release flag