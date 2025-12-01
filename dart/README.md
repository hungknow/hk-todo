# Todo Dart Package

This package provides Dart bindings for the Todo Rust library using flutter_rust_bridge.

## Setup

1. Build the Rust library:
   ```bash
   cd ../crates/dart-todo
   cargo build --release
   ```

2. Generate Dart bindings:
   ```bash
   cd ../crates/dart-todo
   flutter_rust_bridge_codegen generate
   ```

3. Install Dart dependencies:
   ```bash
   cd ../../dart
   dart pub get
   ```

## Usage

See `docs/todo_dart.md` for detailed usage examples.

## Note

The `lib/bindings/todo_bridge.dart` file is auto-generated. Do not edit it manually.

