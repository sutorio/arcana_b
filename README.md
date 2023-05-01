# arcana_b

## Overview

TODO

## Structure

Leverages [workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html).

| Workspace               | Purpose                                                             |
| ----------------------- | ------------------------------------------------------------------- |
| `arcana_b` (root)       | Monorepo base, no app                                               |
| `system_evolution`      | System development for `arcana_b`                                   |
| `limn`                  | WGSL shader development                                             |

The `system_evolution` crate is a development playground for Bevy, used to test out ideas/techniques prior to comitting them.
