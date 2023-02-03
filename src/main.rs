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
    let mut t = Yamd::new();
    t.push(H::new("foo", 1));
    t.push(p);
    println!("{t:?}");
}
