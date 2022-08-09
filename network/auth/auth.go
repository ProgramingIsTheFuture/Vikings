package auth

import (
	"encoding/json"
	"fmt"
	"net"
	"network/game"
	"strings"
)

func newAddr(ip, port string) net.Addr {
	return &net.UDPAddr{IP: net.ParseIP("127.0.0.1"), Port: 9997}

}

func authenticate(conn *net.TCPConn) {
	defer conn.Close()

	var buffer = make([]byte, 2048)
	n, err := conn.Read(buffer)
	if err != nil {
		conn.Close()
		return
	}

	type login struct {
		Port string `json:"port"`
		Data string `json:"data"`
	}

	type Data struct {
		Username string `json:"username"`
	}

	var loginData = login{}
	err = json.Unmarshal(buffer[:n], &loginData)
	if err != nil {
		conn.Close()
		return
	}

	var data = Data{}
	err = json.Unmarshal([]byte(loginData.Data), &data)
	if err != nil {
		conn.Close()
		return
	}

	newAddr := newAddr(strings.Split(conn.RemoteAddr().String(), ":")[0],
		loginData.Port)

	// After adding password we should here verify the password instead
	for _, v := range game.World {
		if v.Username == data.Username {
			// Edit to send a worning user already exists
			conn.Close()
			return
		}
	}

	// Last write to confirm the inserting into the world
	n, err = conn.Write([]byte("Success"))
	if err != nil {
		conn.Close()
		return
	}

	fmt.Println("Auth adding.. N: ", n, "Err: ", err)
	game.World = append(
		game.World,
		game.User{
			Username: data.Username,
			Addr:     newAddr,
		},
	)
}

func Auth() {
	tcpAddr := net.TCPAddr{IP: net.ParseIP("127.0.0.1"), Port: 9998}
	lstn, err := net.ListenTCP("tcp", &tcpAddr)

	if err != nil {
		panic(err)
	}

	for {
		conn, err := lstn.AcceptTCP()
		if err != nil {
			continue
		}

		go authenticate(conn)
	}
}
