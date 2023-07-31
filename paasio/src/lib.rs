use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    bytes_through: usize,
    read_ops: usize,
}

impl<R: Read> ReadStats<R> {

    pub fn new(wrapped: R) -> ReadStats<R> {
        Self { reader: wrapped, bytes_through: 0, read_ops: 0 }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.read_ops
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.reader.read(buf) {
            Ok(bytes_read) => {
                self.bytes_through += bytes_read;
                self.read_ops += 1;
                Ok(bytes_read)
            }
            _error => panic!("Cannot read data")
        }

    }
}

pub struct WriteStats<W> {
    writer: W,
    bytes_through: usize,
    write_ops: usize,
}

impl<W: Write> WriteStats<W> {

    pub fn new(wrapped: W) -> WriteStats<W> {
        Self { writer: wrapped, bytes_through: 0, write_ops: 0 }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.write_ops
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.writer.write(buf) {
            Ok(bytes_written) => {
                self.bytes_through += bytes_written;
                self.write_ops += 1;
                Ok(bytes_written)
            }
            _error => panic!("Cannot write data")
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
