# Conways Game of Life (Sandbox)

## Development

### Watch

```bash
cargo install cargo-watch
```

```bash
cargo watch
```

Without --release it is slower.

```bash
cargo watch -x 'run --release'
```

### Test

Without --release random board generation is very slow.

```bash
cargo watch -x "test --release -- --nocapture"

cargo watch -x "test --release -- benchmark_new_life_random --nocapture"
cargo watch -x "test --release -- benchmark_draw_image_data_url --nocapture"
cargo watch -x "test --release -- benchmark_life --nocapture"
```

## Build release

```bash
cargo build --release
```
