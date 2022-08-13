package auth

import (
	"encoding/json"
	"net"
	"strings"
)

type User struct {
	Username string `json:"username"`
}

func authenticate(conn *net.TCPConn) {
	// Closes the connection after this function returns
	defer conn.Close()

	// Read the first TCP input
	var buffer = make([]byte, 2048)
	n, err := conn.Read(buffer)
	if err != nil {
		conn.Close()
		return
	}

	// Login structure
	type login struct {
		Port string `json:"port"`
		Data string `json:"data"`
	}

	var loginData = login{}
	err = json.Unmarshal(buffer[:n], &loginData)
	if err != nil {
		conn.Close()
		return
	}

	var user = User{}
	err = json.Unmarshal([]byte(loginData.Data), &user)
	if err != nil {
		conn.Close()
		return
	}

	if !checkUserWorld(user) {
		conn.Close()
		return
	}

	// Last write to confirm the inserting into the world
	_, err = conn.Write([]byte("Success"))
	if err != nil {
		conn.Close()
		return
	}

	addUser(user, strings.Split(conn.RemoteAddr().String(), ":")[0],
		loginData.Port)
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
