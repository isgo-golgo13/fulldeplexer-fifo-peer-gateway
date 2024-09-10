use super::{fullduplexer_fifo_context::FifoContext, FullDuplexer}; // Import FifoContext
use libc::{open, read, write, O_CREAT, O_NONBLOCK, O_RDONLY, O_WRONLY};
use std::ffi::CString;
use std::io::{Read, Result, Write};
use std::os::unix::io::RawFd;

pub struct FullDuplexPeer {
    pub id: String,
    pub read_fifo: String,
    pub write_fifo: String,
}

impl FullDuplexPeer {
    // Updated to use FifoContext
    pub fn new(context: FifoContext) -> Self {
        unsafe {
            let read_path = CString::new(context.read_fifo.clone()).unwrap();
            let write_path = CString::new(context.write_fifo.clone()).unwrap();
            libc::mkfifo(read_path.as_ptr(), 0o644);
            libc::mkfifo(write_path.as_ptr(), 0o644);
        }

        Self {
            id: context.id,
            read_fifo: context.read_fifo,
            write_fifo: context.write_fifo,
        }
    }

    fn open_fifo(path: &str, flags: i32) -> Result<RawFd> {
        let c_path = CString::new(path).unwrap();
        let fd = unsafe { open(c_path.as_ptr(), flags | O_NONBLOCK) };
        if fd < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(fd)
        }
    }

    fn send_posix(&self, buffer: &[u8]) -> Result<usize> {
        let fd = Self::open_fifo(&self.write_fifo, O_WRONLY)?;
        let n = unsafe { write(fd, buffer.as_ptr() as *const _, buffer.len()) };
        if n < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(n as usize)
        }
    }

    fn receive_posix(&self, buffer: &mut [u8]) -> Result<usize> {
        let fd = Self::open_fifo(&self.read_fifo, O_RDONLY)?;
        let n = unsafe { read(fd, buffer.as_mut_ptr() as *mut _, buffer.len()) };
        if n < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(n as usize)
        }
    }
}

// FullDuplexer trait implementation
impl FullDuplexer for FullDuplexPeer {
    fn send(&self, _ctx: &Context, reader: &mut dyn Read, n: i64) -> Result<usize> {
        let mut buffer = vec![0; n as usize];
        reader.read(&mut buffer)?;
        self.send_posix(&buffer)
    }

    fn send_all(&self, _ctx: &Context, readers: Vec<&mut dyn Read>, n: i64) -> Result<usize> {
        let mut total_bytes = 0;
        for reader in readers {
            total_bytes += self.send(_ctx, reader, n)?;
        }
        Ok(total_bytes)
    }

    fn receive(&self, _ctx: &Context, writer: &mut dyn Write, n: i64) -> Result<usize> {
        let mut buffer = vec![0; n as usize];
        self.receive_posix(&mut buffer)?;
        writer.write_all(&buffer)?;
        Ok(buffer.len())
    }

    fn receive_all(&self, _ctx: &Context, writers: Vec<&mut dyn Write>, n: i64) -> Result<usize> {
        let mut total_bytes = 0;
        for writer in writers {
            total_bytes += self.receive(_ctx, writer, n)?;
        }
        Ok(total_bytes)
    }
}
