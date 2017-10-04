use std::env;
use std::path::Path;
use std::io::{self, Seek, BufRead, Write, SeekFrom};
use std::fs::File;

pub fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest = Path::new(&out_dir).join("lib.rs");
    let src = Path::new("resources/cmudict-0.7b");

    let succeed = expand_rules(&src, &dest);
    println!("{:#?}", succeed);
    assert!(succeed.is_ok());

}

fn expand_rules(src: &Path, dst: &Path) -> io::Result<()> {
    let rules = try!(File::open(src));
    let reader = io::BufReader::new(rules);
    let mut out = try!(File::create(dst));
    let lines = reader.lines();
    let mut num_lines = 0;

    try!(writeln!(out, r##"pub const CMUDICT: [&'static str; XXXXXX] = ["##));

    for line in lines.skip_while(|l| {
        match l {
            &Ok(ref line) => line.starts_with(";;;"),
            &Err(_) => panic!("Couldn't read line"),
        }
    }) {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                println!("Error reading line, {}", e);
                continue;
            }
        };
        if line.starts_with(";;;") {
            continue;
        }
        try!(writeln!(out, r##"  r#"{}"#,"##, line));
        num_lines += 1;
    }

    try!(writeln!(out, r##"];"##));
    try!(out.seek(SeekFrom::Start(0)));
    try!(writeln!(out,
                  r##"pub const CMUDICT: [&'static str; {:?}] = ["##,
                  num_lines));

    Ok(())
}
