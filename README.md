# Final_Project_Graphs

My final project is about the following dataset : https://snap.stanford.edu/data/twitch-social-networks.html. It is about links between Twitch streamers of a same nationality : the goal of this project is to check if the "6 degrees seperation" rule is respected when one looks to at such a special social media platform as Twitch. 

This dataset is divied into 6 parts, each one representing streamers (nodes) of a same nationality and the fiendly links (edges) between them : it is also interesting to compare different nationalities to see if they all obey to the main rule to the same extent. Here, I will use the French, English and Portuguese dataset : these are different in composition (number of nodes, edges) and in size which makes the comparison meaningful.

Now, let's explain how I coded this project.

---

First, I chose to represent the undirected graph with an adjacency list. To do so, in the six-degrees_module.rs, :
* The struct Graph contains the number of vretices as well as the adjacency list of the graph.
* The function reverse_edges() will be used to create an undirected graph by adding the inverse of all the edges of a directed graph
* 4 methods are implemented into the struct Graph : 
  * One will add all directed edges to the graph with the function add_directed_edges() with respect to a list of edges given as input.
  * One will then sort the edges with the function sort_graph_list()
  * This will let us create a directed graph by implementing the sorted list of edges into it wwith the function create_directed()
  * Thus, by creating a directed graph and adding the inverse of all directed edges, one can create an undirected graph with the function create_undirected()
 
---

Now that the graph is well implemented, one can :
* Read the file in order to create the list of edges. Each line of the file consists of two numbers (indicating that the first vertex is linked to the second one) seperated by a coma. Thus, one has to create an empty list_of_edges and to push every edge when the program reads them.
* Build the graph : the function let us avoid redundant code. With the number of vertices (indicated on the website introducing the dataset) and a string indicating the nationality at stake, on can create an undirected graph by reading the file and using the methods presented above.

---

Third, let's focus on the two main functions that let us computing the six degrees of seperation :
* One needs first to compute the distance between two vertices with distance_2_vertices() :
  * One initializes two varibales: the queue (for BFS) and a list storing the distance between the starting node and all others. The initial value of the queue is the starting node and the element in the list distance corresponding to the starting node is initialized to 0.
  * Then, the while loop continues to run as long as there is an element in the queue. Inside the while loop :
    - If the vector that one visits is the terminal node, one stops the program and returns the distance between the starting and the terminal nodes.
    - If not, one computes BFS algorithm by adding 1 for each neighbor of the visited node that where not visited before.
* Now, one can compute an approximation of the 6 degrees of seperation : after taking a graph as an input, the for loop will let us visit all the nodes of the graph. Inside this loop :
  * We will compute 10 distances between the node at stake and a random node in the graph. If one wanted to compute the degrees of seperation completely, one should have computed the distance between all nodes. However, given the considerable size of my dataset, the running time would have been enormous : even with an arbitrary number of terminal nodes (10), it could take up to 10 minutes for the program to execute compute the degrees of seperation for my largest dataset. Thus, by choosing 10, I considerably reduce the running time while having a pretty accurate estimation of the degrees of seperation in the graph.
  * This loop will add the distance between those two vertices to a variable sum that will be divided by len*10 at the end. On top of that, the program counts the number of times the 6 degrees of seperation rule is violated : again, this is not accurate since we don't compute every distance but it is a reasonable approximation to compare different nationalities.
* At the end, the program returns the average distance between two nodes as well as the number of times the initial rule is violated.

---

After that, one can print all the results and compare them together :
* The function print_results() will print for each nationality :
  * The average degree of seperation.
  * The accuracy of the 6 degrees of seperation rule on the graph.
  * The number of times the rule was violated
* At the end, the main() function will also outputs the nationality that respects the least the initial rule.

---

This is a possible output :
```
French
Average of degrees of seperation between two nodes: 2.68061
The 6 degrees seperation rule is respected 100.00% of the time
Number of trips violating the rule: 0
Elapsed time: 549.1411381s

English
Average of degrees of seperation between two nodes: 3.67856 
The 6 degrees seperation rule is respected 98.99% of the time
Number of trips violating the rule: 72
Elapsed time: 267.8726369s

Portugese
Average of degrees of seperation between two nodes: 2.52850
The 6 degrees seperation rule is respected 100.00% of the time
Number of trips violating the rule: 0
Elapsed time: 34.3655015s

The nationality respecting the least the rule of six degrees of seperation is "English"
```

---

Thus, the results we have are the following :
* The accuracy between those countries are linked to the ratio number of vertices/number of edges : the lower the ratio, the lower the average degree of seperation.
* I computed the running time of the function for 1) receiving feedbaks on my function's complexity and 2) justifying the choice of computing an approximation of the degrees of seperation and not the precise one.
