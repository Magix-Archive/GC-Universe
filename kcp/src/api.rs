use std::ffi::c_void;
use std::io::{self, Write};

use Kcp;
use kcp::KCP_OVERHEAD;
use Error;

type KcpOutputCallback = extern "C" fn(buf: *const char, len: i32, kcp: *mut Kcp<Stub>, user: *mut c_void) -> i32;

//#[derive(Default)]
pub struct Stub
{
    kcp: *mut Kcp<Stub>,
    callback: Option<KcpOutputCallback>,
}

impl Write for Stub {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        match self.callback {
            None => Ok(data.len()),
            Some(callback) => {
                let buf = data.as_ptr() as *const char;
                let null: *mut c_void = std::ptr::null_mut();
                let _written = callback(buf, data.len() as i32, self.kcp, null);
                //print!("Written {} bytes\n", written);
                //Ok(written as usize)
                // TODO: this is HACK around some buggy uses!
                Ok(data.len())
            },
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn ikcp_create(conv: u32, token: u32, user: *mut c_void) -> *mut Kcp<Stub> {
    if !user.is_null() {
        panic!("Rust-KCP doesn't support user data!");
    }

    let s = Stub {
        kcp: std::ptr::null_mut() as *mut Kcp<Stub>,
        callback: None,
    };

    let k = Kcp::new(conv, token, s);

    return Box::into_raw(Box::new(k));
}

// release kcp control object
#[no_mangle]
pub extern "C" fn ikcp_release(kcp: *mut Kcp<Stub>) {
    unsafe {
        Box::from_raw(kcp);
    }
}

// set output callback, which will be invoked by kcp
#[no_mangle]
pub extern "C" fn ikcp_setoutput(kcp: *mut Kcp<Stub>, callback: KcpOutputCallback) {
    unsafe {
        (*kcp).output.0.kcp = kcp;
        (*kcp).output.0.callback = Some(callback);
    }
}

// user/upper level recv: returns size, returns below zero for EAGAIN
#[no_mangle]
pub extern "C" fn ikcp_recv(kcp: *mut Kcp<Stub>, buffer: *mut u8, len: i32) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts_mut(buffer, len as usize) };
    unsafe {
        let ret = (*kcp).recv(slice);
        match ret {
            Ok(size) => return size as i32,
            Err(ref e) => match e {
                Error::RecvQueueEmpty => return -1,
                Error::ExpectingFragment => return -2,
                _ => ret.unwrap() as i32,
            },
        }
    }
}

// user/upper level send, returns below zero for error
#[no_mangle]
pub extern "C" fn ikcp_send(kcp: *mut Kcp<Stub>, buffer: *const u8, len: i32) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts(buffer, len as usize) };
    unsafe {
        return (*kcp).send(slice).unwrap() as i32;
    }
}

// update state (call it repeatedly, every 10ms-100ms), or you can ask 
// ikcp_check when to call it again (without ikcp_input/_send calling).
// 'current' - current timestamp in millisec. 
#[no_mangle]
pub extern "C" fn ikcp_update(kcp: *mut Kcp<Stub>, current: u32) {
    unsafe {
        (*kcp).update(current).unwrap();
    }
}

// Determine when should you invoke ikcp_update:
// returns when you should invoke ikcp_update in millisec, if there 
// is no ikcp_input/_send calling. you can call ikcp_update in that
// time, instead of call update repeatly.
// Important to reduce unnacessary ikcp_update invoking. use it to 
// schedule ikcp_update (eg. implementing an epoll-like mechanism, 
// or optimize ikcp_update when handling massive kcp connections)
#[no_mangle]
pub extern "C" fn ikcp_check(kcp: *const Kcp<Stub>, current: u32) -> u32 {
    unsafe {
        return (*kcp).check(current);
    }
}

// when you received a low level packet (eg. UDP packet), call it
#[no_mangle]
pub extern "C" fn ikcp_input(kcp: *mut Kcp<Stub>, data: *const u8, size: i64) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts(data, size as usize) };
    unsafe {
        let ret = (*kcp).input(slice);

        match ret {
            Ok(size) => return size as i32,
            Err(ref e) => match e {
                Error::InvalidSegmentSize(_size) => return -1,
                _ => ret.unwrap() as i32,
            },
        }
    }
}

// flush pending data
#[no_mangle]
pub extern "C" fn ikcp_flush(kcp: *mut Kcp<Stub>) {
    unsafe {
        (*kcp).flush().unwrap();
    }
}

// check the size of next message in the recv queue
#[no_mangle]
pub extern "C" fn ikcp_peeksize(kcp: *const Kcp<Stub>) -> i32 {
    unsafe {
        return (*kcp).peeksize().unwrap() as i32;
    }
}

// change MTU size, default is 1400
#[no_mangle]
pub extern "C" fn ikcp_setmtu(kcp: *mut Kcp<Stub>, mtu: i32) -> i32 {
    unsafe {
        let ret = (*kcp).set_mtu(mtu as usize);
        match ret {
            Ok(_) => return 0,
            Err(_) => return -1, // TODO: there may be -2 also
        }
    }
}

// set maximum window size: sndwnd=32, rcvwnd=32 by default
#[no_mangle]
pub extern "C" fn ikcp_wndsize(kcp: *mut Kcp<Stub>, sndwnd: i32, rcvwnd: i32) -> i32 {
    unsafe {
        (*kcp).set_wndsize(sndwnd as u16, rcvwnd as u16);
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn ikcp_waitsnd(kcp: *const Kcp<Stub>) -> i32 {
    unsafe {
        return (*kcp).wait_snd() as i32;
    }
}

// fastest: ikcp_nodelay(kcp, 1, 20, 2, 1)
// nodelay: 0:disable(default), 1:enable
// interval: internal update timer interval in millisec, default is 100ms
// resend: 0:disable fast resend(default), 1:enable fast resend
// nc: 0:normal congestion control(default), 1:disable congestion control
#[no_mangle]
pub extern "C" fn ikcp_nodelay(kcp: *mut Kcp<Stub>, nodelay: i32, interval: i32, resend: i32, nc: i32) -> i32 {
    unsafe {
        (*kcp).set_nodelay(nodelay != 0, interval, resend, nc != 0);
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn ikcp_rcvbuf_count(_kcp: *const Kcp<Stub>) -> i32 {
    panic!("Function exists in header but is not implemented!");
}

#[no_mangle]
pub extern "C" fn ikcp_sndbuf_count(_kcp: *const Kcp<Stub>) -> i32 {
    panic!("Function exists in header but is not implemented!");
}

/*
#[no_mangle]
pub unsafe extern "C" fn ikcp_log(kcp: *mut Kcp<Stub>, mask: i32, fmt: *const char, ...) {
    panic!("Foo");
}
*/

// setup allocator
#[no_mangle]
pub extern "C" fn ikcp_allocator(_new_malloc: extern "C" fn(size: u64) -> *mut c_void, _new_free: extern "C" fn(ptr: *mut c_void)) {
    panic!("Setting custom allocator is not supported for Rust-KCP!");
}

// read conv
#[no_mangle]
pub extern "C" fn ikcp_getconv(ptr: *const c_void) -> u32 {
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, KCP_OVERHEAD) };
    return ::get_conv(slice);
}
