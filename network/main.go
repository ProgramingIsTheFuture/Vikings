package main

import (
	"network/auth"
	"network/game"
)

func main() {
	go auth.Auth()

	game.Game()

}
