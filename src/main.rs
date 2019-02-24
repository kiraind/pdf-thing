extern crate printpdf;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

static MONTSERRAT_BASELINE_SHIFT: f64 = 0.86;

struct AtomicText {
    text: String,
    x: f64,
    y: f64,
    width: f64,
    classes: Vec<String>
}

fn main() {
    let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let text = "Проблемы координации";

    let mut font_reader = std::io::Cursor::new(include_bytes!("../fonts/Montserrat-SemiBold.ttf").as_ref());

    let font = doc.add_external_font(&mut font_reader).unwrap();

    // `use_text` is a wrapper around making a simple string
    let font_size = 31; //MontserratBaselineShift

    current_layer.use_text(
        text,
        font_size,
        Mm(0.0) + Mm(20.0),
        Mm(297.0) - Mm(20.0) - Mm(20.8) - Pt(font_size as f64 * MONTSERRAT_BASELINE_SHIFT).into(),
        &font
    );

    let text2 = "Когда они оказались в кабинете, профессор Квиррелл запечатал дверь, раз-";

    let mut font_reader2 = std::io::Cursor::new(include_bytes!("../fonts/OpenSans-Regular.ttf").as_ref());

    let font2 = doc.add_external_font(&mut font_reader2).unwrap();

    let font_size2 = 12;

    current_layer.use_text(
        text2,
        font_size2,
        Mm(0.0) + Mm(20.0) + Mm(10.0),
        Mm(297.0) - Mm(20.0) - Mm(39.63) - Pt(font_size2 as f64 * MONTSERRAT_BASELINE_SHIFT).into(),
        &font2
    );



    // text fill color = blue
    let blue = Rgb::new(13.0 / 256.0, 71.0 / 256.0, 161.0 / 256.0, None);
    let orange = Rgb::new(244.0 / 256.0, 67.0 / 256.0, 54.0 / 256.0, None);
    current_layer.set_fill_color(Color::Rgb(blue));
    current_layer.set_outline_color(Color::Rgb(orange));

    // For more complex layout of text, you can use functions
    // defined on the PdfLayerReference
    // Make sure to wrap your commands
    // in a `begin_text_section()` and `end_text_section()` wrapper
    // current_layer.begin_text_section();
 
    //     // setup the general fonts.
    //     // see the docs for these functions for details
    //     current_layer.set_font(&font, 24);
    //     current_layer.set_text_cursor(Mm(10.0), Mm(100.0));
    //     current_layer.set_line_height(24);
    //     current_layer.set_word_spacing(3000);
    //     current_layer.set_character_spacing(10);
    

    //     // write two lines (one line break)
    //     current_layer.write_text(text, &font);
    //     current_layer.add_line_break();
    //     current_layer.write_text(text2, &font);
    //     current_layer.add_line_break();

    //     current_layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
    //     current_layer.set_character_spacing(0);
    //     current_layer.set_text_matrix(TextMatrix::Rotate(10.0));

    //     // write one line, but write text2 in superscript
    //     current_layer.write_text(text, &font);
    //     current_layer.set_line_offset(10);
    //     current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
    //     current_layer.set_font(&font, 18);
    //     current_layer.write_text(text2, &font);

    // current_layer.end_text_section();
    
    let (page2, _) = doc.add_page(Mm(210.0), Mm(297.0), "Page 2, Layer 1");
    let _ = doc.get_page(page2).add_layer("Layer 3");


    doc.save(&mut BufWriter::new(File::create("test_fonts.pdf").unwrap())).unwrap();
}