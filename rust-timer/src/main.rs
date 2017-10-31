
/*
 * to simulate the fuction like golang
 * select {
 * case time.After(30):
 *     doSomething();
 * case i := msg_chan:
 *     handleMsg();
 * }
 */
extern crate mio;
extern crate slab;

use mio::*;

use std::thread;
use std::time::*;

use std::vec::Vec;
use std::io;

#[derive(Debug)]
struct Timer {
    _seconds:u64,

    _registration:Registration,
}

impl Timer {
    fn new(long:u64)->Timer {
        
        let (registration, set_readiness) = Registration::new2();
        
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(long));

            //event triggered
            set_readiness.set_readiness(mio::Ready::readable()).unwrap();
        });
        
        Timer {
            _seconds:long,
            _registration : registration,
        }
    }
}

struct Context {
    _timers:slab::Slab<Timer, Token>,
    _events : Events,
}

impl Context {
    fn new()->Context {
        Context{
            _events:Events::with_capacity(1024),
            _timers:slab::Slab::with_capacity(1024),
        }
    }

    fn select(&mut self,  ds:&mut Vec<u64>) -> io::Result<()> {
        let poll = Poll::new()?;

        for t in ds {
            let index: Token;
            let timer = Timer::new(*t);
            { 
                let entry  = self._timers.vacant_entry().unwrap();
                index = entry.insert(timer).index();
            }

            let t2 = self._timers.get(index).unwrap();
            poll.register(&t2._registration, index, Ready::readable(), PollOpt::edge()).unwrap();
        };

        let size = poll.poll(&mut self._events, None)?;
        for i in 0..size {
            let event = self._events.get(i);

            if let Some(e) = event {
                let ready = e.readiness();
                let token = e.token();

                if ready.is_readable() {

                    let timer = self._timers.get(token);
                    println!("client: {:?}, token: {:?}", timer, token);                
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}

fn main() {
    
    let mut vec = vec![];
    vec.push(10);
    vec.push(10);
    vec.push(30);
    vec.push(20 );

    let mut context = Context::new();

    context.select(&mut vec).unwrap();
    println!("Hello, world!");
}
