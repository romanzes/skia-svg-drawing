use std::fs::File;
use std::io::{BufReader, Write};
use skia_safe::{Canvas, Color, EncodedImageFormat, Size, surfaces};
use skia_safe::svg::{Dom as SvgDom};

const WIDTH: i32 = 1080;
const HEIGHT: i32 = 1920;

fn main() {
    pdf_test();
}

pub fn png_test() {
    if let Some(mut surface) =
        surfaces::raster_n32_premul((WIDTH, HEIGHT))
    {
        draw(surface.canvas());
        let image = surface.image_snapshot();
        let mut context = surface.direct_context();
        if let Some(data) = image.encode(context.as_mut(), EncodedImageFormat::PNG, None) {
            let bytes = data.as_bytes();
            let mut output_file = File::create("./output.png").unwrap();
            output_file.write_all(bytes).unwrap();
        }
    }
}

pub fn pdf_test() {
    let document = skia_safe::pdf::new_document(None);
    let mut page = document.begin_page(Size::new(WIDTH as f32, HEIGHT as f32), None);
    draw(page.canvas());
    let data = page.end_page().close();
    let mut file = File::create("./output.pdf").unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

fn draw(canvas: &mut Canvas) {
    canvas.clear(Color::WHITE);
    let file = File::open("./pinocchio.svg").unwrap();
    let reader = BufReader::new(file);
    let mut svg = SvgDom::read(reader).unwrap();
    svg.set_container_size((WIDTH, HEIGHT));
    svg.render(canvas);
}
