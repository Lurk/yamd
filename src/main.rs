mod a;
mod b;
mod h;
mod i;
mod inline_code;
mod mdy;
mod p;
mod s;
mod text;

use a::A;
use h::H;
use mdy::Mdy;
use p::P;

fn main() {
    let p = P::new().push(A::new(
        "http://foo.bar/",
        Some("http://foo.bar/".to_string()),
    ));
    let t = Mdy::new().push(H::new("foo", 1)).push(p);
    println!("{:?}", t);
}
