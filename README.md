## Experiment 2.1: Original code of broadcast chat

### `server.rs`:
![alt text](image.png)  
The continuous loop in `server.rs` handles both incoming and outgoing messages. The first task in the loop, `Some(msg)`, receives incoming messages from connected clients, then broadcasts the messages to all connected clients. The second task in the loop, `Ok(msg)`, broadcasts the outgoing message by the client to themselves.

### `client.rs`:
![alt text](image-1.png)  
The continuous loop in `client.rs` handles both user input and server messages. The first task in the loop, `line`, take the user input and sends the message to the server. The second task in the loop, `msg`, receives incoming messages from the server and display them to the user.

### What happens when user types some texts in the client? 
As explained in the previous explanation on how the `server.rs` and `client.rs` work, when a user types some texts in a connected client, the input text will be sent to the server and the server will broadcast the input text to all connected clients including the client where the input text came from. Every connected clients will receive this message broadcasted by the server and will display the message to the user.

![alt text](image-2.png)

For example, in the picture above, firstly in the first client terminal (second from left most terminal) the user typed `"this is client 1 typing"` and so that exact text is broadcasted and displayed to all clients, including the first client terminal, which is why there are two texts of `"this is client 1 typing"` in the first client terminal, first one being the input typed by the user and the second one being the message broadcasted by the server and displayed by the client to the user.