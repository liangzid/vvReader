use text_parser::format::{Heading,RawStructruedText};
use std::path::Path;
use epubparse::epub_to_book;

pub fn parse_epub(pth:&Path)->RawStructruedText{
    let mut res=RawStructruedText::default();

    let bytes=std::fs::read(pth).unwrap();
    let book=epub_to_book(&bytes).unwrap();

    // now parse the book into the Heading style!
    let mut overall_content="".to_owned();
    let mut heads:Vec<Heading>=vec![];

    for x in book.chapters{
	let mut i=1;
	heads.push(Heading{name:x.title.clone(),
			   location:overall_content.len(),
			   level:i as u8
	});
	overall_content=overall_content+&x.text;
	if x.subchapters.len()==0{continue;}
	else{
	    i+=1;
	    for xx in x.subchapters.iter(){
		heads.push(Heading{name:xx.title.clone(),
			   location:overall_content.len(),
			   level:i as u8
		});
		overall_content=overall_content+&xx.text;
		if xx.subchapters.len()==0{continue;}
		else{
		    i+=1;
		    for xxx in xx.subchapters.iter(){
			heads.push(Heading{name:xxx.title.clone(),
					   location:overall_content.len(),
					   level:i as u8
			});
			overall_content=overall_content+&xxx.text;
			if xxx.subchapters.len()==0{continue;}
			else{
			    i+=1;
			    for xxxx in xxx.subchapters.iter(){
				heads.push(Heading{name:xxx.title.clone(),
					   location:overall_content.len(),
					   level:i as u8
				});
				overall_content=overall_content+&xxx.text;
				if xxx.subchapters.len()==0{continue;}
				else{
				    println!("Warnning!")
				}
			    }
			}

		    }
		}

	    }
	}
	    
	}
    RawStructruedText { heads: heads,
			raw_text: overall_content.clone() }
    }




pub fn main(){
    let pth=Path::new("/home/liangzi/code/xdd悉达多.epub");
    let hs=parse_epub(pth,);
    println!("hs: {:?}",hs);
    
}
