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
    pub name: String,
    pub pageMargins: [ f64; 4 ],
    pub firstLineIndent: f64,
    pub paragraphSpacing: f64,
    pub columnSpacing: f64,
    pub mainFontFamily: String,
    pub mainFontSize: u32,
    pub mainLineHeight: f64,
    pub mainTextAlign: TextAlign,
    pub mainTextHypnate: bool,
    pub textEntryWidth: f64,
    pub textEntryLeftLineWidth: f64,
    pub textEntryBlockAlign: TextAlign,
    pub textEntryLineNumbers: bool,
    pub textEntryFontFamily: String,
    pub textEntryFontWeight: FontWeight,
    pub textEntryFontSize: u32,
    pub textEntryLineHeight: f64,
    pub textEntryTextAlign: TextAlign,
    pub textEntryCaption: bool,
    pub textEntryCaptionFontWeight: FontWeight,
    pub textEntryCaptionAlign: TextAlign,
    pub headingFontFamily: String,

    pub h1_fontWeight: FontWeight,
    pub h1_fontSize: u32,
    pub h1_topMargin: f64,
    pub h1_bottomMargin: f64,
    pub h1_textAlign: TextAlign,

    pub h2_fontWeight: FontWeight,
    pub h2_fontSize: u32,
    pub h2_topMargin: f64,
    pub h2_bottomMargin: f64,
    pub h2_textAlign: TextAlign,

    pub h3_fontWeight: FontWeight,
    pub h3_fontSize: u32,
    pub h3_topMargin: f64,
    pub h3_bottomMargin: f64,
    pub h3_textAlign: TextAlign,

    pub h4_fontWeight: FontWeight,
    pub h4_fontSize: u32,
    pub h4_topMargin: f64,
    pub h4_bottomMargin: f64,
    pub h4_textAlign: TextAlign,

    pub h5_fontWeight: FontWeight,
    pub h5_fontSize: u32,
    pub h5_topMargin: f64,
    pub h5_bottomMargin: f64,
    pub h5_textAlign: TextAlign,

    pub h6_fontWeight: FontWeight,
    pub h6_fontSize: u32,
    pub h6_topMargin: f64,
    pub h6_bottomMargin: f64,
    pub h6_textAlign: TextAlign,

    pub listDefaultBullet: String,
    pub listBulletWeight: FontWeight,
    pub listBulletRightMargin: f64,
    pub listTopMargin: f64,
    pub listBottomMargin: f64 
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