use std::ops::Range;
use std::default::{self, Default};

use egui::{Context, TextFormat, FontFamily};
use crate::app::documentFormat::DocLabeled;


pub fn open_one_reader(ctx: &Context,is_dark:bool,
		       lang:&String,
		       hlc:&mut (u8,u8,u8),
		       fontsz:&mut f32,
		       fname: &str, is_open: &mut bool,
		       docl: &mut DocLabeled,
		       is_open_highlight:&mut bool,
		       is_open_note:&mut bool,
) {
    egui::Window::new(fname)
        .default_open(true)
        .default_height(400.0)
        .default_width(600.0)
        .collapsible(true)
        // .constrain(true)
        .open(is_open)
        .show(ctx, |ui| {


	    // render all popup windows
	    for note in docl.notes.iter_mut(){
		let tt_ti=match lang.as_str(){
		    "zh"=>"添加评论",
		    _=>"New comment"
		};
		let pos=ctx.input(|i|
				  {i.pointer.hover_pos()})
		    .unwrap_or_default();
		egui::Window::new(tt_ti).open(&mut note.3)
		    .default_pos(pos)
		    .show(ctx, |ui|{
		    ui.text_edit_multiline(&mut note.2);
			// ui.horizontal_centered(|ui|{
			    // let tt_ti=match lang.as_str(){
			    // 	"zh"=>"确定",
			    // 	_=>"Done"
			    // };
			    // if ui.button(tt_ti).clicked(){
			    // 	note.3=false;
			    // }
			    // let tt_ti=match lang.as_str(){
			    // 	"zh"=>"取消",
			    // 	_=>"Cancel"
			    // };
			    // if ui.button(tt_ti).clicked(){
			    // 	note.3=false;
			    // 	note.2="".to_owned();
			    // }

			// });
		});
		
	    }

            egui::SidePanel::left("headline")
                .resizable(true)
                .default_width(200.0)
                .show_inside(ui, |ui| {
		    let tt_ti=match lang.as_str(){
			"zh"=>"目录",
			_=>"Headings"
		    };
                    ui.vertical_centered(|ui| {
                        ui.heading(tt_ti);
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label("111111");
                        ui.label("111111");
                        ui.label("111111");
                        ui.label("111111");
                    });
                });
            egui::CentralPanel::default()
                // .show(ui, |ui|{
                .show_inside(ui, |ui| {

		    // add menu bar here.
		    ui.horizontal(|ui|{
			let tt_ti=match lang.as_str(){
			    "zh"=>"高亮标记",
			    _=>"Highlight"
			};
			ui.checkbox(is_open_highlight, tt_ti);
			let tt_ti=match lang.as_str(){
			    "zh"=>"颜色",
			    _=>"Colors"
			};
			ui.menu_button(tt_ti,|ui|{
			
			    if is_dark{
				
			    ui.selectable_value(hlc, (71, 71, 135), "THEME I");
			    ui.selectable_value(hlc, (12, 36, 97), "THEME II");
			    ui.selectable_value(hlc, (183, 21, 64), "THEME III");
			    }
			    else{
			    ui.selectable_value(hlc, (246, 229, 141), "Bee Keeper");
			    ui.selectable_value(hlc, (224, 86, 253), "Heliotrope");
			    ui.selectable_value(hlc, (106, 176, 76), "Pure Apple");
			    }
			} );
			let tt_fz=match lang.as_str(){
			    "zh"=>"字体大小:",
			    _=>"Font Size:"
			};
			ui.label(tt_fz);
			ui.add(egui::Slider::new(fontsz,
				5.0..=20.0));

			let tt_nt=match lang.as_str(){
			    "zh"=>"标记评论",
			    _=>"Comment somethings"
			};
			ui.checkbox(is_open_note, tt_nt);
		    });
		    ui.separator();

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        render_selected_text(ctx, ui,lang,
					     docl,
					     is_open_highlight,
					     is_open_note,
					     hlc, fontsz,);
                    });
                });
	    
	    if (true).clone() {
            egui::SidePanel::right("notes")
                .resizable(true)
                .default_width(200.0)
                .show_inside(ui, |ui| {

		    let tt_ti=match lang.as_str(){
			"zh"=>"评论",
			_=>"Comments",
		    };
		    ui.vertical_centered(|ui| {
			ui.heading(tt_ti);
		    });

		    render_notes_side_show(ui,lang,docl);
                });
	    }
        });
}



