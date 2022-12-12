# Development
To build in development just run `cargo build`.

# Building for Release
To build optimized for release run:
```sh
cargo build --release --no-default-features
```

This is thanks to the trick from [this stackoverflow post](https://stackoverflow.com/questions/69428144/can-i-activate-a-dependencys-feature-only-for-debug-profile).