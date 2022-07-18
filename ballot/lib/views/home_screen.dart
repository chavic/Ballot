import 'package:ballot/models/model_data.dart';
import 'package:ballot/views/registration.dart';
import 'package:flutter/material.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  MyHomePageState createState() => MyHomePageState();
}

class MyHomePageState extends State<HomeScreen> {
  final controller = TextEditingController();

  List<Election> elections = allElections;

  void searchCandidate(String query) {
    final results = allElections.where((candidate) {
      final candidateName = candidate.name.toLowerCase();
      final input = query.toLowerCase();

      return candidateName.contains(input);
    }).toList();
    setState(() => elections = results);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        centerTitle: true,
        title: Image.asset(
          "assets/ballot_t.png",
          fit: BoxFit.cover,
          height: 60,
        ),
        backgroundColor: Colors.white,
      ),
      body: Column(
        children: <Widget>[
          Container(
            margin: const EdgeInsets.fromLTRB(14, 14, 14, 14),
            child: TextField(
              controller: controller,
              decoration: InputDecoration(
                prefixIcon: const Icon(Icons.search_outlined),
                hintText: 'Search',
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(50),
                  borderSide: const BorderSide(color: Colors.green),
                ),
              ),
              onChanged: searchCandidate,
            ),
          ),
          Expanded(
            child: ListView.builder(
              itemCount: elections.length,
              itemBuilder: (context, index) {
                final election = elections[index];
                return ListTile(
                  leading: Image.network(
                    election.urlImage,
                    fit: BoxFit.cover,
                    width: 50,
                    height: 50,
                  ),
                  title: Text(election.name),
                  onTap: () => Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: ((context) => Registration(election: election)),
                    ),
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}
