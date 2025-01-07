import 'package:flutter/material.dart';
import 'dart:io';
import 'package:window_size/window_size.dart';

const appName = 'UKMCL';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  if (Platform.isWindows || Platform.isLinux || Platform.isMacOS) {
    setWindowTitle(appName);
    setWindowMinSize(Size(950, 550));
    setWindowMaxSize(Size(950, 550));
  }
  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        body: Center(
          child: Text('Hello, world!'),
        ),
      ),
    );
  }
}

// class Row1 extends StatefulWidget {
//   const Row1({super.key});

//   @override
//   Widget build(BuildContext context) {
//     return Center (
      
//     )
//   }
// }
