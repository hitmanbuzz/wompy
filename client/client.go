package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
)

func main() {
	conn, err := net.Dial("tcp", "127.0.0.1:6969")
	if err != nil {
		panic(err)
	}

	fmt.Println("Connected to 127.0.0.1:6969")
	defer conn.Close()

	for {
		fmt.Printf(">> ")
		reader := bufio.NewReader(os.Stdin)
		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Println("Error reading input:", err)
			continue
		}

		_, err = conn.Write([]byte(input))
		if err != nil {
			fmt.Printf("Error sending msg: %v\n", err)
		}
	}
}
