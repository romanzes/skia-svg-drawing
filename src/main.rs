use std::fs::File;
use std::io::{BufReader, Write};
use skia_safe::{Color, Size};
use skia_safe::svg::Dom as SvgDom;

fn main() {
    let document = skia_safe::pdf::new_document(None);
    let mut page = document.begin_page(Size::new(1080.0, 1920.0), None);
    page.canvas().clear(Color::from_rgb(255, 255, 255));

    let file = File::open("./pinocchio.svg").unwrap();
    let reader = BufReader::new(file);
    let svg = SvgDom::read(reader).unwrap();
    svg.render(page.canvas());

    let data = page.end_page().close();
    let mut file = std::fs::File::create("./output.pdf").unwrap();
    file.write_all(data.as_bytes()).unwrap();
}
