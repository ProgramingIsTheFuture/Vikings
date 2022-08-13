package auth

import (
	"fmt"
	"net"
	"network/game"
)

// Creates a new net Addr
func newAddr(ip, port string) net.Addr {
	return &net.UDPAddr{IP: net.ParseIP("127.0.0.1"), Port: 9997}
}

// Check if this user already exists inside the world
func checkUserWorld(user User) bool {
	// After adding password we should here verify the password instead
	for _, v := range game.World {
		if v.Username == user.Username {
			// Edit to send a worning user already exists
			return false
		}
	}

	return true
}

// Adds a user to the game world
func addUser(user User, addr, port string) {
	newAddr := newAddr(addr, port)

	fmt.Printf("Adding [%s] to the World\n", user.Username)
	game.World = append(
		game.World,
		game.User{
			Username: user.Username,
			Addr:     newAddr,
		},
	)
}
