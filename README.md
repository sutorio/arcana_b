# arcana_b

## Overview

TODO

## Structure

Leverages [workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html).

| Workspace               | Purpose                                                             |
| ----------------------- | ------------------------------------------------------------------- |
| `arcana_b` (root)       | Bevy binary app, hot reloading behind feature flag for development  |
| `system_evolution`      | Library of systems for `arcana_b`, dynamically loaded               |
| `limn`                  | WGSL shader development                                             |

The root app can be used as a development playground for Bevy, used to test out ideas/techniques prior to comitting them.

## Usage

> **NOTE:** this uses bleeding-edge Bevy, expect issues. Where possible, set dependencies to their development branches.

> **NOTE:** the main loop and the code the main loop uses are split into separate workspaces. Due to this, dependencies
> that require config in the main loop must be duplicated across (for example, Rapier's bevy plugins). 

Core code is written in the `system_evolution` workspace. `system_evolution` is a library. The built library code is
dynamically linked to the main game loop defined in `src/main.rs`.

[`cargo-watch`]() is leveraged to automatically rebuild code on changes

Requires two terminals, one to build the systems library, one to run the main loop.

In the first terminal, at project root, run the following to watch the library code:

```sh
cargo watch -w system_evolution -x 'build -p system_evolution'
```

In the second terminal, run the following the run the actual app, roloading on changes:

```sh
cargo watch -i system_evolution -x 'run --features reload'
```

### Process

- For a given feature, develop primarily in `system_evolution`. Once complete, transfer across to the finalised set of examples.

### IMPORTANT CAVEATS

The hot reloading has some *significant* downsides.

[READ THIS CAREFULLY](https://robert.kra.hn/posts/hot-reloading-rust/)

Changes to structs/enums will **not** trigger changes. The memory layout for structs/enums/etc
**is fixed at compile time**. What this means is that changing them on-the-fly will result in
undefined behaviour.

Because of this, the reloading is of limited use without the data structures already being in place.
It is still useful enough for it to be valuable.

- TODO: ensure that it only runs when relevant code is changed: currently it reruns on everything, so if (eg) the .gitignore file is changed the code reruns.
- TODO: is it possible to remove reloading as a feature & still get LSP completion in the main function? Even if not, need to set flags to allow dead code.
- TODO: ensure this part of the overall project is private in terms of publishing; it must be committed, but will never be released. It is a playground, doesn't need to be versioned or anything.

