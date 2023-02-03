mod a;
mod b;
mod deserializer;
mod h;
mod i;
mod inline_code;
mod mdy;
mod p;
mod s;
mod serializer;
mod text;

use a::A;
use h::H;
use mdy::Mdy;
use p::P;

use crate::deserializer::Branch;

fn main() {
    let mut p = P::new();
    p.push(A::new("http://foo.bar/", "http://foo.bar/"));
    let t = Mdy::new().push(H::new("foo", 1)).push(p);
    println!("{t:?}");
}
