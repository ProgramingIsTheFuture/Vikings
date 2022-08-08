package main

import (
	"fmt"
	"net"
)

func main() {
	packet, err := net.ListenPacket("udp", "127.0.0.1:9999")
	if err != nil {
		panic(err)
	}

	defer packet.Close()

	fmt.Println("Reading...")
	buffer := make([]byte, 1024)
	n, addr, err := packet.ReadFrom(buffer)
	if err != nil {
		return
	}

	fmt.Println(string(buffer[:n]), addr)

	// for
	for {
		_, err = packet.WriteTo([]byte("Hello from server"), addr)
		if err != nil {
			continue
		}

	}

}
