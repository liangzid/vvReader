use text_parser::format::{Heading,RawStructruedText};
use std::path::Path;
use docx_rust::DocxFile;


pub fn parse_docx(pth:&Path)->RawStructruedText{
    let mut res=RawStructruedText::default();

    let mut overall_content="".to_owned();
    let mut heads:Vec<Heading>=vec![];

    let docx = DocxFile::from_file(pth.to_str().unwrap()).unwrap();
    let mut docx = docx.parse().unwrap();
    println!("file content: {:?}",docx);
    res
}


pub fn main(){
    let pth=Path::new("/mnt/d/dode/nsh.docx");
    let res=parse_docx(pth);
}
