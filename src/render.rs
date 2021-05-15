use std::io::Write;

pub trait Render {
    fn render<W: Write>(&self, writer: &mut W);
}