pub fn render_selected_text(ctx: &Context, ui: &mut egui::Ui,
			    lang:&String,
			    docl: &mut DocLabeled,
			    is_open_highlight:&bool,
			    is_open_note:&bool,
			    hlc:&mut (u8,u8,u8), // highlight color
			    fontsz:&mut f32,
) {
    let mut layouter = |ui: &egui::Ui, easy_mark: &str, wrap_width: f32| {
        let mut job = docl.rendering(*fontsz);
        // println!("easy_mark: {}", easy_mark);
        // let mut job = LayoutJob::default();
        // job.append(easy_mark, 0.0,
        // 	   TextFormat{color: Color32::RED, ..Default::default()});
        job.wrap.max_width = wrap_width;
        ui.fonts(|f| f.layout_job(job))
    };

    let te = egui::TextEdit::multiline(&mut docl.raw_text.clone().as_str())
        .desired_width(f32::INFINITY)
        .layouter(&mut layouter)
        .show(ui);

    if let Some(cursor_range) = te.cursor_range {
        use egui::TextBuffer as _;
        let selected_chars = cursor_range.as_sorted_char_range();
        // println!(
        //     "cursor_range:{:?}\n char_range:{:?}",
        //     &cursor_range, selected_chars
        // );
        if selected_chars.start != selected_chars.end
	    && ctx.input(|i| i.pointer.any_released())
	{
	    if *is_open_highlight {
            docl.update_highlight(selected_chars.start, selected_chars.end, (*hlc).clone());
        }
	    if *is_open_note{
		docl.notes.push((selected_chars.start,
				 selected_chars.end,
				 "".to_owned(),
				 true,false,
		));
	    }
	}

	    
    };
}


pub fn render_notes_side_show(ui:&mut egui::Ui, lang:& String,
		    docl:&mut DocLabeled, ){
    
    egui::ScrollArea::vertical().show(ui,
|ui| {

    let mut if_del=false;
    let mut i=0;
    while i< docl.notes.len(){
	if docl.notes[i].0==docl.notes[i].1{
	    docl.notes.remove(i);
	    continue;
	}
	let note=&mut docl.notes[i];

	let sel_txt=docl.raw_text[note.0..((note.1)-1)].to_owned();
	let nt=note.2.clone();

	// now render the UI for it.

	let bg=egui::Color32::RED;
	let sc=egui::Color32::from_rgb(246, 229, 141);
	let fid=egui::FontId{size:8.5,
			     family:FontFamily::Monospace,
			     ..Default::default()};
	let mut job=egui::text::LayoutJob::default();
	job.append(
	    sel_txt.as_str(),
	    0.0,
	    egui::TextFormat{
		color:sc.clone(),
		background:bg.clone(),
		font_id:fid.clone(),
		..Default::default()
	    });
	ui.label(job);
	if note.4{
	    ui.text_edit_multiline(&mut note.2);
	}
	else{
	    ui.label(note.2.as_str());
	}

	ui.horizontal(|ui|{
	    let tt_ti=match lang.as_str(){
		"zh"=>"编辑",
		_=>"Edit",
	    };
	    if ui.button(tt_ti).clicked(){
		note.4=true;
	    }
	    let tt_ti=match lang.as_str(){
		"zh"=>"结束编辑",
		_=>"Edit Done",
	    };
	    if ui.button(tt_ti).clicked(){
		note.4=false;
	    }
	    let tt_ti=match lang.as_str(){
		"zh"=>"删除",
		_=>"Delete it",
	    };
	    if ui.button(tt_ti).clicked(){
		if if_del{
		    ui.add(egui::Spinner::new());
		}
		else{
		    *note=(0,0,"".to_owned(),false,false);
		    if_del=true;
		}
	    }
	});
	ui.separator();
	i+=1;
    }

    });
}
