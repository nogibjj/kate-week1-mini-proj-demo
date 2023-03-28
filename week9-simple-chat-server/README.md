# Week9 Simple Chat Server
This is a basic chat server that allows multiple users to connect and send messages to each other using TCP.

## Example usage
* Start the server by `make run`. The server prints out a message when a client connects. Also, it prints out chat messages of all the clients.
![](0.png)
* The first client joins by connecting to the server: `telnet 127.0.0.1 8080`. The client can send messages, which will be send to all other users connected to this server. Also, it receives messages from all other users.
![](1.png)
* The second client joins by connecting to the server: `telnet 127.0.0.1 8080`.  
![](2.png)
* The third client joins by connecting to the server: `telnet 127.0.0.1 8080`
![](3.png)