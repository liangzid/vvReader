
use std::path::Path;
use std::default::{self, Default};
use std::{collections::HashMap, hash::Hash};
use serde;

mod parse_text;
use parse_text::{parse_text};

mod parse_epub;
use parse_epub::parse_epub;

// pub mod format;
use format::{Heading,RawStructruedText};


pub fn ExtractHeadline(pth:&Path)->RawStructruedText{
    let fname=pth.to_str().unwrap_or_default();
    let is_text=fname.ends_with(".txt");
    let mut res=RawStructruedText::default();
    if is_text{
	let temp=parse_text(pth, true);
	res.heads=temp.0;res.raw_text=temp.1;
	}
    else if fname.ends_with(".epub") {
	
    }
    



    res
}


fn main() {
    println!("Hello, world!");
}
