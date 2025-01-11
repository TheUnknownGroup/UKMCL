import 'package:flutter/material.dart';

class Text1 extends StatelessWidget {
  const Text1({super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      // height: 16.5,
      // width: 425.6,
      child: Align(
        alignment: Alignment.centerLeft,
        child: Text(
          "We hope you enjoy our launcher!",
          style: TextStyle(
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Color.fromRGBO(223, 231, 235, 1)
          ),
        ),
      ),
    );
  }
}