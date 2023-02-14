# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.10.1](https://github.com/justinrubek/nutmeg/compare/0.10.0..0.10.1) - 2023-02-14
#### Continuous Integration
- add cocogitto to packages - ([792209a](https://github.com/justinrubek/nutmeg/commit/792209af12d382e5b3119ac26174ce485f89525d)) - [@justinrubek](https://github.com/justinrubek)
#### Miscellaneous Chores
- **(cog)** update changelog definition - ([9314913](https://github.com/justinrubek/nutmeg/commit/93149133070cdb23aef04eadf6c859061b961d20)) - [@justinrubek](https://github.com/justinrubek)

- - -

## 0.10.0 - 2023-02-14
#### Bug Fixes
- fix pre-commit rustfmt - (7a6cbad) - Justin Rubek
- fix wasm builds in nix - (3d868ed) - Justin Rubek
- rename client binary in flake app - (2307c2a) - Justin Rubek
- add indexmap to deps to fix petgraph error - (59865bf) - Justin Rubek
- use correct default app - (ef1cc0f) - Justin Rubek
#### Build system
- **(nix)** rework rust toolchain definition - (04fc966) - Justin Rubek
- **(nix)** remove package builds from checks - (f813c4d) - Justin Rubek
- **(nix)** unify rust toolchains - (ed011fd) - Justin Rubek
- **(nix)** rename deps-only build - (6a55ad6) - Justin Rubek
- **(nix)** configure checks - (5a995a3) - Justin Rubek
- **(nix)** remove flake-utils input - (6dccbc3) - Justin Rubek
- **(nix)** switch to crane and fenix for rust tooling - (71d9e6e) - Justin Rubek
- **(nix)** consolidate flake into multiple parts - (f32559a) - Justin Rubek
#### Continuous Integration
- remove github actions tests - (056f49e) - Justin Rubek
- run flake checks instead of cargo tests - (7b8ff3b) - Justin Rubek
#### Miscellaneous Chores
- **(bomper)** remove nix files from config - (1bdaebe) - Justin Rubek
- add x86 toolchain - (9570e89) - Justin Rubek
- clean up flake - (6266b18) - Justin Rubek
- remove version from workspace - (d60e9e7) - Justin Rubek
- fix clippy issues - (9841717) - Justin Rubek
#### Style
- enable rustfmt - (7fbd603) - Justin Rubek

- - -

## 0.9.0 - 2022-08-26
#### Features
- smoothed movement using velocity - (74f4e6d) - Justin Rubek

- - -

## 0.8.1 - 2022-08-25
#### Bug Fixes
- rename parallel feature from rapier2d - (89a6b9e) - Justin Rubek

- - -

## 0.8.0 - 2022-08-25
#### Build system
- enable features for rapier - (d379e41) - Justin Rubek
#### Features
- player now grows when eating - (016569a) - Justin Rubek
- use collision for eating food instead of shape cast - (1a49bb8) - Justin Rubek
#### Refactoring
- tweak player speed and torque - (63782ca) - Justin Rubek

- - -

## 0.7.0 - 2022-08-24
#### Features
- food is now destroyed when the player collides with it - (1adb10c) - Justin Rubek
#### Refactoring
- removed println - (ef5d9bf) - Justin Rubek
- extract logic into separate folders - (411e4ee) - Justin Rubek
- bundle physics into separate plugin - (6cbc92a) - Justin Rubek
- pull game logic into nutmeg_core - (5989801) - Justin Rubek

- - -

## 0.6.0 - 2022-08-23
#### Features
- Added food particles - (5dae087) - Justin Rubek
- camera follows player's ball - (a767e46) - Justin Rubek
#### Miscellaneous Chores
- add license - (9556d94) - Justin Rubek
#### Refactoring
- tweak player speed and torque - (671d8c8) - Justin Rubek

- - -

## 0.5.0 - 2022-08-23
#### Features
- implement rapier2d physics for ball - (a702456) - Justin Rubek

- - -

## 0.4.7 - 2022-08-22
#### Bug Fixes
- rename script import in index.html - (4faffcc) - Justin Rubek

- - -

## 0.4.6 - 2022-08-22
#### Continuous Integration
- fixed name of package when copying in action - (286d6de) - Justin Rubek
- fixed name of cli devshell - (f370aad) - Justin Rubek

- - -

## 0.4.5 - 2022-08-22
#### Continuous Integration
- added separate CI devshell - (a3624c9) - Justin Rubek

- - -

## 0.4.4 - 2022-08-22
#### Continuous Integration
- fetch all commits when bumping - (f3ba7cf) - Justin Rubek

- - -

## 0.4.3 - 2022-08-22
#### Continuous Integration
- remove quotes around call using nix develop -c - (3bb19b6) - Justin Rubek

- - -

## 0.4.2 - 2022-08-22
#### Continuous Integration
- properly execute git command to determine version - (fffc52d) - Justin Rubek

- - -

## 0.4.1 - 2022-08-22
#### Continuous Integration
- remove dependency on job that didn't exist - (e6b70c4) - Justin Rubek

- - -

## 0.4.0 - 2022-08-22
#### Build system
- **(cargo)** remove profile in sub-crate - (d336819) - Justin Rubek
#### Continuous Integration
- re-remove clippy check - (d3cfe68) - Justin Rubek
- re-add clippy check - (9b8a6fc) - Justin Rubek
- Rework actions for ergonomics - (0ca1f2d) - Justin Rubek
#### Features
- make ball follow cursor - (7094d6e) - Justin Rubek

- - -

## 0.3.0 - 2022-08-22
#### Continuous Integration
- fixed naming of CD workflow - (039c06a) - Justin Rubek
- split ci and cd into separate workflows - (c895bf8) - Justin Rubek
- fixed package names and test commands - (fdb4551) - Justin Rubek
- added auto version bumping - (f5ef65a) - Justin Rubek
#### Documentation
- **(README)** added spacing - (1bce618) - Justin Rubek
- add README - (7125781) - Justin Rubek
#### Features
- added moving ball - (ec063f9) - Justin Rubek

- - -

## 0.2.1 - 2022-08-22
#### Build system
- Configure project with cog and bomper - (99da9bf) - Justin Rubek
#### Miscellaneous Chores
- **(gitignore)** update .gitignore - (bb355cc) - Justin Rubek
- **(version)** 0.2.0 - (fde680d) - Justin Rubek

- - -

## 0.2.0 - 2022-08-22
#### Miscellaneous Chores
- **(gitignore)** update .gitignore - (d76b033) - Justin Rubek

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).