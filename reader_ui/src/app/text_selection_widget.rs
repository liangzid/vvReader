use egui::Context;
use crate::app::documentFormat::DocLabeled;


pub fn open_one_reader(ctx: &Context, fname: &str, is_open: &mut bool, docl: &mut DocLabeled) {
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
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        render_selected_text(ctx, ui, docl);
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



pub fn render_selected_text(ctx: &Context, ui: &mut egui::Ui, docl: &mut DocLabeled) {
    let mut layouter = |ui: &egui::Ui, easy_mark: &str, wrap_width: f32| {
        let mut job = docl.rendering();
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
        if selected_chars.start != selected_chars.end && ctx.input(|i| i.pointer.any_released()) {
            docl.update_highlight(selected_chars.start, selected_chars.end, (255, 0, 0));
        }
    };
}


