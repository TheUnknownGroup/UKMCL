import 'package:flutter/material.dart';
import 'dart:io';
import 'package:window_size/window_size.dart';

const appName = 'UKMCL';

void main() {
  runApp(const MyApp());
  
  WidgetsFlutterBinding.ensureInitialized();
  if (Platform.isWindows || Platform.isLinux || Platform.isMacOS) {
    setWindowTitle(appName);
    setWindowMinSize(Size(950, 550));
    setWindowMaxSize(Size(950, 550));
  }
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: const MyPage(),
    );
  }
}

class MyPage extends StatelessWidget {
  const MyPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        
      ),
      body: const Center(
        child: Text('Welcome!'),
      ),
    );
  }
}