# The Design of Ballot
## Abstract
We introduce an electronic voting system that can be generalized to different voting scenarios such as company or government, and that tries to address many of the flaws of modern electronic voting systems while still been minimal enough to be extended for a number of general voting uses. By combining properties of blockchains, and the decentralization, and Web3 technologies to ensure the future of the nation is kept secure. Electronic voting is coming eventually, so let's make it as secure and scalable as possible.

## Introduction
There's potential to create several solutions to many of our modern by leveraging recent advances in technologies. Technology allows us to do many things that would have seemed impossible a few generations, seemly allowing us to surpass our physical and cognitive limitations. But the same things that make technology so convenient, are what make it a bad fit for systems that have the potential to shape society in ways it's creators wouldn't even imagine. Electronic voting is one such system, but what if there's a way to have the convenience of technology and the limitations that make it difficult for malicious actors to take advantage of the physical system. The wave of decentralized technology, blockchain, and Web3 offer a new way of building systems that are less vulnerable to the attacks of malicious actors. The existence of the blockchain and its continuity proves that it's possible to design systems that don't allow one member or actor to exert an influence that's against the interests of the majority. This is perfect for democracy.

#### Threat Model
Before we consider the design of our system, we must know what we are up against. What is our attacker capable of and what are the weaknesses of existing systems.
- Our adversaries have an army of smart hackers and computers ready to attack.
- They have the ability to corrupt members of the network as well as individual voters.
- They have access to and the ability to corrupt the nodes in the network in order to introduce, alter or delete votes.
- Furthermore, they have access to the network infrastructure, and can easily see all traffic.

Under such conditions, it's easy to see why a naive implementation of an electronic voting system would lead to malicious parties gaining an upper hand and undermining democracy. We also see that these problems arise from the fundamental property of our current technology, allowing the bad actor an array of options on a wide attack surface area. The fact that technology is so convenient is what makes attacks against electronic voting scalable. Many studies have shown many previous voting systems have failed to ensure the integrity of the system, we hope our base system can be used as a starting point into better ways to approach the issues of implementing an Electronic voting system.

#### Trust
Before we move into the details of the system, we must talk about trust. We know that power has the ability to corrupt, and those in power can do things against the better interest of the people. Any attempts to make a system in which we must trust a certain entity are void, because they do not solve the actual problem, only move the issue to another area, potentially making things worse because they could make it easier to compromise the system. No matter what, they must be no central entity to be trusted, as they become the new target. Not even the developers should be able to overbearingly control in the system, and like physical voting, we must make sure the amount of effort required to attack our system, isn't scalable.

We need the people to trust the system, not other people. This requires the voter knowing that they have been counted correctly, and being able to verify this both publicly and individually. There must be clear who has won and who has lost and election. Election security is always a concern, so the integrity of the system must never come into question, as the results of this would be disastrous.

This brings us the fundamental pillars we strive to reach. **Correctness**, **Secrecy**, **Coercion Resistance** and **Recoverability**.

## Key Properties
In order to gain complete confidence in the system, these key properties must be attained.

### Correctness
- Votes must be **cast-as-intended**, **recorded-as-counted**, and **counted-as-recorded**.
- Anyone must be able to verify the correctness of each vote.
- Any voter can have proof that their votes were recorded and counted as intended.
- Eligibility verification must be done before a vote is cast.

### Secrecy and Coercion Resistance
- Votes must be kept secret during and after.
- The system must ever reveal how a given voter voted, to prevent vote selling and coercion
	- Individual vote secrecy, through voter receipt and publicly posted data, no inference can be made on voter's choice
	- Community vote secrecy though voter receipt and publicly posted data, no inference can be made on community's voting preferences
- Coercion resistance, An adversary instructing a voter to vote in a certain way cannot determine if the voter followed the instructions or not

### Recoverability
- Software Independence, a detected change or error in an election outcome can be corrected although re-running the election should be possible, it's best to have it has a last resort.
- Public VVPR Verifiably, one-to-one correspondence of VVPR with the digitally recorded vote.
- Resolve whether recorded-as-intended using publicly available data.

These are the considerations and heights we hope to reach with our system.

#### Notable Protocols
A few notable mentions in previous attempts. Each representing a general direction.
- Homomorphic tallying
- Scantegrity II
- Star-vote
- Optical scanning

Ideally, the system should be hybrid to allow of integration with offline machines that connect those in rural areas to the system to not be unfair against those without technological sophistication.

## The Design of Ballot
Ballot is a decentralized voting system that takes inspiration from blockchain technology and other decentralized and distributed systems. As you'll see, it implements its own blockchain but forgoes a consensus mechanism, rather, it treats the blockchain like a database and handles state replication with novel database partitioning and replication algorithms to allow for massive gains in scalability while being tamperproof just like a distributed ledger. Under the hood, it uses and manages multiple blockchains for different purposes, as we'll see.

