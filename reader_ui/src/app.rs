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
// mod md_editor_render;
mod account;

use crate::EasyMarkEditor;

use account::{render_login_windows,render_signup_windows};
use communicate::{activate, get_history, merge_records, push_record, query_login, signup};
use documentFormat::DocLabeled;
use donate::render_donate_win;
use password::{password,password_ui};
use text_selection_widget::{render_selected_text,open_one_reader};
use utils::code_view_ui;
// use md_editor_render::render_md_editor;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Heading {
    head_name: String,
    head_position: i32,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Clone)]
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
	    bool, // is open highlight or not
	    (u8,u8,u8), // highlight color
	    f32, // font size of reader
	    bool, //is open note function or not
        ),
    >,

    // md editor related
    md_states:Vec<(
	bool, // is UI open or not.
	String, // the save file path of current UI.
	EasyMarkEditor,
    )>,

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
	    //
	    // default as activated. Free for everyone!
            // activation_state: "not_activate".to_owned(),
            activation_state: "activate".to_owned(),
            utype_ls: vec![
                "nothing".to_owned(),
                "regular".to_owned(),
                "VIP1".to_owned(),
            ],
            activation_ls: vec!["not_activate".to_owned(), "activate".to_owned()],
            is_open_activate_help: false,

            reading_records: HashMap::new(),
	    md_states:vec![],
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
	    md_states,

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
	    .default_width(100.0).show(ctx, |ui|{
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
		    ui.radio_value(is_dark_theme, false, "☀").clicked();
		    ui.radio_value(is_dark_theme, true, "🌙").clicked();
                });
            let tt_pay=match lang.as_str(){
		    "zh"=>"赞助本网站", _=>"Donate"
		    };
		ui.vertical_centered(|ui|{
		    if ui.button(tt_pay).clicked(){
		        *is_open_payment_qr=true;
		    }
		});

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
                    ui.label("数据导出/导入：✔");
                    ui.label("跨设备云端存储：✖");
                    ui.label("AI检索：✔");
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
                    ui.label("Records Import/Export：✔");
                    ui.label("Cloud Storage and sync：✖");
                    ui.label("AI-based Retrieval：✔");
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
			"zh"=>"导入",
			_=>"import past records"
		    };      #[cfg(not(target_arch = "wasm32"))]
                    if ui.button(tt_export).clicked() {
                        if activation_state=="not_activate"{
                            *is_open_activate_help=true;
                        }
                        else{
                            if let Some(pth) = rfd::FileDialog::new().pick_file() {
                                let res =std::fs::read_to_string(pth.clone()).unwrap();
				let tmp:(HashMap<String,(bool,DocLabeled,bool,(u8,u8,u8),f32,bool)>,Vec<(bool,String,EasyMarkEditor)>)=serde_json::from_str(&res).unwrap();
				*reading_records=tmp.0;
				*md_states=tmp.1;
                            }
                        }
                    }
                    
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
                            if let Some(pth) = rfd::FileDialog::new().save_file() {
                                let res = serde_json::to_string(&((*reading_records).clone(),
			(*md_states).clone())).unwrap();
                                let _=std::fs::write(pth, res);
                            }
                        }
                    }

                    egui::Window::new("Notion").open(is_open_activate_help)
                    .show(ctx, |ui| {
                        ui.label("Sorry, you have no permission to do this operation!");
                        ui.hyperlink_to("Update your account now!", "https://liangzid.github.io/");
                    });

	    });

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
			*reading_records = HashMap::new();
			*md_states = vec![];
                    }

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
			    if ui.button(tt_exports).clicked(){
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

                ui.separator();

		let tt_loadlocalf=match lang.as_str(){
		    "zh"=>"阅读本地文件",
		    _=>"Read a file from local",
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
				(*is_dark_theme).clone(),
			    );			    
			    let newpth=uploadpath.clone();
			    let fnme=newpth
				.to_str().unwrap();
			    reading_records.insert(String::from(fnme),
						   (true,doc,false,
						    (255,0,0),12.0,
						    false,
			    ));
			}
		}

		// markdown editor new file
		let ttmd=match lang.as_str(){
		    "zh"=>"记笔记",
		    _=>"Take notes",
		};
		if ui.button(ttmd).clicked(){
		    md_states.push((true,"Undefined".to_owned(),
				    EasyMarkEditor::default(),
		    ));
		}

		// markdown editor readfile
		let ttmd=match lang.as_str(){
		    "zh"=>"加载过去的笔记",
		    _=>"Load past notes",
		};
		if ui.button(ttmd).clicked(){
		    if let Some(rfl)=
			rfd::FileDialog::new().pick_file(){

			    // first obtain the filename. 
			    let tmp_fname=rfl.clone().to_str().unwrap().to_owned();

			    for md in md_states.iter_mut(){
				if md.1==tmp_fname {
				    if md.0==false{
					md.0=true;
					let ct=std::fs::read_to_string(rfl.clone()).unwrap();
					md.2.code=ct;
					break;
				    }
				}
			    }

		md_states.push((true,"Undefined".to_owned(),
			EasyMarkEditor::default()));

			    // let ct=std::fs::
			    // read_to_string(uploadpath.clone())
				// .unwrap();

			    
			}
		}
		
                });

	
            egui::CentralPanel::default().show(ctx, |ui| {


		//rendering all reading windows
		for rec in reading_records.iter_mut(){
		    rec.1.1.default_color=default_color.clone();
		    open_one_reader(&ctx,(*is_dark_theme).clone(),
				    lang,
				    &mut rec.1.3,
				    &mut rec.1.4,
				    rec.0.as_str(),
				    &mut rec.1.0,
				    &mut rec.1.1,
				    &mut rec.1.2,
				    &mut rec.1.5
		    );
		}


		//
		// render all md editor windows
		for md in md_states.iter_mut(){
			egui::Window::new(md.1.as_str()).
			    open(&mut md.0)
			    .show(ctx, |ui|{
				md.2.lang=lang.clone();
				md.2.fname=md.1.clone();
				md.2.ui(ui, &mut md.1);
			    });
		}

            });
        

    }
}


