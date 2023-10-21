use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(serde::Deserialize, serde::Serialize)]
struct Tag<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub content: Cow<'a, str>,
}

fn main() {
    println!("cargo:rerun-if-changed=tags.json");

    let json_tags = include_str!("tags.json");
    let tags = match serde_json::from_str::<Vec<Tag>>(json_tags) {
        Ok(tags) => tags,
        Err(err) => {
            eprintln!("tags.json is invalid: {}", err);
            std::process::exit(1)
        }
    };

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("tags.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    let mut map = phf_codegen::Map::new();
    for tag in tags {
        map.entry(
            tag.id,
            &format!(
                r###"crate::tag::Tag {{ title: r#"{}"#, content: r#"{}"#}}"###,
                tag.title, tag.content
            ),
        );
    }
    write!(
        &mut file,
        "pub(super) static TAGS: phf::Map<&'static str, super::Tag<'static>> = {}",
        map.build()
    )
    .unwrap();
    writeln!(&mut file, ";").unwrap();
}
