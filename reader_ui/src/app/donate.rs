use egui;
use std::default::{self, Default};
use std::{collections::HashMap, hash::Hash};

use chrono::{DateTime, Local};
use egui::text::LayoutJob;
use egui::Context;
use egui::{
    emath::align, util::History, Color32, FontData, FontDefinitions, FontFamily, TextFormat,
};
use env_logger::fmt::Color;

use egui_extras::{Size, StripBuilder};
use rfd;
use serde;
use serde_json;

pub fn render_donate_win(ctx: &egui::Context, is_open_payment_qr: &mut bool, lang: &String) {
    egui::Window::new("").default_width(280.0)
                    .open(is_open_payment_qr)
                    .show(ctx, |ui| {
			let tt_donate=match lang.as_str(){
			    "zh"=>"如果你喜欢这个项目，可以请作者喝杯奶茶，哈哈哈！开支：\n Server cost: 750 CHY /year\n Domain name cost: 88  CHY /year",
			    _=>"Donate to support this site, and me!\n Server cost: 750 CHY /year\n Domain name cost: 88  CHY /year"
			};
			ui.label(tt_donate);
            let wechat_img=include_bytes!("../../data/wechat_qr.jpg");
            use image::GenericImageView;
            let image=image::load_from_memory(wechat_img).
            expect("failed load img");
            let img_buffer=image.to_rgba8();
            let size = (image.width() as usize, image.height() as usize);
            let pixels = img_buffer.into_vec();
            let size=[size.0,size.1];
            let x=egui::ColorImage::from_rgba_premultiplied(size,&pixels);

			let wechat_texture: egui::TextureHandle =
				ui.ctx().load_texture(
				    "../../data/wechat_qr.jpg",
				    x,
				    Default::default()
				);
            let mut size=wechat_texture.size_vec2();
            size.x=size.x/4.0;
            size.y=size.y/4.0;
			ui.image(&wechat_texture,size);

            let wechat_img=include_bytes!("../../data/alipay_qr.jpg");
            let image=image::load_from_memory(wechat_img).
            expect("failed load img");
            let img_buffer=image.to_rgba8();
            let size = (image.width() as usize, image.height() as usize);
            let pixels = img_buffer.into_vec();
            let size=[size.0,size.1];
            let x=egui::ColorImage::from_rgba_premultiplied(size,&pixels);

			let wechat_texture: egui::TextureHandle =
				ui.ctx().load_texture(
				    "../../data/wechat_qr.jpg",
				    x,
				    Default::default()
				);
            let mut size=wechat_texture.size_vec2();
            size.x=size.x/4.0;
            size.y=size.y/4.0;
			ui.image(&wechat_texture,size);
                    });
}
