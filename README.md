# finalproject_210_mbta
Maps out all possible new connections in the mbta metro transit system and prints out an optimal (based on maximized ridership an minimized distance traveled) new path.

Final Project Writeup

The goal of my project was to find out where a potential new line in the MBTA should be placed. This potential new line would have the shortest amount of rail tracking, while also trying to target metro stops which are already busy (this would ideally reduce congestion in these stops).

The code does so by getting two files. The first file contains the name of each MBTA metro stop, its latitude and longitude points (to calculate distance), ridership data which would indicate how busy the stop is (this is used as weights for each node), and index number which is only used later on help connect data between the files. The functions which collect this information are placed in a separate module called importing.
The second file just has two columns and has a list of every single potential connection between metro stops (does not include stops that are already connected to each other and does not include any edges that would have to cross over another metro line - this data is not found by the program, it was created beforehand). 

The program has a struct for a graph. The graph is just a collection of nodes. Nodes are also a separate struct and contain data for each metro stop which is mostly found in the first file.

The program then uses both files to create a graph, using the first for the nodes and the second for the edges. The program will also calculate the distance between nodes as it imports the edges and adds the distance as weight for the edges (all edges are undirected).

Once the graph is set up, the program moves on to the algorithm section of the program. The function that I use was originally based on dijkstra's algorithm (and gradually strayed from the original function but still holds the basis of it by using a distances table and a heap). The ‘distances’ are a combination of the weights on the edges and the weights on the nodes. I wanted to maximize the node weight while minimizing the edge weight, so I designed the program to minimize the edge weight while dividing it by the node weight (the higher the node weight, the more the edge weight is lowered)(I also understand that this isn’t necessarily ‘maximizing’ and ‘minimizing’ in the real mathematical sense, but in the scope of what I know about shortest path algorithms, this is a close estimate). 

The function was originally designed to start at one random point and end at another, but was adjusted to keep going past the ‘end’ point and keep going until there are no more possible options for the path to go (the sum of the distances table falls below a certain threshold). 

This function produces a path which is not necessarily the best, but rather ‘good’ (it's randomized). To improve the quality of the final product, have the program run 100 times, and then I finally print the output with the highest total node weight. The final output is a vector of numbers (each one an index number for the metro stop in the path), and then prints the actual names of the metro stops in the ideal path.
