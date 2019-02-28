use serde_repr::*;

pub trait Atomic {
    
}

#[derive(Serialize, Deserialize)]
pub struct RichString {
    pub chars: String,
    pub attrs: Vec<u32>,
    pub length: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AtomicText {
    pub text: RichString,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub classes: Vec<String>,
}

impl Atomic for AtomicText {
    
}

#[derive(Serialize, Deserialize)]
pub struct Fragment {
    pub page_number: u32,
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub classes: Vec<String>,
    pub atomics: Vec<AtomicText>,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub title: String,
    pub author: String,
    pub lang: String,       // ISO 639-1
    pub created: String,    // ISO 8601
    pub edited: String,     // ISO 8601
    pub generator: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum TextAlign {
    Left    = 0,
    Center  = 1,
    Right   = 2,
    Justify = 3,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum FontWeight {
    Weight100 = 100,
    Weight200 = 200,
    Weight300 = 300,
    Weight400 = 400,
    Weight500 = 500,
    Weight600 = 600,
    Weight700 = 700,
    Weight800 = 800,
    Weight900 = 900,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Style {
    name: String,
    pageMargins: [ f64; 4 ],
    firstLineIndent: f64,
    paragraphSpacing: f64,
    columnSpacing: f64,
    mainFontFamily: String,
    mainFontSize: u32,
    mainLineHeight: f64,
    mainTextAlign: TextAlign,
    mainTextHypnate: bool,
    textEntryWidth: f64,
    textEntryLeftLineWidth: f64,
    textEntryBlockAlign: TextAlign,
    textEntryLineNumbers: bool,
    textEntryFontFamily: String,
    textEntryFontWeight: FontWeight,
    textEntryFontSize: u32,
    textEntryLineHeight: f64,
    textEntryTextAlign: TextAlign,
    textEntryCaption: bool,
    textEntryCaptionFontWeight: FontWeight,
    textEntryCaptionAlign: TextAlign,
    headingFontFamily: String,

    h1_fontWeight: FontWeight,
    h1_fontSize: u32,
    h1_topMargin: f64,
    h1_bottomMargin: f64,
    h1_textAlign: TextAlign,

    h2_fontWeight: FontWeight,
    h2_fontSize: u32,
    h2_topMargin: f64,
    h2_bottomMargin: f64,
    h2_textAlign: TextAlign,

    h3_fontWeight: FontWeight,
    h3_fontSize: u32,
    h3_topMargin: f64,
    h3_bottomMargin: f64,
    h3_textAlign: TextAlign,

    h4_fontWeight: FontWeight,
    h4_fontSize: u32,
    h4_topMargin: f64,
    h4_bottomMargin: f64,
    h4_textAlign: TextAlign,

    h5_fontWeight: FontWeight,
    h5_fontSize: u32,
    h5_topMargin: f64,
    h5_bottomMargin: f64,
    h5_textAlign: TextAlign,

    h6_fontWeight: FontWeight,
    h6_fontSize: u32,
    h6_topMargin: f64,
    h6_bottomMargin: f64,
    h6_textAlign: TextAlign,

    listDefaultBullet: String,
    listBulletWeight: FontWeight,
    listBulletRightMargin: f64,
    listTopMargin: f64,
    listBottomMargin: f64 
}

#[derive(Serialize, Deserialize)]
pub struct Page {
    pub id: u32,
    pub format: String,
    pub fragments: Vec<Fragment>,
}

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub meta: Meta,
    pub pages: Vec<Page>,
    pub style: Style,
}