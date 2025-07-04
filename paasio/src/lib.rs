use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    num_reads: usize,
    num_bytes: usize,
    data: R,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            num_reads: 0,
            num_bytes: 0,
            data: wrapped,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes
    }

    pub fn reads(&self) -> usize {
        self.num_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.num_reads += 1;
        let result = self.data.read(buf)?;
        self.num_bytes += result;

        Ok(result)
    }
}

pub struct WriteStats<W> {
    num_writes: usize,
    num_bytes: usize,
    data: W,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            num_writes: 0,
            num_bytes: 0,
            data: wrapped,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes
    }

    pub fn writes(&self) -> usize {
        self.num_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.num_writes += 1;
        let result = self.data.write(buf)?;
        self.num_bytes += result;

        Ok(result)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
