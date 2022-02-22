# Debugging Swift Toolbox 🧰

## Setup

Follow README.md

# Build the build-dist target

```
cargo make build-dist
```

## Debugging an intermittent crash on startup

# Linux
```
cd py39-dist
for i in {1..1000}; do 
    rust-gdb -ex='set confirm on' -ex run -ex=quit --args swift-console --log-console --file ../console_backend/tests/data/ins_updates.sbp
    echo ${i}
done
```

# macOS

```
cd py39-dist
for i in {1..1000}; do 
    rust-lldb -o run -o quit -- ./swift-console --log-console --file ../console_backend/tests/data/ins_updates.sbp
    echo ${i}
done
```

For your convenience a script is provided that will perform this looped
execution. You can find it at `./debug_intermittent_startup_crash.sh`
