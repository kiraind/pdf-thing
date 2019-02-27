#![allow(non_snake_case)]

pub trait Atomic {
    
}

pub struct AtomicText {
    text: String,
    x: f64,
    y: f64,
    width: f64,
    classes: Vec<String>,
}

impl AtomicText {
    fn from_json(iter: &mut Iterator<Item=char>) -> Option<AtomicText> {
        let atomic = AtomicText {
            text: "".to_string(),
            x: 0.0,
            y: 0.0,
            width: 0.0,
            classes: vec![],
        };

        while let Some(ch) = iter.next() {
            if ch == '{' {
                break;
            } else if ch == ']' {
                return None;
            }
        }

        // todo

        let mut level = 0;

        while let Some(ch) = iter.next() {
            if ch == '{' {
                level += 1;
            } else if ch == '}' {
                level -= 1;
                
                if level == 0 {
                    break;
                }
            }
        }

        Some(atomic)
    }
}

impl Atomic for AtomicText {
    
}

pub struct Fragment {
    page_number: u32,
    x: f64,
    y: f64,
    height: f64,
    classes: Vec<String>,
    atomics: Vec<AtomicText>,
}

impl Fragment {
    fn from_json(iter: &mut Iterator<Item=char>) -> Option<Fragment> {
        while let Some(ch) = iter.next() {
            if ch == '{' {
                break;
            } else if ch == ']' {
                return None;
            }
        }

        let mut fragment = Fragment {
            page_number: 0,
            x: 0.0,
            y: 0.0,
            height: 0.0,
            classes: vec![],
            atomics: vec![],
        };

        while let Some(key) = get_next_string(iter) {
            match key.as_ref() {
                "pageNumber" => {
                    fragment.page_number = get_next_int(iter).unwrap() as u32;
                },
                "x" => {
                    fragment.x = get_next_float(iter).unwrap();
                },
                "y" => {
                    fragment.y = get_next_float(iter).unwrap();
                },
                "height" => {
                    fragment.height = get_next_float(iter).unwrap();
                },
                "classes" => {
                    while let Some(ch) = iter.next() {
                        if ch == '[' {
                            break;
                        }
                    }

                    'outer: while let Some(class) = get_next_string(iter) {
                        fragment.classes.push(class);

                        while let Some(ch) = iter.next() {
                            if ch == ']' {
                                while let Some(ch) = iter.next() {
                                    if ch == ',' || ch == '}' {
                                        break;
                                    }
                                }
                                break 'outer;
                            } else {
                                if ch == ',' {
                                    break;
                                }
                            }
                        }
                    }
                },
                "atomics" => {
                    while let Some(atomic) = AtomicText::from_json(iter) {
                        fragment.atomics.push(atomic);
                    }
                },
                _ => { println!("Unknown key of fragment: '{}'", key) }
            }
        }

        Some(fragment)
    }
}

pub struct Meta {
    pub title: String,
    pub author: String,
    pub lang: String,       // ISO 639-1
    pub created: String,    // ISO 8601
    pub edited: String,     // ISO 8601
    pub generator: String,
}

#[allow(dead_code)]
pub enum TextAlign {
    Left    = 0,
    Center  = 1,
    Right   = 2,
    Justify = 3,
}

#[allow(dead_code)]
pub enum FontWeight {
    Weight100,
    Weight200,
    Weight300,
    Weight400,
    Weight500,
    Weight600,
    Weight700,
    Weight800,
    Weight900,
}

#[allow(dead_code)]
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

