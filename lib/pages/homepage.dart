import 'package:flutter/material.dart';
import 'package:ukmcl/components//text.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          image: DecorationImage(
            image: AssetImage("assets/background.png"), fit: BoxFit.cover,
          ),
        ),
        child: Text1(),
      ),
    );
  }
}
