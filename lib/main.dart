import 'package:flutter/material.dart';

void main() {
  runApp(const MainApp());
}

const appName = 'UKMCL';
var new1 = '';

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      home: Scaffold(
        body: Center(
          child: Text('Welcome to $appName! We hope you enjoy our app.'),
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
