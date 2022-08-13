package game

import (
	"net"
)

type User struct {
	Username string   `json:"username"`
	Addr     net.Addr `json:"addr"`
}

type WorldType []User

var World = WorldType{}

func Game() {
	packet, err := net.ListenPacket("udp", "127.0.0.1:9999")
	if err != nil {
		panic(err)
	}

	defer packet.Close()

	// World loop
	for {
		for _, v := range World {
			_, err = packet.WriteTo([]byte("Hello from server"), v.Addr)
			if err != nil {
				panic(err)
			}
		}
	}
}
