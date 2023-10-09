use std::default::{Default,self};
use egui::Layout;
use egui::TextBuffer;
use serde_json;
use serde;
use egui::TextFormat;
use egui::Color32;
use egui::text::LayoutJob;

/// format the document for annotation and other things.
#[derive(Default,Debug)]
#[derive(serde::Deserialize, serde::Serialize,)]
#[serde(default)]// if we add new fields, give them default values when deserializing old state
pub struct DocLabeled {
    pub raw_text:String,
    highlights:Vec<(usize, // begin index
		    usize, // end index
		    (u8,u8,u8), // R,G,B
    )>,
    notes:Vec<(usize, // label begin index
	       usize, // label end index
	       String, //the content what we noted.
    )>,
    current_index:usize, // current index of person's reading.
    default_color:Color32,
}

impl DocLabeled{

    pub fn new(raw_text:String,highlights:Vec<(usize,usize,(u8,u8,u8))>,
	    notes:Vec<(usize,usize,String)>,current_index:usize,
	    default_color:Color32)->DocLabeled{
	DocLabeled{raw_text:raw_text,highlights:highlights,
	notes:notes,current_index:current_index,default_color:default_color}
    }

    pub fn update_highlight(&mut self,
			    bg_idx:usize,
			    end_idx:usize, // the range of cursor selection
			    colors:(u8,u8,u8)
    ){

	// 1. update the highlight list
	// A: whether it is in one cursor or not?
	let mut is_delight=false;
	let mut idx_found=0;
	let mut record_bg=0;
	let mut record_end=0;
	let mut record_color=(0,0,0);
	for (i,record) in self.highlights.iter().enumerate(){
	    if (record.0<=bg_idx) && (record.1>=end_idx){
		is_delight=true;
		idx_found=i;
		record_bg=record.0;
		record_end=record.1;
		record_color=record.2;
		// we think this situation will only exist for once.
		break;
	    }
	}
	if is_delight{
	    self.highlights.swap_remove(idx_found);
	    if record_bg!=bg_idx{
		self.highlights.push((record_bg, bg_idx, record_color));
	    };
	    if record_bg!=end_idx{
		self.highlights.push((end_idx,record_end,record_color));
	    }
	}
	else{
	    // scan overried regions
	    let mut overried_idxes=vec![];
	    for (i,record) in self.highlights.iter().enumerate(){
		if record.0>=bg_idx && record.1<= end_idx{
		    overried_idxes.push(i);
		}
	    }
	    // println!("bgidx:{},endidx:{} other: {:?}",bg_idx,end_idx,overried_idxes);
	    // delete such regions
	    let mut acc=0;
	    for x in overried_idxes{
		self.highlights.remove(x-acc);
		acc+=1;
	    }

	    // and merge cross regions finally.
	    for (i,record) in self.highlights.iter_mut().enumerate(){
		// for left part
		if bg_idx<= record.1 && bg_idx >= record.0{
		    *record=(record.0,bg_idx,record.2);
		}
		if end_idx<=record.1 && end_idx >= record.0{
		    *record=(end_idx,record.1,record.2);
		}
	    }

	    self.highlights.push((bg_idx,end_idx,colors));
	}

	// 2. sort the vectors 
             self.highlights.sort_by(|&v1, &v2| v1.0.cmp(&(v2.0)));
	println!("highlight: {:?}",self.highlights);
	    
    }
    
    
    pub fn update_notes(&mut self,
			bg_idx:usize,
			end_idx:usize,
			note:String
    ){
	self.notes.push((bg_idx,end_idx,note));
    }

    /// render the struct text into the egui style rich texts.
    pub fn rendering(&self)->LayoutJob{
	let light_color=Color32::WHITE;

	let mut job = LayoutJob::default();

	let mut bgn_idx=0;
	let end_idx=self.raw_text.len();

	println!("------------------------");
	println!("highlights: {:?}",self.highlights);
	if self.highlights.len()==0{
	    job.append(&self.raw_text.char_range(bgn_idx..end_idx),
			0.0,
			TextFormat {
			    color: self.default_color,
			    ..Default::default() 
			},
	    );
	}

	for record in self.highlights.iter(){
	    if bgn_idx!=record.0{
		job.append(&self.raw_text[bgn_idx..=record.0-1],
			   0.0,
			   TextFormat {
			       color: self.default_color,
			      ..Default::default() 
			   },
		);
	    }
	    job.append(&self.raw_text[record.0..=record.1-1],
		       0.0,
		       TextFormat {
			   color:light_color,
			   background: Color32::from_rgb(record.2.0,
						    record.2.1,
						    record.2.2,
			   ),
			   ..Default::default()
		       },
	    );
	    bgn_idx=record.1;
	}
	if bgn_idx!=end_idx{
	    job.append(&self.raw_text[bgn_idx..=end_idx-1],
		       0.0,
		       TextFormat {
			   color:self.default_color,
			   ..Default::default()
		       },
	    );
	}
	job
    }

    // pub fn renderingWidget(&self)->egui::Response{
	
    // }

}


pub fn main(){

    // test doclabeled.
    let mut docl= DocLabeled::new(
	"01234567890123456789".to_owned(),
    vec![],vec![],0,Color32::WHITE);
    docl.update_highlight(0,3,(255,0,0));
    docl.update_highlight(1,2,(255,0,0));
    docl.update_highlight(1,4,(255,0,0));
    docl.update_highlight(6,9,(255,0,0));
    docl.update_highlight(3,7,(255,0,0));
}