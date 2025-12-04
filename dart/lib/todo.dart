import 'dart:io';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:path/path.dart' as path;
import 'bindings/todo_bridge.dart/frb_generated.dart';

/// Initialize the Rust library
Future<void> initRustLib() async {
  // Determine the library path based on the platform
  final libraryPath = _getLibraryPath();
  final externalLibrary = ExternalLibrary.open(libraryPath);
  
  await RustLib.init(externalLibrary: externalLibrary);
}

String _getLibraryPath() {
  // Adjust paths based on your build setup
  // The target directory is in the project root, one level up from dart/
  final projectRoot = path.dirname(Directory.current.path);
  if (Platform.isLinux) {
    return path.join(projectRoot, 'target', 'debug', 'libtodo.so');
  } else if (Platform.isMacOS) {
    return path.join(projectRoot, 'target', 'debug', 'libtodo.dylib');
  } else if (Platform.isWindows) {
    return path.join(projectRoot, 'target', 'debug', 'todo.dll');
  } else {
    throw UnsupportedError('Platform not supported');
  }
}

