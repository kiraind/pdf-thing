pub struct Meta {
    pub title: String,
    pub author: String,
    pub lang: String,       // ISO 639-1
    pub created: String,    // ISO 8601
    pub edited: String,     // ISO 8601
    pub generator: String,
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
                _ => { println!("Unknown key: '{}'", key) }
            }
        }

        meta
    }
}

pub struct Fragment {

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

        while let Some(key) = get_next_string(iter) {
            match key.as_ref() {
                "id" => {
                    page.id = get_next_int(iter).unwrap() as u32;
                },
                "format" => {
                    page.format = get_next_string(iter).unwrap();
                },
                "fragments" => {
                    
                },
                _ => { /* todo warn */ }
            }
        }

        Some(page)
    }
}

pub struct Document {
    pub metadata: Meta,

    pub pages: Vec<Page>,
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

impl Document {
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
                            // println!("Koko {}", chars_iter.next().unwrap());
                        },
                        "pages" => {
                            while let Some(page) = Page::from_json(&mut chars_iter) {
                                doc.pages.push(page);
                            }
                        },
                        _ => { /* todo warn */ }
                    }
                }
            }
        }

        doc
    }
}