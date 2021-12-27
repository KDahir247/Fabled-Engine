# shipyard_app

`shipyard_app` aims to provide a standardized "Plugin" interface on top of the [`shipyard` ECS crate](https://github.com/leudz/shipyard).

This allows for codebases to more easily divide up many systems and workloads without having to declare all systems in one big workload builder in the root of an application.

Example [from test/tree.rs](https://github.com/storyai/shipyard_app/blob/master/src/test/tree.rs)

```rust
use shipyard_app::{AppBuilder, Plugin};
...

/// Registers
#[derive(Default)]
pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.update_pack::<ChildOf>("update in response to ChildOf changes");
        app.add_system(indexing::tree_indexing);
    }
}
```

## Usage

At the moment, shipyard_app is based off the master branch of https://github.com/leudz/shipyard, so until the updates to WorkloadBuilder are published, you must be on the master branch of shipyard.

The initial interface takes a lot of inspiration from [bevy_app]. Thanks @cart!

[bevy_app]: https://github.com/bevyengine/bevy/tree/b925e22949ee1ca990dfc6a678d8e4636cae5271/crates/bevy_app

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
