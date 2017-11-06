package main

import (
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
	
	trueList = []string{"127.0.0.1:6666"}
	if len(trueList) <= 0 {
		fmt.Println("鍚庣�疘P鍜岀��鍙ｄ笉鑳界┖,鎴栬€呮棤鏁�")
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
			fmt.Println("寤虹珛杩炴帴閿欒��:%v\n", err)
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
		fmt.Printf("杩炴帴%v澶辫触:%v\n", ip, err)
		return
	}
	ExitChan := make(chan bool, 1)
	go func(sconn net.Conn, dconn net.Conn, Exit chan bool) {
		_, err := io.Copy(dconn, sconn)
		fmt.Printf("寰€%v鍙戦€佹暟鎹�澶辫触:%v\n", ip, err)
		ExitChan <- true
	}(sconn, dconn, ExitChan)
	go func(sconn net.Conn, dconn net.Conn, Exit chan bool) {
		_, err := io.Copy(sconn, dconn)
		fmt.Printf("浠�%v鎺ユ敹鏁版嵁澶辫触:%v\n", ip, err)
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
