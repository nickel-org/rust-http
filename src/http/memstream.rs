/// TODO: submit upstream

use std::old_io::{IoResult, Seek, SeekStyle};
use std::old_io::{MemReader, MemWriter};

/// Writes to an owned, growable byte vector but also implements read with fail-on-call methods.
struct MemWriterFakeStream(MemWriter);

impl MemWriterFakeStream {
    pub fn new() -> MemWriterFakeStream { MemWriterFakeStream(MemWriter::new()) }

    pub fn get_ref(&self) -> &[u8] {
        let &MemWriterFakeStream(ref s) = self;
        s.get_ref()
    }
}

impl Writer for MemWriterFakeStream {
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        let &mut MemWriterFakeStream(ref mut s) = self;
        s.write_all(buf)
    }

    fn flush(&mut self) -> IoResult<()> {
        let &mut MemWriterFakeStream(ref mut s) = self;
        s.flush()
    }
}

impl Reader for MemWriterFakeStream {
    fn read(&mut self, _buf: &mut [u8]) -> IoResult<usize> {
        panic!("Uh oh, you didn't aught to call MemWriterFakeStream.read()!")
    }
}

/// Reads from an owned byte vector, but also implements write with fail-on-call methods.
pub struct MemReaderFakeStream(MemReader);

impl MemReaderFakeStream {
    pub fn new(buf: Vec<u8>) -> MemReaderFakeStream { MemReaderFakeStream(MemReader::new(buf)) }
}

impl Reader for MemReaderFakeStream {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let &mut MemReaderFakeStream(ref mut s) = self;
        s.read(buf)
    }
}

impl Seek for MemReaderFakeStream {
    fn tell(&self) -> IoResult<u64> {
        let &MemReaderFakeStream(ref s) = self;
        s.tell()
    }

    fn seek(&mut self, pos: i64, style: SeekStyle) -> IoResult<()> {
        let &mut MemReaderFakeStream(ref mut s) = self;
        s.seek(pos, style)
    }
}

impl Writer for MemReaderFakeStream {
    fn write_all(&mut self, _buf: &[u8]) -> IoResult<()> {
        panic!("Uh oh, you didn't aught to call MemReaderFakeStream.write_all()!")
    }
    fn flush(&mut self) -> IoResult<()> {
        panic!("Uh oh, you didn't aught to call MemReaderFakeStream.flush()!")
    }
}

#[cfg(test)]
mod test {
    use super::{MemReaderFakeStream, MemWriterFakeStream};

    #[test]
    fn test_mem_writer_fake_stream() {
        let mut writer = MemWriterFakeStream::new();
        assert_eq!(writer.get_ref(),            []);
        assert_eq!(writer.write_all(&[0]),          Ok(()));
        assert_eq!(writer.get_ref(),            [0]);
        assert_eq!(writer.write_all(&[1, 2, 3]),    Ok(()));
        assert_eq!(writer.write_all(&[4, 5, 6, 7]), Ok(()));
        assert_eq!(writer.get_ref(),            [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_mem_reader_fake_stream() {
        let mut reader = MemReaderFakeStream::new(vec!(0, 1, 2, 3, 4, 5, 6, 7));
        let mut buf = vec![];
        assert_eq!(reader.read(buf.as_mut_slice()),      Ok(0));
        assert_eq!(reader.tell(),              Ok(0));
        let mut buf = vec![0];
        assert_eq!(reader.read(buf.as_mut_slice()),      Ok(1));
        assert_eq!(reader.tell(),              Ok(1));
        assert_eq!(buf,                        vec![0]);
        let mut buf = vec![0, 0, 0, 0];
        assert_eq!(reader.read(buf.as_mut_slice()),      Ok(4));
        assert_eq!(reader.tell(),              Ok(5));
        assert_eq!(buf,                        vec![1, 2, 3, 4]);
        assert_eq!(reader.read(buf.as_mut_slice()),      Ok(3));
        assert_eq!(&buf[0..3],                  [5, 6, 7]);
        assert_eq!(reader.read(buf.as_mut_slice()).ok(), None);
    }
}
