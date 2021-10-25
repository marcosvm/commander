To build a statically linked executable on Mac OS, with Homebrew installed

```
brew install FiloSottile/musl-cross/musl-cross
```

then

```
CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="x86_64-linux-musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl
```

gives

```→ file target/x86_64-unknown-linux-musl/release/commander
target/x86_64-unknown-linux-musl/release/commander: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, with debug_info, not stripped
```

# Experimental

```
bazel build commander
```

and find executable at: bazel-bin/commander
