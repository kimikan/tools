package main

import (
	"fmt"
	"net"
	"os"
	"runtime"
	"sync"
	"strconv"
)

func main() {
	runtime.GOMAXPROCS(runtime.NumCPU())
	if len(os.Args) != 2 {
		fmt.Fprintf(os.Stderr, "Usage: %s host ", os.Args[0])
		os.Exit(1)
	}
	addr := os.Args[1]
	fmt.Println("[+]Scaning", addr)
	wg := sync.WaitGroup{}
	wg.Add(10)
	
	for i := 0; i < 10000; i++ {
		port := strconv.Itoa(i)
		go ScanPort(addr, port, &wg)
	}
	wg.Wait()

}

func ScanPort(ip, port string, wg *sync.WaitGroup) {
	service := ip + ":" + port
	_, err := net.Dial("tcp", service)
	if err != nil {
		//fmt.Println("[-]", port, "Close")
	} else {
		fmt.Println("[+]", port, "Open")
	}
	wg.Done()
}
