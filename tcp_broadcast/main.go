package main

import (
	"container/list"
	"encoding/binary"
	"encoding/xml"
	"fmt"
	"io/ioutil"
	"net"
	"strconv"
	"sync"
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

//Server indicats some info
type Server struct {
	sync.RWMutex
	_targetWriteChan chan []byte
	_sourceWriteChan chan []byte

	_targetConn net.Conn
	_clients    *list.List

	_config *config
}

func newServer(cfg *config) *Server {

	return &Server{
		_clients: list.New(),
		_config:  cfg,

		_sourceWriteChan: make(chan []byte),
		_targetWriteChan: make(chan []byte),
	}
}

func (p *Server) close() {
	//never close
	p.clear()
}

func (p *Server) run() error {

	localAddr := ":" + strconv.Itoa(p._config.LocalPort)
	lis, err := net.Listen("tcp", localAddr)
	if err != nil {
		fmt.Println(err)
		return err
	}
	defer lis.Close()

	for {
		conn, err := lis.Accept()
		if err != nil {
			fmt.Println("err: ", err)
			continue
		}
		fmt.Println(conn.RemoteAddr(), conn.LocalAddr())
		go p.handle(conn)
	}
	//return nil
}

func (p *Server) clear() {
	p.Lock()

	for {
		v := p._clients.Front()
		if v == nil {
			break
		}
		p._clients.Remove(v)

		if value, ok := v.Value.(net.Conn); ok {
			value.Close()
		}
	}
	p.Unlock()
	p._targetConn = nil
}

func (p *Server) getMessage(conn net.Conn) ([]byte, error) {

	var msgType uint32
	var bodyLen uint32
	err := binary.Read(conn, binary.BigEndian, &msgType)
	if err != nil {
		fmt.Println(err)
		return nil, err
	}

	err = binary.Read(conn, binary.BigEndian, &bodyLen)
	if err != nil {
		return nil, err
	}

	var buf []byte

	buf = make([]byte, bodyLen+12)
	if bodyLen > 0 {
		buf2 := buf[8:]
		for {
			got := 0
			n, err2 := conn.Read(buf2[got:])
			if err2 != nil || n < 0 {
				return nil, err2
			}

			got += n

			if got < int(bodyLen) {
				continue
			}

			break
		}
	}

	n, err := conn.Read(buf[bodyLen+8:])
	if err != nil || n <= 0 {
		return nil, err
	}

	binary.BigEndian.PutUint32(buf[:], msgType)
	binary.BigEndian.PutUint32(buf[4:], bodyLen)

	//fmt.Println(msgType, bodyLen, buf)
	return buf, nil
}

func (p *Server) dispatcher() {
	fmt.Println("dispatcher start")
	go func() {
		for {
			if p._targetConn != nil {
				buf, err := p.getMessage(p._targetConn)
				if err != nil {
					fmt.Println("target conn read: ", err)
					break
				}

				if len(buf) > 0 {
					p._sourceWriteChan <- buf
					//fmt.Println("target read: ", buf)
				}
			}
		}

		if p._targetConn != nil {
			p._targetConn.Close()
			p._targetConn = nil
		}
		p.clear()
		fmt.Println("dispatcher stop")
	}()

	for {
		select {
		case v, ok := <-p._sourceWriteChan:

			if ok {
				cs := []*list.Element{}
				p.RLock()
				for e := p._clients.Front(); e != nil; e = e.Next() {
					if value, ok := e.Value.(net.Conn); ok {
						_, err := value.Write(v)
						if err != nil {
							cs = append(cs, e)
							fmt.Println("write failed: ", err)
							value.Close()
						}
					} else {
						cs = append(cs, e)
					}
				}
				p.RUnlock()

				p.Lock()
				for _, v := range cs {
					p._clients.Remove(v)
				}
				p.Unlock()
			}
		case v, ok := <-p._targetWriteChan:
			if ok {
				if p._targetConn != nil {
					_, err := p._targetConn.Write(v)
					if err != nil {
						p.clear()
						break
					}
				} else {
					break
				}
			}
		} //end select
	} //end for
}

func (p *Server) handle(conn net.Conn) {

	if p._targetConn == nil {
		dconn, err := net.Dial("tcp", p._config.TargetAddr)
		if err != nil {
			fmt.Println("Connect target failed: ", p._config.TargetAddr, err)
			return
		}

		p._targetConn = dconn
		go p.dispatcher()
	}

	go func() {
		defer conn.Close()

		p.Lock()
		element := p._clients.PushBack(conn)
		p.Unlock()

		for {
			buf, err := p.getMessage(conn)
			if err != nil {
				break
			}
			fmt.Println("conn read: ", buf)

			if len(buf) > 0 {
				p.RLock()
				v := p._clients.Front()
				if v != nil {
					if v.Value == conn {
						p._targetWriteChan <- buf
					}
				}
				p.RUnlock()

			} else {
				fmt.Println("read 0 buffer")
				break
			}
		}
		fmt.Println("conn closed")
		p.Lock()

		p._clients.Remove(element)
		p.Unlock()
	}()
}

func main() {
	cfg := loadConfig()
	if cfg == nil {
		fmt.Println("app.def error")
		return
	}

	server := newServer(cfg)
	defer server.close()

	err := server.run()
	if err != nil {
		fmt.Println("Run failed: ", err)
	}
}
