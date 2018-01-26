package main

import (
	"bytes"
	"compress/gzip"
	"fmt"
)

func main() {

	buffer := bytes.NewBuffer([]byte{})

	content := "hello world!"
	w := gzip.NewWriter(buffer)

	n, err := w.Write([]byte(content))
	fmt.Println(n, err)
	w.Close()

	r, err := gzip.NewReader(buffer)

	if err != nil {
		fmt.Println(err)
		return
	}

	var bytes [200]byte

	n, err = r.Read(bytes[:])

	if err != nil {
		fmt.Println(string(bytes[:n]))
	}
	r.Close()
}