Ballot is interface agnostic, this means it can be embedded in an application or used through a command line interface or an API. These interfaces do not interact with the internal blockchains directly, rather they interact with the first major architectural component of the system called **circle**.

Ballot has two major abstractions on which everything else is build upon. 
- **Circle**, which contains the state replication algorithms, provides an interface for the interfaces such as command lines and CLI and manages all the network specific events and operations in the network.
- **Store**, which manages blockchains and adds a custom type of blockchain called a **linked-chain** which allows for partial mutability by linking two chains, one of which is mutable while the other is not.

### Circle
Any number of nodes is allowed to participate in the network, but only a few of the nodes actually have the requirements and qualifications to carry out and election. Those nodes who are meet the criteria and choose to hold the election are referred to as part of the circle, which is a group of node that work together to ensure everything runs during and election, thus for our context, a circle is analogous with an election. The abstraction that handles these functions as well as a few other is called **Circle**.

Members of the circle coordinate each other using a data structure with the help of the Interplanetary Linked-Data packages, and uses these to share information about the partitions of data for distributed computation that allows for scaling of the network. Using a protocol called Insanely Scalable State Machine Replication (ISS) developed by member of Protocol labs, we have a multi-leader network that is fault-tolerant to account for the bottlenecks in the network and hardware. This is the source of the network's scalability. This data is the put in different stores to allow for immutability of votes

In addition to ISS, circle contains the mechanisms for recovery after temperament of the stores, and a minimal API which is used to attach other Interfaces and APIs to.

### Store
Stores are blockchains. To be more precise, they can be one or more chains. Just like a conventional blockchain, a store contains a chain of blocks and some additional configuration. The chain contains blocks linked by their headers, and additional metadata to allow for optimizations and queries of the chain. Because each block is connected to every other block by its header file which is essentially a hash of the previous block, any attempts to change or temper with the chain after it's created will destroy the database and force recovery, thus resisting attempts to alter the votes cast.

## Walkthrough
How do store and circle help us achieve our goals of creating an electronic voting system that has the properties we desire.

#### 1) Announcement
Once enough nodes are in a network, a new circle is announced for the nodes to join, the election begins once a candidate list is shared with all nodes in the network. Each option/candidate has a corresponding store which as their ballot.

#### 2) Verification
Details of each verified voter are stored as a hash, in an immutable chain, to be used later for verifying each vote.

#### 3) Vote: Partitioning -Replication
Part of the ISS specification is the partitioning of incoming votes, using the Interplanetary Linked-Data package, nodes in the circle can share the votes to be stored and partition them, then each of the nodes can work on making sure the other nodes correctly replicate the segment it's responsible.

#### 4) Store
Once incoming votes have been partitioned, they are given to the store and turned into blocks, each block is hashed to generate the header of the next block making which means any temperate will result in a block that's too short in comparison to the others.

#### 5) Recovery
In the event of temperament, nodes will periodically check that they have correctly replicated the data. Any temperate will result in a broken chain, thus triggering recovery which deletes all the data on the current node until the most correct block, and with the help of the other nodes, recovers all the data that's not presently stored on the block.

#### 6) Counting 
This is done in three ways.
1. Whoever has the longest chain can be determined.
2. Each chain has metadata that contains a number of entries for a more precise tally.
3. Lastly, a verified count, in which each entry in every block is verified with a reference blockchain of verified voters. This is the final definitive count released at the end of the circle/election.

#### 7) Self Verification
The voters unique has can be regenerated and used to query their votes, giving them a confimation if their unique hash is found in the any of the candidate stores.

This is a highlighted description of the working of the system and theory behind its design. More can be understood by looking at the code, as there are purposeful omissions and additions to highlight the designs and considerations of the design of the ballot.

## Possible concerns and Limitations
- In theory, a randomization attacks is possible. A voter may be forced to vote for whichever candidate that appears at a fixed position in the permutation, but these should average out and can be mitigated by using random nodes.
- Lack of public confidence due to complex cryptography, which leads to a lack of understanding of how the system works by the larger public.

## Summaries, Structures and Short How's
- A vote is a hash composite from the voter's unique hash, its own ID and stored inside its respective candidate's store.
- A candidate is a unique-indexed pair of an index and a hash of some unique Identifier, that's stored in a linked-store where there's partial mutability and visibility
- A circle is a group of nodes responsible for holding the election, and run the main protocal
- A store is a container for a chain that contains configuration, is the main interface for interacting with chains of blocks.
- A chain is a list of linked blocks with additional metadata for search purposes.
- A block is a file with hashes or json data, that's linked to the next and previous blocks in a chain. 
