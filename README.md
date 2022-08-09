# Vikings

2D Vikings game

### Network

Server uses both TCP and UDP

TCP is used to validate and do the login to all the users that want to get into the game.
The server will determine if the user should enter or not into the world.

***For now the authentication is based on the username***

UDP will be used to send all the information about all other users that are playing inside the world.

#### TO DO

- [ ] Spawn players
- [ ] Move players
- [ ] Building the interaction between players (Showing all players inside the game)
- [ ] System to control the packets received and sent from/to the server (Avoid killing the server) 
- [ ] Make performance tests to the UDP server
- [ ] Add password to the authentication system and a unique identifier like (JWT or UUID)
- [ ] Database to store the user information
- [ ] Way to store previous data inside the world (location, items, etc...) [Redis?]
