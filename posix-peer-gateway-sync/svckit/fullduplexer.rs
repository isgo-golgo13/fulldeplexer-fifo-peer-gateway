use std::io::{Read, Result, Write};

pub struct Context;

pub trait FullDuplexer {
    fn send(&self, ctx: &Context, reader: &mut dyn Read, n: i64) -> Result<usize>;
    fn send_all(&self, ctx: &Context, readers: Vec<&mut dyn Read>, n: i64) -> Result<usize>;
    fn receive(&self, ctx: &Context, writer: &mut dyn Write, n: i64) -> Result<usize>;
    fn receive_all(&self, ctx: &Context, writers: Vec<&mut dyn Write>, n: i64) -> Result<usize>;
}
