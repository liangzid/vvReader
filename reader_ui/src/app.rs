use std::default::{self, Default};
use std::{collections::HashMap, hash::Hash};

use chrono::{DateTime, Local};
use egui::Context;
use egui::{
    emath::align, util::History, Color32, FontData, FontDefinitions, FontFamily, TextFormat,
};
use rfd;
use serde;
use serde_json;

mod communicate;
mod documentFormat;
mod donate;
mod password;
mod text_selection_widget;
mod utils;
mod account;
use account::{render_login_windows,render_signup_windows};
use communicate::{activate, get_history, merge_records, push_record, query_login, signup};
use documentFormat::DocLabeled;
use donate::render_donate_win;
use password::{password,password_ui};
use text_selection_widget::{render_selected_text,open_one_reader};
use utils::code_view_ui;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Heading {
    head_name: String,
    head_position: i32,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // settings, meta-information
    lang: String,
    is_dark_theme: bool,

    // account related
    email: String,
    pwd: String,
    pwd2: String,
    login_state: i8,
    user_type: String,
    activation_state: String,
    utype_ls: Vec<String>,
    activation_ls: Vec<String>,
    is_open_activate_help: bool,

    // Reader related
    reading_records: HashMap<
        String, // real file name
        (
            bool, // its window is open or not.
            // warning: This might be large!
            DocLabeled, // document related informaiton
        ),
    >,

    // contents of user inputs.
    current_fname: String,
    is_open_export: bool,
    is_open_import: bool,
    is_open_login: bool,
    is_open_signup: bool,
    is_open_payment_qr: bool,

    default_color: Color32,
    strong_color: Color32,

    // Example stuff:
    label: String,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            lang: "zh".to_owned(),
            is_dark_theme: false,
            email: "".to_owned(),
            pwd: "".to_owned(),
            pwd2: "".to_owned(),
            login_state: 0,
            user_type: "nothing".to_owned(),
            activation_state: "not_activate".to_owned(),
            utype_ls: vec![
                "nothing".to_owned(),
                "regular".to_owned(),
                "VIP1".to_owned(),
            ],
            activation_ls: vec!["not_activate".to_owned(), "activate".to_owned()],
            is_open_activate_help: false,

            reading_records: HashMap::new(),
            current_fname: "".to_owned(),

            is_open_export: false,
            is_open_import: false,
            is_open_login: true,
            is_open_signup: false,
            is_open_payment_qr: false,

            default_color: Color32::LIGHT_GRAY,
            strong_color: Color32::WHITE,

            label: "example".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        // load CJK fonts.
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "wenquan".to_owned(),
            FontData::from_static(include_bytes!("../data/wenquan.ttf")),
        );

        // set priority
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "wenquan".to_owned());

        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("wenquan".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // close the state storage
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            lang,
            is_dark_theme,

            email,
            pwd,
            pwd2,
            login_state,
            user_type,
            activation_state,
            utype_ls,
            activation_ls,
            is_open_activate_help,
            reading_records,

            current_fname,
            is_open_export,
            is_open_import,
            is_open_login,
            is_open_signup,
            is_open_payment_qr,

            default_color,
            strong_color,

            label,
            value,}=self;

	render_login_windows(ctx, lang, is_open_login, is_open_signup,
			     email, pwd, login_state, user_type,
			     activation_state);
	render_signup_windows(ctx, lang, is_open_login, is_open_signup,
			      email, pwd, pwd2, login_state, user_type,
			      activation_state);

	render_donate_win(ctx, is_open_payment_qr,lang);

	egui::SidePanel::left("options").resizable(true)
	    .default_width(120.0).show(ctx, |ui|{
		// set theme
                let mut color_blue: Color32;
                if *is_dark_theme {
                    ctx.set_visuals(egui::Visuals::dark());
                    color_blue = Color32::from_rgb(255, 255, 1);
                } else {
                    ctx.set_visuals(egui::Visuals::light());
                    color_blue = Color32::from_rgb(33, 24, 68);
                }
                (*default_color, *strong_color) = if
		    ui.visuals().dark_mode {
                    (Color32::LIGHT_GRAY, Color32::WHITE)
                } else {
                    (Color32::DARK_GRAY, Color32::BLACK)
                };

                ui.horizontal(|ui| {
		    match lang.as_str(){"zh"=>{
			ui.label("主题");
		    }
			       _=>{ui.label("Theme:");}
		    }
		    ui.radio_value(is_dark_theme, false, "☀️").clicked();
		    ui.radio_value(is_dark_theme, true, "☪").clicked();
                });
            let tt_pay=match lang.as_str(){
		    "zh"=>"赞助本网站", _=>"Donate"
		    };
		    if ui.button(tt_pay).clicked(){
		        *is_open_payment_qr=true;
		    }

                let mut track_lang = true;
                // let mut lang.as_str() = "zh".to_owned();
                ui.horizontal(|ui| {
		    match lang.as_str() {"zh"=>ui.label("语言"),_=>ui.label("Language"),};
                    track_lang |= ui.radio_value(lang, "zh".to_owned(), "中文").clicked();
                    track_lang |= ui
                        .radio_value(lang, "en".to_owned(), "English")
                        .clicked();
                });

                ui.separator();
                // add the export and import button.
                ui.horizontal(|ui|{
		    match lang.as_str() {"zh"=>ui.label("当前状态："),_=>ui.label("Current State"),};
		    if (*login_state).eq(&0){
			match lang.as_str() {"zh"=>ui.label("未登录"),
				    _=>ui.label("Visitor, not logged in"),}
		    }
		    else{
			match lang.as_str() {"zh"=>ui.label(format!("账户 {} 登录.",email)),
				    _=>ui.label(format!("User {} logged in.",email)),}
		    }
                });

		match lang.as_str(){
		    "zh" =>{

                ui.label("阅读：✔");
                ui.label("高亮：✔");
                ui.label("批注：✔");
                ui.label("数据于当前设备缓存：✔");
                if activation_state=="not_activate"{
                    ui.label("数据导出/导入：✖");
                    ui.label("跨设备云端存储：✖");
                    ui.label("AI检索：✖");
                }
                else{
                    ui.label("数据导出/导入：✔");
                    ui.label("跨设备云端存储：✔");
                    ui.label("AI检索：马上推出");
                }
		    }

		    _=>{

                ui.label("Reading：✔");
                ui.label("Highlight：✔");
                ui.label("Comment：✔");
                ui.label("Store history in local deivce：✔");
                if activation_state=="not_activate"{
                    ui.label("Records Import/Export：✖");
                    ui.label("Cloud Storage and sync：✖");
                    ui.label("AI-based Retrieval：✖");
                }
                else{
                    ui.label("Records Import/Export：✔");
                    ui.label("Cloud Storage and sync：✔");
                    ui.label("AI-based Retrieval：comming soon");
                }
		    }

		}


                ui.horizontal(|ui|{
		    match lang.as_str(){
			"zh"=>{

                    if ui.button("登录").clicked(){
                        *is_open_login=true;
                    }
                    if ui.button("注册").clicked(){
                        *is_open_signup=true;
                    }
		    if ui.button("注销").clicked(){
			*login_state=0;
			*user_type="nothing".to_owned();
			*activation_state="not_activate".to_owned();
		    }

			}
			_=>{
                    if ui.button("Log in").clicked(){
                        *is_open_login=true;
                    }
                    if ui.button("Sign up").clicked(){
                        *is_open_signup=true;
                    }
		    if ui.button("Quit account").clicked(){
			*login_state=0;
			*user_type="nothing".to_owned();
			*activation_state="not_activate".to_owned();
		    }

			}
		    }
                });
                ui.horizontal(|ui| {
                    
		    #[cfg(not(target_arch = "wasm32"))]
		    let tt_export=match lang.as_str(){
			"zh"=>"导出",
			_=>"export"
		    };      #[cfg(not(target_arch = "wasm32"))]
                    if ui.button(tt_export).clicked() {
                        if activation_state=="not_activate"{
                            *is_open_activate_help=true;
                        }
                        else{
                            if let Some(path) = rfd::FileDialog::new().save_file() {
				// todo: export.
                                // let res = serde_json::to_string(historys).unwrap();
                                // std::fs::write(path, res);
                            }
                        }
                    }
                    egui::Window::new("Notion").open(is_open_activate_help)
                    .show(ctx, |ui| {
                        ui.label("Sorry, you have no permission to do this operation!");
                        ui.hyperlink_to("Update your account now!", "https://liangzid.github.io/");
                    });

	    });

		    let tt_imports=match lang.as_str(){
			"zh"=>"文本方式导入",
			_=>"import from string"
		    };
                    if ui.button(tt_imports).clicked() {
                        if activation_state=="not_activate"{
                            *is_open_activate_help=true;    
                        }
                        else{
                            *is_open_import=true;
                        }
                    }
		    let tt_exports=match lang.as_str(){
			"zh"=>"导出为可复制的文本",
			_=>"export as string"
		    };
                    if ui.button(tt_exports).clicked() {
                        if activation_state=="not_activate"{
                            *is_open_activate_help=true;    
                        }
                        else{
			                *is_open_export=true;
                        }
                    }
		    let tt_clear=match lang.as_str(){
			"zh"=>"清空",
			_=>"clear"
		    };
                    if ui.button(tt_clear).clicked() {
			// todo: clear
                        // *historys = vec![];
                        // *comments = vec![];
			// *place = "".to_owned();
			// *analyse = "".to_owned();
			// *temp_comment = "".to_owned();
			// *pop_open = false;
			// *current_point = 0;
			// *is_open_import = false;
			// *is_open_export = false;
			// *is_visual=false;
			
                    }


                ui.separator();

		let tt_done=match lang.as_str(){"zh"=>"毕",_=>"Done."};
		let tt_cp=match lang.as_str(){"zh"=>"复制之",_=>"Copy it."};
        let tt_imports=match lang.as_str(){
			"zh"=>"文本方式导入",
			_=>"import from string"
		    };
		egui::Window::new(tt_imports).default_width(300.0)
		    .open(is_open_import)
		    .show(ctx,|ui|{
			let mut read_text:String="".to_owned();
			ui.text_edit_multiline(&mut read_text);
			if ui.button(tt_done).clicked(){
			    // *historys = serde_json::from_str(&read_text).unwrap();
			    // todo: import with text
			}
		    });
            let tt_exports=match lang.as_str(){
                "zh"=>"导出为可复制的文本",
                _=>"export as string"
                };
		egui::Window::new(tt_exports).default_width(300.0)
		    .open(is_open_export)
		    .show(ctx,|ui|{

			let scroll = egui::ScrollArea::vertical()
			    .max_height(400.0)
			    .auto_shrink([false;2])
			    .show(ui, |ui| {
			
				let res="".to_owned();
			ui.vertical(|ui|{
			    let mut is_copyed=false;
			    if ui.button(tt_cp).clicked(){
				is_copyed=true;
				use clipboard::{ClipboardContext,ClipboardProvider};
				let mut ctx:ClipboardContext = ClipboardProvider::new().unwrap();
				// let res = serde_json::to_string(historys).unwrap();
				let res="".to_owned();
				// ctx.set_contents(res).unwrap();
				ui.output_mut(|o| o.copied_text = res.to_string());
			    }
			// ui.label(res);
			    code_view_ui(ui,&res);
			})
			    });
		    });

		    let tt_import=match lang.as_str(){
			"zh"=>"导入",
			_=>"import"
		    };      #[cfg(not(target_arch = "wasm32"))]
                    if ui.button(tt_import).clicked() {
                        if activation_state=="not_activate"{
                            *is_open_activate_help=true;    
                        }
                        else{
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                // let content = std::fs::read_to_string(path).unwrap();
                                // *historys = serde_json::from_str(&content).unwrap();
				// todo! import
                            }
                        }
                    }
		
                });

	
            egui::CentralPanel::default().show(ctx, |ui| {

		let tt_loadlocalf=match lang.as_str(){
		    "zh"=>"上传本地文件",
		    _=>"Upload file from your device",
		};
		if ui.button(tt_loadlocalf).clicked(){
		    // open a file picker and then load the files.
		    if let Some(uploadpath) = rfd::FileDialog::new()
			.pick_file(){
			    let ct=std::fs::
			    read_to_string(uploadpath.clone())
				.unwrap();
			    if ! uploadpath.clone().ends_with(".txt"){
		    let tt_file_us=match lang.as_str(){
			"zh"=>"不支持的文件类型",
			_=>"Unsupported file types"
		    };
				
				ui.colored_label(egui::Color32::RED,
						 tt_file_us,
				);
				
			    }

			    // println!("ct: {}", &ct);

			    let mut doc=DocLabeled::new(
				ct,vec![],
				vec![],0,
				(*default_color).clone(),
			    );			    
			    let newpth=uploadpath.clone();
			    let fnme=newpth
				.to_str().unwrap();
   reading_records.insert(String::from(fnme),(true,doc));
			    let temp_record=(*reading_records).get_mut(fnme).unwrap();
			}
		}

		//rendering all reading windows
		for rec in reading_records.iter_mut(){
		    open_one_reader(&ctx, rec.0.as_str(),
				    &mut rec.1.0,
				    &mut rec.1.1);
		}
            });
        

    }
}


