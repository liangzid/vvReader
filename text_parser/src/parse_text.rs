use std::path::Path;
use std;
use std::default::{self, Default};
use std::{collections::HashMap, hash::Hash};
use serde;
use regex;
use serde_json;

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Heading{
    name:String,
    location:usize,
    level:u8,
}

pub fn parse_text(pth:&Path)->Vec<Heading>{

    let mut headings=vec![];
    
    let content=std::fs::read_to_string(pth).unwrap();

    let kw_ls=vec!["<目录>",
		   "一、","二、","三、","四、","五、","六","七","八","九",
		   "1.","2.","3.","4.","5.","6.","7.","8.","9.",
		   "(1)","(2)","(3)","(4)","(5)","(6)","(7)","(8)","(9)",
		   "A","B","C","D","E","F","G","H","I","J","K","L","M",
		   "甲","乙","丙","丁","戊",
    ];

    let chapter_re=regex::Regex::new(r".第(.*?)章(.*?)").unwrap();
    
    // type1: contain the keywords
    let mut heads=vec![];
    let mut head_lines=vec![];
    let conts:Vec<&str>=str::split(&content,"\n").collect();
    let mut i=0;
    for c in conts.iter(){
	let mut iffound=false;
	for kw in kw_ls.iter(){
	    if c.starts_with(kw) || c.ends_with(kw){
		heads.push((*c).to_owned());
		head_lines.push(i);
		iffound=true;
		break;
	    }
	}
	if !iffound{
	    // then we begain to match regexpression
	    if chapter_re.is_match(c){
		heads.push((*c).to_owned());
		head_lines.push(i);
		iffound=true;
	    }
	    
	}
	i+=1;
    } // now we have the content of headings. the next step is: set the
    // postion of them.
    for (i,h) in heads.iter().enumerate(){
	let locate=find_substr_position(&content, h);
	headings.push(Heading{name:h.to_owned(),
			      location:locate,level:1});
    }
    headings

    
}

fn find_substr_position(s:&str,sub:&str)->usize{
    let char_indices = s.chars().collect::<Vec<_>>();

    let start = char_indices.windows(sub.len())
	.position(|window| window == sub.chars()
		    .collect::<Vec<_>>())
		    .unwrap();

    println!("{}", start); 
    start
}
