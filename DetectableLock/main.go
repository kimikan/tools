package main

import (
	"fmt"
	"net"
	"os"
	"runtime"
	"strconv"
	"strings"
	"time"
)

type DetectableLock struct {
	c chan int
	timeOut int

	file string
	line int
}

func newDetectableLock(seconds int) *DetectableLock {
	l := &DetectableLock{
		c: make(chan int, 10),
		timeOut: seconds,
		file: "",
		line: 0,
	}
	l.c<-0
	return l
}

func (p* DetectableLock) lock() {
	select {
	case <-p.c:
		fmt.Println("f")
		break
	case <-time.After(time.Duration(p.timeOut) * time.Second):
		fmt.Println("fucking deadlock ", p.file, p.line)
	}

	_, file, line, ok := runtime.Caller(1)
	if ok {
		p.file = file
		p.line = line
	}
	//fmt.Println(file, line)
}

func (p* DetectableLock) unlock() {
	p.c <- 0
}

func main() {
	lock := newDetectableLock(2)
	go func() {
		fmt.Println("task1")
		lock.lock()
		fmt.Println("task1 got lock")
		time.Sleep(5 * time.Second)
		lock.unlock()
		fmt.Println("task1 end")
	}()
	go func() {
		fmt.Println("task2")
		lock.lock()
		fmt.Println("task2 got lock")
		time.Sleep(4 * time.Second)
		lock.unlock()
		fmt.Println("task2 end")
	}()

	var x string
	fmt.Scanf("%s", &x)
	fmt.Println("end")
}

func main2() {
	service := ":1200"
	tcpAddr, err := net.ResolveTCPAddr("tcp4", service)
	checkError(err)
	listener, err := net.ListenTCP("tcp", tcpAddr)
	checkError(err)
	for {
		conn, err := listener.Accept()
		if err != nil {
			continue
		}
		go handleClient(conn)
	}
}

func handleClient(conn net.Conn) {
	_ = conn.SetReadDeadline(time.Now().Add(2 * time.Minute)) // set 2 minutes timeout
	request := make([]byte, 128)                              // set maxium request length to 128B to prevent flood attack
	defer conn.Close()                                        // close connection before exit
	for {
		readLen, err := conn.Read(request)

		if err != nil {
			fmt.Println(err)
			break
		}

		if readLen == 0 {
			break // connection already closed by client
		} else if strings.TrimSpace(string(request[:readLen])) == "timestamp" {
			daytime := strconv.FormatInt(time.Now().Unix(), 10)
			_, _ = conn.Write([]byte(daytime))
		} else {
			daytime := time.Now().String()
			_, _ = conn.Write([]byte(daytime))
		}

		request = make([]byte, 128) // clear last read content
	}
}

func checkError(err error) {
	if err != nil {
		_, _ = fmt.Fprintf(os.Stderr, "Fatal error: %s", err.Error())
		os.Exit(1)
	}
}
