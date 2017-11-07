package main

import (
	"encoding/xml"
	"fmt"
	"io"
	"io/ioutil"
	"net"
	"strconv"
)

type config struct {
	TargetAddr string `xml:"target_addr"`
	LocalPort  int    `xml:"local_port"`
}

func loadConfig() *config {
	content, err := ioutil.ReadFile("app.def")
	if err != nil {
		return nil
	}

	c := new(config)
	err = xml.Unmarshal(content, c)
	if err != nil {
		return nil
	}

	return c
}

func main() {
	cfg := loadConfig()
	if cfg == nil {
		fmt.Println("app.def error")
		return
	}

	server(cfg.TargetAddr, cfg.LocalPort)
}

func server(targetAddr string, port int) {
	localAddr := ":" + strconv.Itoa(port)
	lis, err := net.Listen("tcp", localAddr)
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
		go handle(conn, targetAddr)
	}
}

func handle(sconn net.Conn, targetAddr string) {
	defer sconn.Close()

	dconn, err := net.Dial("tcp", targetAddr)
	if err != nil {
		fmt.Println("Connect failed: ", targetAddr, err)
		return
	}

	ExitChan := make(chan bool, 1)
	go func(sconn net.Conn, dconn net.Conn, Exit chan bool) {
		_, err := io.Copy(dconn, sconn)
		if err != nil {
			fmt.Println("copy failed:", targetAddr, err)
		}
		ExitChan <- true
	}(sconn, dconn, ExitChan)

	go func(sconn net.Conn, dconn net.Conn, Exit chan bool) {
		_, err := io.Copy(sconn, dconn)
		if err != nil {
			fmt.Println("Copy failed : ", targetAddr, err)
		}
		ExitChan <- true
	}(sconn, dconn, ExitChan)

	<-ExitChan
	dconn.Close()
}
