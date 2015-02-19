use headers::serialization_utils::{push_quoted_string, quoted_string, WriterUtil};
use std::old_io::IoResult;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityTag {
    pub weak: bool,
    pub opaque_tag: String,
}

pub fn weak_etag(opaque_tag: String) -> EntityTag {
    EntityTag {
        weak: true,
        opaque_tag: opaque_tag,
    }
}

pub fn strong_etag(opaque_tag: String) -> EntityTag {
    EntityTag {
        weak: false,
        opaque_tag: opaque_tag,
    }
}

impl fmt::Display for EntityTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.weak {
            f.write_str(&push_quoted_string(String::from_str("W/"), &self.opaque_tag)[])
        } else {
            f.write_str(&quoted_string(&self.opaque_tag)[])
        }
    }
}

impl super::HeaderConvertible for EntityTag {
    fn from_stream<R: Reader>(reader: &mut super::HeaderValueByteIterator<R>) -> Option<EntityTag> {
        let weak = match reader.next() {
            Some(b) if b == b'W' || b == b'w' => {
                if reader.next() != Some(b'/') || reader.next() != Some(b'"') {
                    return None;
                }
                true
            },
            Some(b) if b == b'"' => {
                false
            },
            _ => {
                return None;
            }
        };
        let opaque_tag = match reader.read_quoted_string(true) {
            Some(tag) => tag,
            None => return None,
        };
        reader.some_if_consumed(EntityTag {
            weak: weak,
            opaque_tag: opaque_tag,
        })
    }

    fn to_stream<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
        if self.weak {
            try!(writer.write_all(b"W/"));
        }
        writer.write_quoted_string(&self.opaque_tag)
    }

    fn http_value(&self) -> String {
        format!("{}", self)
    }
}

#[test]
fn test_etag() {
    use headers::test_utils::{assert_conversion_correct, assert_interpretation_correct,
                              assert_invalid};
    assert_conversion_correct("\"\"", strong_etag(String::new()));
    assert_conversion_correct("\"fO0\"", strong_etag(String::from_str("fO0")));
    assert_conversion_correct("\"fO0 bar\"", strong_etag(String::from_str("fO0 bar")));
    assert_conversion_correct("\"fO0 \\\"bar\"", strong_etag(String::from_str("fO0 \"bar")));
    assert_conversion_correct("\"fO0 \\\"bar\\\"\"", strong_etag(String::from_str("fO0 \"bar\"")));

    assert_conversion_correct("W/\"\"", weak_etag(String::new()));
    assert_conversion_correct("W/\"fO0\"", weak_etag(String::from_str("fO0")));
    assert_conversion_correct("W/\"fO0 bar\"", weak_etag(String::from_str("fO0 bar")));
    assert_conversion_correct("W/\"fO0 \\\"bar\"", weak_etag(String::from_str("fO0 \"bar")));
    assert_conversion_correct("W/\"fO0 \\\"bar\\\"\"", weak_etag(String::from_str("fO0 \"bar\"")));
    assert_interpretation_correct("w/\"fO0\"", weak_etag(String::from_str("fO0")));

    assert_invalid::<EntityTag>("");
    assert_invalid::<EntityTag>("fO0");
    assert_invalid::<EntityTag>("\"\\\"");
    assert_invalid::<EntityTag>("\"\"\"\"");
}
