mod a;
mod b;
mod h;
mod i;
mod inline_code;
mod p;
mod tree;

use a::A;
use h::H;
use p::P;
use tree::Tree;

fn main() {
    let p = P::new().push(H::new("foo", 1)).push(A::new(
        "http://foo.bar//",
        Some("http://foo.bar//".to_string()),
    ));
    let t = Tree::new().push(p);
    println!("{:?}", t);
}
