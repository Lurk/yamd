mod nodes;
mod sd;

use nodes::anchor::Anchor;
use nodes::heading::Heading;
use nodes::paragraph::Paragraph;
use nodes::yamd::Yamd;

use sd::deserializer::Branch;

fn main() {
    let mut p = Paragraph::new();
    p.push(Anchor::new("http://foo.bar/", "http://foo.bar/"));
    let mut t = Yamd::new();
    t.push(Heading::new("foo", 1));
    t.push(p);
    println!("{t:?}");
}