impl Style {
    fn default() -> Style {
        Style {
            name: "Default".to_string(),

            pageMargins: [ 20.0, 20.0, 20.0, 20.0 ],
            firstLineIndent: 10.0,
            paragraphSpacing: 0.0,
            columnSpacing: 10.0,

            mainFontFamily: "Open Sans".to_string(),
            mainFontSize: 12,
            mainLineHeight: 1.5,
            mainTextAlign: TextAlign::Justify,
            mainTextHypnate: true,

            textEntryWidth: 1.0,
            textEntryLeftLineWidth: 0.0,
            textEntryBlockAlign: TextAlign::Left,
            textEntryLineNumbers: false,

            textEntryFontFamily: "Roboto Mono".to_string(),
            textEntryFontWeight: FontWeight::Weight400,
            textEntryFontSize: 10,
            textEntryLineHeight: 1.3,
            textEntryTextAlign: TextAlign::Left,

            textEntryCaption: false,
            textEntryCaptionFontWeight: FontWeight::Weight400,
            textEntryCaptionAlign: TextAlign::Left,

            headingFontFamily: "Montserrat".to_string(),

            h1_fontWeight: FontWeight::Weight600,
            h1_fontSize: 36,
            h1_topMargin: 26.0,
            h1_bottomMargin: 10.0,
            h1_textAlign: TextAlign::Left,

            h2_fontWeight: FontWeight::Weight600,
            h2_fontSize: 31,
            h2_topMargin: 20.8,
            h2_bottomMargin: 8.0,
            h2_textAlign: TextAlign::Left,

            h3_fontWeight: FontWeight::Weight600,
            h3_fontSize: 26,
            h3_topMargin: 15.6,
            h3_bottomMargin: 6.0,
            h3_textAlign: TextAlign::Left,

            h4_fontWeight: FontWeight::Weight600,
            h4_fontSize: 22,
            h4_topMargin: 10.4,
            h4_bottomMargin: 4.0,
            h4_textAlign: TextAlign::Left,

            h5_fontWeight: FontWeight::Weight600,
            h5_fontSize: 17,
            h5_topMargin: 5.2,
            h5_bottomMargin: 2.0,
            h5_textAlign: TextAlign::Left,

            h6_fontWeight: FontWeight::Weight600,
            h6_fontSize: 12,
            h6_topMargin: 0.0,
            h6_bottomMargin: 0.0,
            h6_textAlign: TextAlign::Left,

            listDefaultBullet: "—".to_string(),
            listBulletWeight: FontWeight::Weight600,
            listBulletRightMargin: 2.5,
            listTopMargin: 2.0,
            listBottomMargin: 2.0 
        }
    }

    fn from_json(iter: &mut Iterator<Item=char>) -> Option<Style> {
        #[allow(unused_mut)]
        let mut style = Style::default();

        let mut level = 0;

        while let Some(ch) = iter.next() {
            if ch == '{' {
                level += 1;
            } else if ch == '}' {
                level -= 1;

                if level == 0 {
                    break;
                }
            }
        }

        Some(style)
    }
}

impl Meta {
    fn from_json(iter: &mut Iterator<Item=char>) -> Meta {
        let mut meta = Meta {
            title: String::new(),
            author: String::new(),
            lang: String::new(),
            created: String::new(),
            edited: String::new(),
            generator: String::new(),
        };

        while let Some(ch) = iter.next() {
            if ch == '{' {
                break;
            }
        }

        while let Some(key) = get_next_string(iter) {
            match key.as_ref() {
                "title" => {
                    meta.title = get_next_string(iter).unwrap();
                },
                "author" => {
                    meta.author = get_next_string(iter).unwrap();
                },
                "lang" => {
                    meta.lang = get_next_string(iter).unwrap();
                },
                "created" => {
                    meta.created = get_next_string(iter).unwrap();
                },
                "edited" => {
                    meta.edited = get_next_string(iter).unwrap();
                },
                "generator" => {
                    meta.generator = get_next_string(iter).unwrap();
                },
                _ => { println!("Unknown key of meta: '{}'", key) }
            }
        }

        meta
    }
}

pub struct Page {
    pub id: u32,
    pub format: String,
    pub fragments: Vec<Fragment>,
}

impl Page {
    fn from_json(iter: &mut Iterator<Item=char>) -> Option<Page> {
        while let Some(ch) = iter.next() {
            if ch == '{' {
                break;
            } else if ch == ']' {
                return None;
            }
        }

        let mut page = Page {
            id: 0xffffffff,
            format: "".to_string(),
            fragments: Vec::new(),
        };

        'outer: while let Some(key) = get_next_string(iter) {
            match key.as_ref() {
                "id" => {
                    page.id = get_next_int(iter).unwrap() as u32;
                },
                "format" => {
                    page.format = get_next_string(iter).unwrap();
                },
                "fragments" => {
                    page.fragments.push( Fragment::from_json(iter).unwrap() );
                },
                _ => {
                    println!("Unknown key of page '{}'", key);
                }
            }
        }

