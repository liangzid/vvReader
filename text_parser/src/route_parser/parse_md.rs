use std::path::Path;
use std;
use regex;
use crate::format::{Heading,RawStructruedText};

pub fn parse_md(pth:&Path)->RawStructruedText{
    parse_mdorg(pth, "#".to_owned())
}

pub fn parse_org(pth:&Path)->RawStructruedText{
    parse_mdorg(pth, "*".to_owned())
}


pub fn parse_mdorg(pth:&Path, note:String)->RawStructruedText{
    let mut res=RawStructruedText::default();
    
    let mut heads:Vec<Heading>=vec![];

    let content=std::fs::read_to_string(pth).unwrap();
    let conts:Vec<&str>=str::split(&content,"\n").collect();

    for c in conts.iter(){
	let mut level=0;
	if (*c).starts_with(&note){
	    if c.starts_with(&(note.clone()+" ")){level=1;}
	    else if c.starts_with(&(note.clone()+&note+" ")) {level=2;}
	    else if c.starts_with(&(note.clone()+&note+&note
				    +" ")) {level=3;}
	    else if c.starts_with(&(note.clone()+&note+&note+&note
				    +" ")) {level=4;}

	    let locate=content.find(c).unwrap_or_default();

	    heads.push(Heading { name: (**c).to_owned(),
				 location: locate,
				 level: level });
	}
    }
    res.heads=heads;
    res.raw_text=content.clone();

    res
    
}







