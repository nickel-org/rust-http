use std::fmt;
use std::str::FromStr;
use std::ascii::AsciiExt;

pub use self::Method::{Options, Get, Head, Post, Put, Delete, Trace,
                       Connect, Patch, ExtensionMethod};

/// HTTP methods, as defined in RFC 2616, §5.1.1.
///
/// Method names are case-sensitive.
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum Method {
    Options,
    Get,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Connect,
    Patch,  // RFC 5789
    ExtensionMethod(String),
}

impl FromStr for Method {
    type Err = ();
    /**
     * Get a *known* `Method` from an *ASCII* string, regardless of case.
     *
     * If you want to support unregistered methods, use `from_str_or_new` instead.
     *
     * (If the string isn't ASCII, this will at present fail: TODO fix that.)
     */
    fn from_str(method: &str) -> Result<Method, ()> {
        if !method.is_ascii() {
            return Err(());
        }
        match method {
            "OPTIONS" => Ok(Options),
            "GET"     => Ok(Get),
            "HEAD"    => Ok(Head),
            "POST"    => Ok(Post),
            "PUT"     => Ok(Put),
            "DELETE"  => Ok(Delete),
            "TRACE"   => Ok(Trace),
            "CONNECT" => Ok(Connect),
            "PATCH"   => Ok(Patch),
            _         => Err(())
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Options                => "OPTIONS",
            Get                    => "GET",
            Head                   => "HEAD",
            Post                   => "POST",
            Put                    => "PUT",
            Delete                 => "DELETE",
            Trace                  => "TRACE",
            Connect                => "CONNECT",
            Patch                  => "PATCH",
            ExtensionMethod(ref s) => &s[..],
        })
    }
}

impl Method {
    /**
     * Get a `Method` from an *ASCII* string.
     *
     * (If the string isn't ASCII, this will at present fail.)
     */
    pub fn from_str_or_new(method: &str) -> Option<Method> {
        assert!(method.is_ascii());
        Some(match method {
            "OPTIONS" => Options,
            "GET"     => Get,
            "HEAD"    => Head,
            "POST"    => Post,
            "PUT"     => Put,
            "DELETE"  => Delete,
            "TRACE"   => Trace,
            "CONNECT" => Connect,
            "PATCH"   => Patch,
            _         => ExtensionMethod(String::from_str(method)),
        })
    }
}
