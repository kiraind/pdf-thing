pub struct Page {

}

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

        let mut level = 0;

        while let Some(ch) = iter.next() {
            if level == 0 {
                if ch == '{' {
                    level += 1;
                }
                continue;
            }

            if level == 1 {
                let key = get_next_string(iter);

                match key.as_ref() {
                    "title" => {
                        meta.title = get_next_string(iter);
                    },
                    "author" => {
                        meta.author = get_next_string(iter);
                    },
                    "lang" => {
                        meta.lang = get_next_string(iter);
                    },
                    "created" => {
                        meta.created = get_next_string(iter);
                    },
                    "edited" => {
                        meta.edited = get_next_string(iter);
                    },
                    "generator" => {
                        meta.generator = get_next_string(iter);
                    },
                    _ => { /* todo warn */ }
                }
            }

        }

        meta
    }
}

pub struct Document {
    pub metadata: Meta,

    pub pages: Vec<Page>,
}

pub fn get_next_string(iter: &mut Iterator<Item=char>) -> String {
    let mut string = String::new();

    let mut inside_str = false;

    while let Some(ch) = iter.next() {
        // wait for string to start
        if !inside_str {
            if ch == '"' {
                inside_str = true;
            } else {
                continue
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

    string
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
                let key = get_next_string(&mut chars_iter);

                match key.as_ref() {
                    "doctype" => {
                        // todo check for ':'
                        let doctype = get_next_string(&mut chars_iter);
                        
                        println!("Found doctype: '{}'", doctype);
                    },
                    "meta" => {
                        doc.metadata = Meta::from_json(&mut chars_iter);
                    }
                    _ => { /* todo warn */ }
                }
            }
        }

        doc
    }
}