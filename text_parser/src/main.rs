
use std::path::Path;
use std::default::{self, Default};
use std::{collections::HashMap, hash::Hash};
use serde;

mod parse_text;
use parse_text::{parse_text};

mod parse_epub;
use parse_epub::parse_epub;

mod parse_docx;
use parse_docx::parse_docx;

mod parse_md;
use parse_md::{parse_md,parse_org};

// pub mod format;
use text_parser::format::{Heading,RawStructruedText};


pub fn ExtractHeadline(pth:&Path)->RawStructruedText{
    let fname=pth.to_str().unwrap_or_default();
    let is_text=fname.ends_with(".txt");
    let mut res=RawStructruedText::default();
    if is_text{
	let temp=parse_text(pth, true);
	res.heads=temp.0;res.raw_text=temp.1;
	}
    else if fname.ends_with(".epub") {
	res=parse_epub(pth);
    }
    else if fname.ends_with(".md"){
	res=parse_md(pth);
    }
    else if fname.ends_with(".org"){
	res=parse_md(pth);
    }

    res
}


fn main() {
    println!("Hello, world!");
}
