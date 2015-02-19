#![allow(deprecated)]

use super::branchify::generate_branchified_method;
use super::get_writer;
use std::old_io::IoResult;

pub fn generate(output_dir: Path) -> IoResult<()> {
    let mut writer = get_writer(output_dir, "read_method.rs");
    try!(writer.write_all(b"\
// This automatically generated file is included in request.rs.
pub mod dummy {
use std::old_io::{Stream, IoResult};
use method::Method;
use method::Method::{Connect, Delete, Get, Head, Options, Patch, Post, Put, Trace, ExtensionMethod};
use server::request::MAX_METHOD_LEN;
use rfc2616::{SP, is_token_item};
use buffer::BufferedStream;

#[inline]
pub fn read_method<S: Stream>(stream: &mut BufferedStream<S>) -> IoResult<Method> {
"));

    try!(generate_branchified_method(
        &mut *writer,
        branchify!(case sensitive,
            "CONNECT" => Connect,
            "DELETE"  => Delete,
            "GET"     => Get,
            "HEAD"    => Head,
            "OPTIONS" => Options,
            "PATCH"   => Patch,
            "POST"    => Post,
            "PUT"     => Put,
            "TRACE"   => Trace
        ),
        1,
        "stream.read_byte()",
        "SP",
        "MAX_METHOD_LEN",
        "is_token_item(b)",
        "ExtensionMethod({})"));
    writer.write_all(b"}\n}\n")
}
