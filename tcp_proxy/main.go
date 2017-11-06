package main

import (
	"bufio"
	"fmt"
	"io"
	"net"
	"os"
	"sync"
)

var lock sync.Mutex
var trueList []string
var ip string

func main() {
	ip = ":9999"

	file, err := os.Open("app.def")
	if err != nil {
		return
	}
	defer file.Close()
	rb := bufio.NewReader(file)

	trueList = []string{}
	for {
		line, _, err := rb.ReadLine()

		if err == io.EOF {
			break
		}

		trueList = append(trueList, string(line))
		//do something fmt.Println(string(line))
	}

	if len(trueList) <= 0 {
		fmt.Println("no target address")
		os.Exit(1)
	}
	server()
}

func server() {
	lis, err := net.Listen("tcp", ip)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer lis.Close()
	for {
		conn, err := lis.Accept()
		if err != nil {
			fmt.Println("err: ", err)
			continue
		}
		fmt.Println(conn.RemoteAddr(), conn.LocalAddr())
		go handle(conn)
	}
}

func handle(sconn net.Conn) {
	defer sconn.Close()
	ip, ok := getIP()
	if !ok {
		return
	}
	dconn, err := net.Dial("tcp", ip)
	if err != nil {
		fmt.Println("Connect failed: ", ip, err)
		return
	}
	ExitChan := make(chan bool, 1)
	go func(sconn net.Conn, dconn net.Conn, Exit chan bool) {
		_, err := io.Copy(dconn, sconn)
		fmt.Println("copy failed:", ip, err)
		ExitChan <- true
	}(sconn, dconn, ExitChan)
	go func(sconn net.Conn, dconn net.Conn, Exit chan bool) {
		_, err := io.Copy(sconn, dconn)
		fmt.Println("Copy failed : ", ip, err)
		ExitChan <- true
	}(sconn, dconn, ExitChan)
	<-ExitChan
	dconn.Close()
}

func getIP() (string, bool) {
	lock.Lock()
	defer lock.Unlock()

	if len(trueList) < 1 {
		return "", false
	}
	ip := trueList[0]
	trueList = append(trueList[1:], ip)
	return ip, true
}
