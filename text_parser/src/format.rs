use std::path::Path;
use std::default::{self, Default};
use std::{collections::HashMap, hash::Hash};
use serde;

#[derive(Default, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Heading{
    pub name:String,
    pub location:usize,
    pub level:u8,
}


#[derive(Default, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct RawStructruedText{
    pub heads:Vec<Heading>,
    pub raw_text:String,
}

impl RawStructruedText{
    pub fn transfer2tree(heads:&Vec<Heading>)->HashMap<(String,usize),
					 HashMap<(String,usize),
						 Vec<(String,usize)>>>{
	let mut tree:HashMap<(String,usize),HashMap<(String,usize),Vec<(String,usize)>>>=HashMap::new();
	let mut point=0;
	let mut clevel1:(String,usize)=("".to_owned(),0);
	let mut clevel2:(String,usize)=("".to_owned(),0);
	for (ih,h) in heads.into_iter().enumerate(){
	    if h.level==1{
		tree.insert((h.name.clone(),h.location), HashMap::new());
		clevel1=(h.name.clone(),h.location);
		point=1;
	    }
	    else if h.level==2{
		if point==0{
		    tree.insert((" ".to_owned(),h.location), HashMap::new());
		    clevel1=(" ".to_owned(),h.location);
		    point=1;
		}
		(*((tree.get_mut(&clevel1)).unwrap()))
		    .insert(
		    (h.name.clone(),h.location),vec![]);
		clevel2=(h.name.clone(),h.location);
		point=2;
	}
	else{ // all of others, as level 3.
	    if point==0{
		tree.insert((" ".to_owned(),h.location),
			    HashMap::new());
		clevel1=(" ".to_owned(),h.location);
		point=1;
	    }
	    if point==1{
		clevel2=(" ".to_owned(),h.location);
		(*tree.get_mut(&clevel1).unwrap())
		    .insert(
		    clevel2.clone(),vec![]);
		point=2;
	    }
	    (*(*tree.get_mut(&clevel1).unwrap())
	     .get_mut(&clevel2).unwrap())
		.push((h.name.clone(),h.location));
	}
    }
    tree
    }
}
