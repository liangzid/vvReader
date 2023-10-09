use egui::Context;
use crate::app::documentFormat::DocLabeled;


pub fn open_one_reader(ctx: &Context, lang:&String,
		       hlc:&mut (u8,u8,u8),
		       fontsz:&mut f32,
		       fname: &str, is_open: &mut bool,
		       docl: &mut DocLabeled,
		       is_open_highlight:&mut bool) {
    egui::Window::new(fname)
        .default_open(true)
        .default_height(400.0)
        .default_width(600.0)
        .collapsible(true)
        .constrain(true)
        .open(is_open)
        .show(ctx, |ui| {
            egui::SidePanel::left("headline")
                .resizable(true)
                .default_width(200.0)
                .show_inside(ui, |ui| {
                    // ui.heading("Headline of Books");
                    ui.vertical_centered(|ui| {
                        ui.heading("Headline of Books");
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
			
			    ui.selectable_value(hlc, (255 as u8,0 as u8,0 as u8), "Red");
			    ui.selectable_value(hlc, (0,255,0), "Green");
			    ui.selectable_value(hlc, (0,0,255), "Blue");
			} );
			let tt_fz=match lang.as_str(){
			    "zh"=>"字体大小:",
			    _=>"Font Size:"
			};
			ui.label(tt_fz);
			ui.add(egui::Slider::new(fontsz,
				5.0..=20.0));
		    });

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        render_selected_text(ctx, ui, docl,
			is_open_highlight, hlc, fontsz,);
                    });
                });
            egui::SidePanel::right("notes")
                .resizable(true)
                .default_width(200.0)
                .show_inside(ui, |ui| {
                    ui.heading("Notes of Books");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label("111111");
                        ui.label("222222");
                        ui.label("111111");
                        ui.label("111111");
                    });
                });
        });
}



pub fn render_selected_text(ctx: &Context, ui: &mut egui::Ui,
			    docl: &mut DocLabeled,
			    is_open_highlight:&bool,
			    hlc:&mut (u8,u8,u8), // highlight color
			    fontsz:&mut f32,
) {
    let mut layouter = |ui: &egui::Ui, easy_mark: &str, wrap_width: f32| {
        let mut job = docl.rendering(fontsz);
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
        println!(
            "cursor_range:{:?}\n char_range:{:?}",
            &cursor_range, selected_chars
        );
        if selected_chars.start != selected_chars.end
	    && ctx.input(|i| i.pointer.any_released())
	    && *is_open_highlight {
            docl.update_highlight(selected_chars.start, selected_chars.end, (255, 0, 0));
        }
    };
}


