mod a;
mod b;
mod deserializer;
mod h;
mod i;
mod inline_code;
mod p;
mod s;
mod serializer;
mod text;
mod yamd;

use a::A;
use h::H;
use p::P;
use yamd::Yamd;

use crate::deserializer::Branch;

fn main() {
    let mut p = P::new();
    p.push(A::new("http://foo.bar/", "http://foo.bar/"));
    let t = Yamd::new().push(H::new("foo", 1)).push(p);
    println!("{t:?}");
}
