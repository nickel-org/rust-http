use std::old_io::{MemReader, MemWriter};
use std::fmt;
use std::vec::Vec;
use headers::{HeaderConvertible, HeaderValueByteIterator};

pub fn from_stream_with_str<T: HeaderConvertible>(s: &str) -> Option<T> {
    let mut bytes = s.bytes().collect::<Vec<_>>();
    bytes.push_all(b"\r\n/");
    let mut reader = MemReader::new(bytes);
    let mut iter = HeaderValueByteIterator::new(&mut reader);
    HeaderConvertible::from_stream(&mut iter)
}

pub fn to_stream_into_str<T: HeaderConvertible>(v: &T) -> String {
    let mut writer = MemWriter::new();
    v.to_stream(&mut writer).unwrap();
    String::from_utf8(writer.get_ref().to_vec()).unwrap()
}

// Verify that a value cannot be successfully interpreted as a header value of the specified type.
#[inline]
pub fn assert_invalid<T: HeaderConvertible + fmt::Debug>(string: &str) {
    assert_eq!(from_stream_with_str::<T>(string), None);
}

// Verify that all of the methods from the HeaderConvertible trait work correctly for the given
// valid header value and correct decoded value.
#[inline]
pub fn assert_conversion_correct<T: HeaderConvertible + fmt::Debug>(string: &'static str, value: T) {
    assert_eq!(from_stream_with_str(string), Some(value.clone()));
    let s = to_stream_into_str(&value);
    assert_eq!(&s[..], string);
    let s = value.http_value();
    assert_eq!(&s[..], string);
}

// Verify that from_stream interprets the given valid header value correctly.
#[inline]
pub fn assert_interpretation_correct<T: HeaderConvertible + fmt::Debug>(string: &'static str, value: T) {
    assert_eq!(from_stream_with_str(string), Some(value));
}
