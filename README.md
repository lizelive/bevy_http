# http loader for bevy

adds the ability to load assets from http and https urls

working on merging into bevy

## usage

```rust ignore 
fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("remote://icon.png");
}
```