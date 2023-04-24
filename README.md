# arcana_b

## Overview

TODO

## Structure

Leverages [workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html).

| Workspace               | Purpose                                                        |
| ----------------------- | -------------------------------------------------------------- |
| `ammasment`             | Bevy development (finalised examples)                          |
| `amassment_dev`         | Bevy playground (main loop with hot reloading of systems)      |
| `amassment_dev_systems` | Library of systems for the Bevy playground, dynamically loaded |
| `limn`                  | WGSL shader development                                        |

### `amassment`

#### `amassment_dev` and `amassment_dev_systems`

Bevy development playground, leveraging the [hot reloading technique described here](https://robert.kra.hn/posts/hot-reloading-rust/).

The hot reloading has some *significant* downsides, discussed in that workspace's README.
