import 'package:animated_splash_screen/animated_splash_screen.dart';
import 'package:ballot/views/home_screen.dart';
import 'package:flutter/material.dart';
import 'package:page_transition/page_transition.dart';

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Ballot',
      home: AnimatedSplashScreen(
        splashTransition: SplashTransition.sizeTransition,
        splash: Image.asset("assets/ballot.png"),
        backgroundColor: Color.fromARGB(255, 255, 255, 255),

        // duration: 1000,
        nextScreen: const HomeScreen(),
        pageTransitionType: PageTransitionType.fade,
      ),
    );
  }
}
