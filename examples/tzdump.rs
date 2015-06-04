extern crate tz;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;


fn main() {
    for arg in env::args().skip(1) {
        match File::open(&Path::new(&arg)) {
            Ok(mut file) => {
                let mut contents = Vec::new();
                file.read_to_end(&mut contents).unwrap();
                match tz::parse(contents) {
                    Ok(tzdata) => tzdump(tzdata),
                    Err(e)     => println!("{}", e),
                }
            },
            Err(e) => println!("Couldn't open file {}: {}", arg, e),
        }
    }
}

fn tzdump(mut tz: tz::TZData) {
    tz.transitions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    for t in tz.transitions {
        let l = &*t.local_time_type;
        println!("{:11?}: name:{:5} offset:{:5} DST:{:5} type:{:?}",
                  t.timestamp, l.name, l.offset, l.is_dst, l.transition_type);
    }
}
