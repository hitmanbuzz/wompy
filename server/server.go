package main

import (
	"fmt"
	"net"
)

const IP = "127.0.0.1:6969"

type User struct {
	id   uint
	ip   string
	name string
	msg  []string
}

type Server struct {
	hostIP string
	listen net.Listener
	// store no.s of connections even if they are disconnected
	totalConn uint
	// store no.s of good connection only
	goodConn uint
	users    map[uint]*User
}

func NewServer() *Server {
	return &Server{
		hostIP:    IP,
		listen:    nil,
		totalConn: 0,
		goodConn:  0,
		users:     make(map[uint]*User),
	}
}

func (s *Server) SetServer() {
	ln, err := net.Listen("tcp", s.hostIP)
	if err != nil {
		panic(err)
	}

	fmt.Println("Server running at:", s.hostIP)
	s.listen = ln
}

func (s *Server) RunServer() {
	if s.listen == nil {
		panic("Server is not set properly")
	}

	defer s.listen.Close()

	for {
		conn, err := s.listen.Accept()
		if err != nil {
			fmt.Println(err)
			continue
		}
		go s.handleConnection(conn)
	}
}

func (s *Server) handleConnection(conn net.Conn) {
	defer conn.Close()
	buf := make([]byte, 1024)
	fmt.Printf("New Connection: %s\n", conn.RemoteAddr().String())
	s.totalConn++
	s.goodConn++

	new_user := User{
		id:   s.totalConn,
		ip:   conn.RemoteAddr().String(),
		name: fmt.Sprintf("default-%d", s.totalConn),
		msg:  make([]string, 0),
	}

	s.users[s.totalConn] = &new_user

	for {
		n, err := conn.Read(buf)
		if err != nil {
			s.goodConn--
			fmt.Printf("`%s` disconneted\n", new_user.name)
			return
		}

		msg := string(buf[:n])
		s.InsertMessageUser(new_user.id, msg)
	}
}

func (s *Server) InsertMessageUser(id uint, msg string) {
	_, exist := s.users[id]
	if !exist {
		return
	}

	s.users[id].msg = append(s.users[id].msg, msg)
	user := s.users[id]
	// fmt.Printf("`default-%d` msg inserted: %s\n", id, msg)
	fmt.Printf("Name: %s | IP: %s | Msgs: %v\n", user.name, user.ip, user.msg)
}

func main() {
	server := NewServer()
	server.SetServer()
	server.RunServer()
}