        Some(page)
    }
}

pub struct Document {
    pub metadata: Meta,
    pub pages: Vec<Page>,
    pub style: Style,
}

pub fn get_next_string(iter: &mut Iterator<Item=char>) -> Option<String> {
    let mut string = String::new();

    let mut inside_str = false;

    while let Some(ch) = iter.next() {
        // wait for string to start
        if !inside_str {
            if ch == '"' {
                inside_str = true;
            } else if ch == ',' || ch == '{' || ch == '}' || ch == '[' || ch == ']' {
                return None
            } else {
                continue;
            }
        }

        while let Some(ch) = iter.next() {
            if ch == '"' {
                inside_str = false;
                break;
            }

            string.push(ch);
        }

        if !inside_str {
            break;
        }
    }

    while let Some(ch) = iter.next() {
        if  ch == ':' || ch == ',' || ch == ']' || ch == '}' {
            break;
        }
    }

    Some(string)
}

pub fn get_next_int(iter: &mut Iterator<Item=char>) -> Option<i64> {
    let mut number: i64 = 0;

    let mut started = false;

    while let Some(ch) = iter.next() {
        if ch.is_numeric() {
            started = true;
        } else if started {
            break
        }

        if started {
            number *= 10;
            number += ( (ch as u32) - ('0' as u32) ) as i64;
        }
    }

    Some(number)
}

pub fn get_next_float(iter: &mut Iterator<Item=char>) -> Option<f64> {
    let mut number: f64 = 0.0;

    let mut started    = false;
    let mut whole_part = true;
    let mut fractional_order = 0.1;

    while let Some(ch) = iter.next() {
        if ch == '.' {
            whole_part = false;
            continue;
        }

        if ch.is_numeric() {
            started = true;
        } else if started {
            break
        }

        if started {
            let digit = ( (ch as u32) - ('0' as u32) ) as f64;

            if whole_part {
                number *= 10.0;
                number += digit;
            } else {
                number += fractional_order * digit;
                fractional_order /= 10.0;
            }
        }
    }

    Some(number)
}

impl Document {
    #[allow(dead_code)]
    pub fn new() -> Document {
        Document {
            metadata: Meta {
                title: "New document".to_string(),
                author: "unknown".to_string(),
                lang: "en".to_string(),
                created: "1970-01-01T00:00:00.000Z".to_string(),
                edited: "1970-01-01T00:00:00.000Z".to_string(),
                generator: ":edf-thing".to_string(),
            },
            pages: Vec::new(),
            style: Style::default(),
        }
    }

    pub fn from_json(json: &String) -> Document {
        let mut doc = Document {
            metadata: Meta {
                title: String::new(),
                author: String::new(),
                lang: String::new(),
                created: String::new(),
                edited: String::new(),
                generator: String::new(),
            },
            pages: Vec::new(),
            style: Style::default(),
        };

        let mut chars_iter = json.chars();
        let mut level = 0;

        while let Some(ch) = chars_iter.next() {
            if level == 0 {
                if ch == '{' {
                    level += 1;
                }
                continue;
            }

            if level == 1 {
                if let Some(key) = get_next_string(&mut chars_iter) {
                    match key.as_ref() {
                        "doctype" => {
                            // todo check for ':'
                            let doctype = get_next_string(&mut chars_iter).unwrap();
                            
                            println!("Found doctype: '{}'", doctype);
                        },
                        "meta" => {
                            doc.metadata = Meta::from_json(&mut chars_iter);
                        },
                        "pages" => {
                            while let Some(page) = Page::from_json(&mut chars_iter) {
                                doc.pages.push(page);
                            }
                        },
                        "style" => {
                            doc.style = Style::from_json(&mut chars_iter).unwrap();
                        },
                        _ => {
                            println!("Unknown key of document: '{}'", key);
                        }
                    }
                }
            }
        }

        doc
    }
}