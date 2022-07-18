import 'package:anim_search_bar/anim_search_bar.dart';
import 'package:ballot/models/model_data.dart';
import 'package:flutter/material.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  MyHomePageState createState() => MyHomePageState();
}

class MyHomePageState extends State<HomeScreen> {
  TextEditingController textController = TextEditingController();

  List<Candidate> candidate = candidates;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        centerTitle: true,
        title: Image.asset(
          "assets/ballot_t.png",
          fit: BoxFit.cover,
          height: 50,
        ),
        backgroundColor: Colors.white,
      ),
      body: Container(
        color: Color.fromARGB(255, 231, 231, 231),
        width: double.infinity,
        padding: const EdgeInsets.symmetric(horizontal: 20),
        child: Column(
          children: [
            AnimSearchBar(
              width: 400,
              textController: textController,
              onSuffixTap: () {
                setState(() {
                  textController.clear();
                });
              },
              color: Color.fromARGB(255, 255, 255, 255),
              // helpText: "Search",
              autoFocus: true,
              closeSearchOnSuffixTap: true,
              animationDurationInMilli: 1000,
              rtl: true,
            ),
            Expanded(
                child: ListView.builder(
                    itemCount: candidate.length,
                    itemBuilder: (context, index) {
                      final candidate = candidates[index];

                      return ListTile(
                        leading: Image.network(
                          candidate.urlImage,
                          fit: BoxFit.cover,
                          width: 50,
                          height: 50,
                        ),
                        title: Text(candidate.name),
                      );
                    })),
          ],
        ),
      ),
    );
  }
}
