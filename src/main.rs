mod nodes;
mod sd;

use nodes::a::A;
use nodes::h::H;
use nodes::p::P;
use nodes::yamd::Yamd;

use sd::deserializer::Branch;

fn main() {
    let mut p = P::new();
    p.push(A::new("http://foo.bar/", "http://foo.bar/"));
    let t = Yamd::new().push(H::new("foo", 1)).push(p);
    println!("{t:?}");
}
