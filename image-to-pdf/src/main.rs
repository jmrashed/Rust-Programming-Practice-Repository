use std::fs::File;
use image::io::Reader as ImageReader;
use printpdf::*;

fn main() {
    // Read the image file
    let image_path = "path/to/image.jpg"; // Replace with the actual path to the uploaded image
    let image = ImageReader::open(image_path).unwrap().decode().unwrap().to_rgb8();

    // Create a new PDF document
    let (doc, page1, layer1) = PdfDocument::new("Image to PDF", Mm(210.0), Mm(297.0), "Layer 1");

    // Draw the image onto the PDF page
    let image_width = Mm(image.width() as f64);
    let image_height = Mm(image.height() as f64);
    let image_area = PdfRect::new(Mm(0.0), Mm(0.0), image_width, image_height);
    let mut page_graphics = doc.get_page(page1).get_graphics();
    page_graphics.draw_image(
        &image,
        image_area,
        layer1.clone(),
    ).unwrap();

    // Save the PDF to a file
    let pdf_path = "output.pdf"; // Replace with the desired output file path
    let mut file = File::create(pdf_path).unwrap();
    doc.save(&mut file).unwrap();
}
