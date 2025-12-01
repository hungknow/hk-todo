import 'dart:io';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:path/path.dart' as path;
import 'bindings/todo_bridge.dart';

/// Initialize the Rust library
Future<void> initRustLib() async {
  // Determine the library path based on the platform
  final libraryPath = _getLibraryPath();
  final externalLibrary = ExternalLibrary.open(libraryPath);
  
  await RustLib.init(externalLibrary: externalLibrary);
}

String _getLibraryPath() {
  // Adjust paths based on your build setup
  // For development, you might point to the built library in target/
  if (Platform.isLinux) {
    return path.join(Directory.current.path, 'target', 'debug', 'libdart_todo.so');
  } else if (Platform.isMacOS) {
    return path.join(Directory.current.path, 'target', 'debug', 'libdart_todo.dylib');
  } else if (Platform.isWindows) {
    return path.join(Directory.current.path, 'target', 'debug', 'dart_todo.dll');
  } else {
    throw UnsupportedError('Platform not supported');
  }
}

