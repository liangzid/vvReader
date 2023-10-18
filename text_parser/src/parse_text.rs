use std::path::Path;
use std;
use regex;
use crate::format::{Heading,RawStructruedText};

pub fn parse_text(pth:&Path, use_newline:bool)->(Vec<Heading>,String){

    let mut headings=vec![];
    
    let content=std::fs::read_to_string(pth).unwrap();

    let kw_ls=vec!["<目录>",
		   "一、","二、","三、","四、","五、","六、","七、","八、","九、",
		   "1.","2.","3.","4.","5.","6.","7.","8.","9.",
		   "(1)","(2)","(3)","(4)","(5)","(6)","(7)","(8)","(9)",
		   // "A","B","C","D","E","F","G","H","I","J","K","L","M",
		   // "甲","乙","丙","丁","戊",
    ];

    let kw_bgls=vec!["甲","乙","丙","丁","戊",];

    let re1=regex::Regex::new(r"第(.*?)章").unwrap();
    let re2=regex::Regex::new(r"第(.*?)回").unwrap();
    let re3=regex::Regex::new(r"第(.*?)品").unwrap();
    
    // type1: contain the keywords
    let mut heads:Vec<&str>=vec![];
    let mut head_lines:Vec<usize>=vec![];
    let conts:Vec<&str>=str::split(&content,"\n").collect();
    let mut i=0;
    for (ii, c) in conts.iter().enumerate(){
	if ii==0 || ii==conts.len()-1{continue;};
	if ! (conts[ii-1]==""&& conts[ii+1]==""){continue;}

	if use_newline&&c.len()< 70{
	    heads.push((c).to_owned());
	    continue;}

	let mut iffound=false;
 	for kw in kw_ls.iter(){
	    if c.contains(kw) && (c.starts_with(kw) || c.ends_with(kw)){
		if c.len() > 50{continue;}
		if (&heads).contains(c){continue;}
		heads.push((c).to_owned());
		head_lines.push(i);
		iffound=true;
		break;
	    }
	}
	for kw in kw_bgls.iter(){
	    if c.contains(kw) && (c.starts_with(kw)){
		if c.len() > 50{continue;}
		if (&heads).contains(c){continue;}
		heads.push((c).to_owned());
		head_lines.push(i);
		iffound=true;
		break;
	    }
	}
	if !iffound{
	    // then we begain to match regexpression
	    if re1.is_match(c)|| re2.is_match(c)
	    || re3.is_match(c){
		if c.len() > 50{continue;}
		if (&heads).contains(c){continue;}
		heads.push((c).to_owned());
		head_lines.push(i);
		iffound=true;
	    }
	}
	i+=1;
    } // now we have the content of headings. the next step is: set the
    // postion of them.
    let mut past_p:usize=0;
    println!("heads: {:?}", heads.clone());
    for h in heads.iter(){
	let locate=find_substr_position(&content,
					h);
	past_p=locate;
	headings.push(Heading{name:(*h).to_owned(),
			      location:locate,level:1});
    }
    (headings,content)
}

fn find_substr_position(s:&str,sub:&str)->usize{
    if !s.contains(sub){return 0;}
    let start = s.find(sub).unwrap_or_default();

    // println!("{}", start); 
    start
}


fn main(){
    // only for test blablabla
    let pth=Path::new("/home/liangzi/code/048-思考中医.txt");
    let hs=parse_text(pth,true);
    println!("hs: {:?}",hs);
    // let pth=Path::new("/home/liangzi/code/003-新修本草.txt");
    // let hs=parse_text(pth,true);
    // println!("hs: {:?}",hs);
}
