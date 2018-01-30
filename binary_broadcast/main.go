package main

import (
	"bytes"
	"encoding/binary"
	"encoding/xml"
	"errors"
	"fmt"
	"io/ioutil"
	"net"
	"strconv"
	"sync"
	"time"
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

type clients struct {
	sync.RWMutex
	_clients []net.Conn

	_selected net.Conn
}

func newClients() *clients {
	return &clients{
		_clients:  []net.Conn{},
		_selected: nil,
	}
}

func (p *clients) add(c net.Conn) {
	p.Lock()
	p._clients = append(p._clients, c)

	if p._selected == nil {
		p._selected = c
	}
	p.Unlock()
}

func (p *clients) remove(c net.Conn) {
	fmt.Println(p)
	p.Lock()
	for k, v := range p._clients {
		if v == c {
			v.Close()
			p._clients = append(p._clients[:k], p._clients[k+1:]...)

		}
	}
	p.Unlock()

	if c == p._selected {
		p.switchSelected()
	}
	fmt.Println(p)
}

func (p *clients) foreach(f func(net.Conn)) {
	p.RLock()
	for _, v := range p._clients {
		f(v)
	}
	p.RUnlock()
}

func (p *clients) selected() net.Conn {
	return p._selected
}

func (p *clients) clear() {
	p.foreach(func(c net.Conn) {
		c.Close()
	})

	p.Lock()
	p._clients = []net.Conn{}
	p.Unlock()
}

func (p *clients) empty() bool {
	p.RLock()
	defer p.RUnlock()

	return len(p._clients) <= 0
}

func (p *clients) switchSelected() {
	p.RLock()
	p._selected = nil
	if p._clients != nil {
		if len(p._clients) > 0 {
			p._selected = p._clients[0]
		}
	}
	p.RUnlock()
}

//Server indicats some info
type Server struct {
	_sourceWriteChan chan []byte

	_targetConn net.Conn
	_clients    *clients

	_logonReplay []byte
	_config      *config
}

func newServer(cfg *config) *Server {

	return &Server{
		_clients: newClients(),
		_config:  cfg,

		_sourceWriteChan: make(chan []byte),
	}
}

func (p *Server) close() {
	//never close
	p.clear()
}

//MsgHead defines a
//generic head structure
type MsgHead struct {
	_msgType    uint32
	_bodyLength uint32
}

func struct2Bytes(msg interface{}) ([]byte, error) {
	buf := bytes.NewBuffer(nil)

	err := binary.Write(buf, binary.BigEndian, msg)

	if err != nil {
		return nil, err
	}

	return buf.Bytes(), nil
}

func generateCheckSum(buf []byte) uint32 {
	var sum uint64
	for b := range buf {
		sum += uint64(int8(b))
	}

	return uint32(sum % 256)
}

//LoginMsgID ..
var LoginMsgID uint32 = 1

//SendCompID ..
var SendCompID = "F000648Q0011"

//TargetCompID ..
var TargetCompID = "VDE"

//HeartBeatInt ..
var HeartBeatInt uint32 = 20

//Password ..
var Password = "F000648Q0011"

//DefaultAppVerID ..
var DefaultAppVerID = "1.00"

func logon(conn net.Conn) error {
	type LogonMsg struct {
		header       MsgHead
		sendCompID   [20]byte
		targetCompID [20]byte
		heartBeat    uint32
		password     [16]byte
		version      [32]byte
		checksum     uint32
	}

	msg := LogonMsg{}
	msg.header._msgType = 1
	msg.header._bodyLength = 92

	copy(msg.sendCompID[:], []byte(SendCompID))
	copy(msg.targetCompID[:], []byte(TargetCompID))

	msg.heartBeat = HeartBeatInt
	copy(msg.password[:], []byte(Password))
	copy(msg.version[:], []byte(DefaultAppVerID))

	buf, err := struct2Bytes(&msg)
	if err != nil {
		return err
	}

	checksum := generateCheckSum(buf[:len(buf)-4])

	binary.BigEndian.PutUint32(buf[len(buf)-4:], checksum)

	n, err := conn.Write(buf)
	if err != nil {
		fmt.Println("logon: ", err)
		return err
	}

	if n != 104 {
		return errors.New("write failed")
	}

	return nil
}

func heartbeat(conn net.Conn) error {
	type heartbeatMsg struct {
		header   MsgHead
		checksum uint32
	}

	msg := heartbeatMsg{
		header: MsgHead{
			_msgType:    3,
			_bodyLength: 0,
		},
		checksum: 0,
	}

	bs, err := struct2Bytes(&msg)
	if err != nil {
		return err
	}
	checksum := generateCheckSum(bs[:len(bs)-4])

	binary.BigEndian.PutUint32(bs[len(bs)-4:], checksum)
	n, err2 := conn.Write(bs)
	if err2 != nil {
		return err2
	}

	if n != len(bs) {
		return errors.New("write failed")
	}
	return nil
}

func (p *Server) connectToTarget() error {

	if p._targetConn != nil {
		return errors.New("already connected! ")
	}

	dconn, err := net.Dial("tcp", p._config.TargetAddr)
	if err != nil {
		fmt.Println("Connect target failed: ", p._config.TargetAddr, err)
		return err
	}
	p._targetConn = dconn

	defer p.closeTarget()

	err = logon(dconn)
	if err != nil {
		fmt.Println("logon failed: ", err)
		return err
	}

	done := sync.WaitGroup{}

	done.Add(1)
	//recving.....
	go func() {
		for {
			msgType, buf, err := p.getMessage(dconn)
			if err != nil {
				fmt.Println("get message failed: ", err)
				break
			}

			if len(buf) > 0 {
				if msgType == 1 && p._logonReplay == nil {
					p._logonReplay = buf
					fmt.Println("logon reply: ", buf)
				}

				if msgType == 3 {
					err1 := heartbeat(dconn)
					if err1 != nil {
						fmt.Println("heartbeat failedx: ", err1)
						break
					}
				}

				if msgType != 1 {
					cs := []net.Conn{}
					p._clients.foreach(func(c net.Conn) {
						_, err := c.Write(buf)
						if err != nil {
							cs = append(cs, c)
							fmt.Println("write to client failed: ", err)
						}
					})

					for _, v := range cs {
						p._clients.remove(v)
					}
				}
			} //end buf>0
		} //end for
		done.Done()
	}()

	done.Add(1)
	//sending....
	go func() {
		for {
			t := time.After(time.Duration(1) * time.Second)
			<-t

			err1 := heartbeat(dconn)
			if err1 != nil {
				fmt.Println("heartbeat failed: ", err1)
				break
			}
		} // end for

		done.Done()
	}()

	done.Wait()

	return nil
}

func (p *Server) run() error {

	go func() {
		for {
			err := p.connectToTarget()
			if err != nil {
				fmt.Println("connect to target failed: ", err)
				t := time.After(time.Duration(1) * time.Second)
				<-t
				fmt.Println("sleep 2s to retry")
			}
		} // end for
	}()

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

func (p *Server) closeTarget() {
	if p._targetConn != nil {
		p._targetConn.Close()
		p._targetConn = nil
	}
}

func (p *Server) clear() {
	p._clients.clear()
	p.closeTarget()
}

func (p *Server) getMessage(conn net.Conn) (uint32, []byte, error) {
	var msgType uint32
	var bodyLen uint32
	err := binary.Read(conn, binary.BigEndian, &msgType)
	if err != nil {
		fmt.Println(err)
		return 0, nil, err
	}

	err = binary.Read(conn, binary.BigEndian, &bodyLen)
	if err != nil {
		return 0, nil, err
	}

	var buf []byte
	if bodyLen > 0 {
		buf = make([]byte, bodyLen)

		for {
			got := 0
			n, err2 := conn.Read(buf[got:])
			if err2 != nil || n < 0 {
				return 0, nil, err2
			}

			got += n

			if got < int(bodyLen) {
				continue
			}

			break
		}

	}

	var checksum uint32
	err2 := binary.Read(conn, binary.BigEndian, &checksum)
	if err2 != nil {
		return 0, nil, err2
	}

	bufx := make([]byte, bodyLen+12)
	//fmt.Println(msgType, bodyLen, buf)
	binary.BigEndian.PutUint32(bufx[:], msgType)
	binary.BigEndian.PutUint32(bufx[4:], bodyLen)
	binary.BigEndian.PutUint32(bufx[bodyLen+8:], checksum)
	copy(bufx[8:], buf[:])
	return msgType, bufx, nil
}

func (p *Server) handle(conn net.Conn) {

	go func() {
		defer p._clients.remove(conn)

		p._clients.add(conn)

		for {
			msgType, buf, err := p.getMessage(conn)
			if err != nil {
				break
			}

			if len(buf) > 0 {
				if msgType == 1 {
					conn.Write(p._logonReplay)
				}
			} else {
				fmt.Println("read 0 buffer")
				break
			}
		}
		fmt.Println("conn closed")
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
