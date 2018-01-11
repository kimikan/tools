package main

import (
	"encoding/xml"
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"time"
)

type config struct {
	SourceFile string `xml:"source_file"`
	TargetExt  string `xml:"target_ext"`
	Interval   int    `xml:"interval"`
	StartTime  int    `xml:"start_time"`
	EndTime    int    `xml:"end_time"`
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

//CopyFile ...
func CopyFile(dstName, srcName string) (written int64, err error) {
	src, err := os.Open(srcName)
	fmt.Println(dstName, srcName)
	if err != nil {
		fmt.Println("src: ", err)
		return
	}
	defer src.Close()
	dst, err := os.OpenFile(dstName, os.O_WRONLY|os.O_CREATE, 0644)
	if err != nil {
		fmt.Println("dst: ", err)
		return
	}
	defer dst.Close()
	return io.Copy(dst, src)
}

func main() {

	cfg := loadConfig()

	if cfg == nil {
		return
	}

	for {
		f := func() {
			n := time.Now()
			n1 := n.Hour()*10000 + n.Minute()*100 + n.Second()
			n2 := n.Year()*10000 + n.Day() + int(n.Month())*100

			nx := n1 + n2*1000000
			fmt.Println("Ok, to go")
			if nx >= cfg.StartTime && nx <= cfg.EndTime {
				CopyFile(fmt.Sprint(nx, cfg.TargetExt), cfg.SourceFile)
			}
		}

		t := time.After(time.Duration(cfg.Interval) * time.Second)
		<-t
		f()
	}
}
