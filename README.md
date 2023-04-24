# arcana_b

## Overview

TODO

## Structure

Leverages [workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html).

| Workspace | Purpose                     |
| --------- | --------------------------- |
| `bevydev` | Bevy development playground |
| `limn`    | WGSL shader development     |

### `bevydev`

Bevy development playground, leveraging the [hot reloading technique described here](https://robert.kra.hn/posts/hot-reloading-rust/).

The hot reloading has some *significant* downsides, discussed in that workspace's README.
