use std::{collections::HashMap, hash::Hash};
use std::default::{Default, self};


use chrono::{DateTime, Local};
use egui::Context;
use egui::{
    emath::align, util::History, Color32, FontData, FontDefinitions, FontFamily, TextFormat,
};
use env_logger::fmt::Color;

use rfd;
use serde_json;
use serde;
use egui_extras::{Size,StripBuilder};

mod communicate;
use communicate::{query_login,get_history,
		  push_record,merge_records,
		  signup,activate
};
mod documentFormat;
use documentFormat::DocLabeled;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default,Debug)]
#[derive(serde::Deserialize, serde::Serialize,)]
#[serde(default)]// if we add new fields, give them default values when deserializing old state
pub struct Heading{
    head_name: String,
    head_position: i32,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]// if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // settings, meta-information
    lang:String,
    is_dark_theme: bool,


    // account related
    email:String,
    pwd:String,
    pwd2:String,
    login_state:i8,
    user_type:String,
    activation_state:String,
    utype_ls:Vec<String>,
    activation_ls:Vec<String>,
    is_open_activate_help:bool,

    // Reader related
    reading_records: HashMap<String, // real file name
    (
	bool, // its window is open or not.
	    // warning: This might be large!
	    DocLabeled, // document related informaiton
    )
        >,

    // contents of user inputs.
    current_fname:String,
    is_open_export:bool,
    is_open_import:bool,
    is_open_login:bool,
    is_open_signup:bool,
    is_open_payment_qr:bool,

    default_color:Color32,
    strong_color:Color32,

    // Example stuff:
    label: String,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
	    lang:"zh".to_owned(),
	    is_dark_theme:false,
	    email:"".to_owned(),
	    pwd:"".to_owned(),
	    pwd2:"".to_owned(),
	    login_state:0,
	    user_type:"nothing".to_owned(),
	    activation_state:"not_activate".to_owned(),
	    utype_ls:vec!["nothing".to_owned(),"regular".to_owned(),
			  "VIP1".to_owned()],
	    activation_ls:vec!["not_activate".to_owned(),
			       "activate".to_owned(),
	    ],
            is_open_activate_help:false,

	    reading_records: HashMap::new(),
	    current_fname:"".to_owned(),

	    is_open_export:false,
	    is_open_import:false,
            is_open_login:true,
            is_open_signup:false,
            is_open_payment_qr:false,

	    default_color:Color32::LIGHT_GRAY,
	    strong_color:Color32::WHITE,

	    label:"example".to_owned(),
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
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

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
	    pwd,pwd2,
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
	    value

        } = self;


	// here we test the reading window.



	

        let now = Local::now().format("%F-%T").to_string();
        if true {
	    let tt_login= match lang.as_str(){
		"zh"=>"登录，以同步您的私有信息",
		_=>"Login to sync your information!",
	    };
            egui::Window::new(tt_login).default_width(300.0)
		    .open(is_open_login)
		    .show(ctx,|ui|{
                
                // ui.heading("Login to your account!");
                ui.horizontal(|ui|{
                    ui.label("Email:");
                    ui.text_edit_singleline(email);
                });
                ui.horizontal(|ui|{
                    ui.label("Password:");
                    password_ui(ui,pwd)
                });
			match lang.as_str(){
			    "zh"=>ui.small("不少于8个字符，仅数字、字母与特殊符号。"),
			    _=>ui.small("no less than 8 characters."),
			};
            ui.horizontal(|ui|{
		let tt_fgt=match lang.as_str(){
		    "zh"=>"忘记密码？",
			_=>"I forget the password",
		};
                if ui.button(tt_fgt).clicked(){
                    let _=1;
                }
		let tt_lgi=match lang.as_str(){"zh"=>"登录",_=>"Login."};
                if ui.button(tt_lgi).clicked(){
                    let _x=1;
		    
            let rt=tokio::runtime::Builder::new_current_thread()
                    .enable_all().build().unwrap();
            let mut res=("".to_owned(),
            "0".to_owned(),"nothing".to_owned(),"not_activate".to_owned());
            rt.block_on(async{
                // println!("email:{}，pwd:{}",&email,&pwd);
                res=query_login(&email,&pwd).await;
            });
		    if res.0=="Ok"{
			*login_state=res.1.parse().unwrap();
			*user_type=res.2;
			*activation_state=res.3;
		    }
		    else if res.0=="pwd_error"{
    			ui.label("Incorrect emails or passwords.");
		    }
		    else{
	    		ui.label("Incorrect emails or passwords.");
		    }
                }
		let tt_sgu=match lang.as_str(){"zh"=>"注册账号",_=>"No account? Sign Up"};
                if ui.button(tt_sgu).clicked(){
                    // *is_open_login=false;
                    *is_open_signup=true;
                }
		
            });
		    });

	    let tt_sguu=match lang.as_str(){"zh"=>"注册，以同步您的私有信息",
				   _=>"Sign up, to sync your information"};
            egui::Window::new(tt_sguu).default_width(300.0)
		    .open(is_open_signup)
		    .show(ctx,|ui|{
                
                // ui.heading("Sign Up Now!");
                ui.horizontal(|ui|{
		        match lang.as_str(){
			    "zh"=>ui.label("邮箱："),
			_   =>ui.label("Email:"),
		        };
                    ui.text_edit_singleline(email);
                });
                ui.horizontal(|ui|{
		    match lang.as_str(){
			"zh"=>ui.label("密码："),
			_=>ui.label("Password:"),
		    };
                    
                    password_ui(ui,pwd)
                });
		match lang.as_str(){
		    "zh"=>ui.small("不少于8个字符，仅数字、字母与特殊符号。"),
		    _=>ui.small("no less than 8 characters."),
		};
            
                ui.horizontal(|ui|{
		    match lang.as_str(){
"zh"=>ui.label("再次输入:"),
_=>ui.label("Password Again:"),
		    };
                    
                    password_ui(ui,pwd2)
                });

                if pwd!=pwd2{
		    let tt_pic=match lang.as_str(){
			"zh"=>"密码不一致",
			_=>"Password inconsistant"
		    };
                    ui.colored_label(egui::Color32::RED,
                         tt_pic);
                }

            ui.horizontal(|ui|{
		let tt_sgu_b=match lang.as_str(){"zh"=>"注册",_=>"Now Sign Up!"};
		let tt_sgu_b_ah=match lang.as_str(){"zh"=>"转至登录页面",_=>"Already have a account? Login."};
                if ui.button(tt_sgu_b).clicked(){
                    let rt=tokio::runtime::Builder::new_current_thread()
                    .enable_all().build().unwrap();
		    let mut res=("".to_owned(),"".to_owned(),
			     "not_activate".to_owned(),"nothing".to_owned(),
		    );
		    rt.block_on(async{
			res=signup(&email,&pwd).await;
		    });

		    if res.0=="Ok"{
			*login_state=res.1.parse().unwrap();
			*user_type=res.2;
			*activation_state=res.3;
		    }
		    else if res.0=="pwd_error"{
    			ui.label(&res.0);
		    }
		    else{
	    		ui.label(&res.0);
		    }

                    let _x=1;
                }
                if ui.button(tt_sgu_b_ah).clicked(){
                    *is_open_login=true;
                    // *is_open_signup=false;
                }
            });
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

                    egui::Window::new("").default_width(280.0)
                    .open(is_open_payment_qr)
                    .show(ctx, |ui| {
			let tt_donate=match lang.as_str(){
			    "zh"=>"如果你喜欢这个项目，可以请作者喝杯奶茶，哈哈哈！开支：\n Server cost: 750 CHY /year\n Domain name cost: 88  CHY /year",
			    _=>"Donate to support this site, and me!\n Server cost: 750 CHY /year\n Domain name cost: 88  CHY /year"
			};
			ui.label(tt_donate);
            let wechat_img=include_bytes!("../data/wechat_qr.jpg");
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
				    "../data/wechat_qr.jpg",
				    x,
				    Default::default()
				);
            let mut size=wechat_texture.size_vec2();
            size.x=size.x/4.0;
            size.y=size.y/4.0;
			ui.image(&wechat_texture,size);

            let wechat_img=include_bytes!("../data/alipay_qr.jpg");
            let image=image::load_from_memory(wechat_img).
            expect("failed load img");
            let img_buffer=image.to_rgba8();
            let size = (image.width() as usize, image.height() as usize);
            let pixels = img_buffer.into_vec();
            let size=[size.0,size.1];
            let x=egui::ColorImage::from_rgba_premultiplied(size,&pixels);

			let wechat_texture: egui::TextureHandle =
				ui.ctx().load_texture(
				    "../data/wechat_qr.jpg",
				    x,
				    Default::default()
				);
            let mut size=wechat_texture.size_vec2();
            size.x=size.x/4.0;
            size.y=size.y/4.0;
			ui.image(&wechat_texture,size);
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
                });

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
		

                // ui.heading("eframe template");
                // ui.hyperlink("https://github.com/emilk/eframe_template");
                // ui.add(egui::github_link_file!(
                //     "https://github.com/emilk/eframe_template/blob/master/",
                //     "Source code."
                // ));
                // egui::warn_if_debug_build(ui);
            });
        }

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
use egui::text::LayoutJob;

/// View some code with syntax highlighting and selection.
pub fn code_view_ui(ui: &mut egui::Ui, mut code: &str) {
    ui.add(
        egui::TextEdit::multiline(&mut code)
            .font(egui::TextStyle::Monospace) // for cursor height
            .code_editor()
            .desired_rows(1)
            .lock_focus(true),
    );
}

pub fn open_one_reader(ctx:&Context, fname: &str,
		       is_open:&mut bool,
		       docl:&mut DocLabeled){
    egui::Window::new(fname).default_open(true)
           .default_height(400.0).default_width(600.0)
	   .collapsible(true)
	   .constrain(true)
	   .open(is_open)
	   .show(ctx, |ui|{
	       egui::SidePanel::left("headline")
                   .resizable(true)
                   .default_width(200.0)
                   .show_inside(ui, |ui|{
		       // ui.heading("Headline of Books");
		       ui.vertical_centered(|ui|{
		       ui.heading("Headline of Books");
		       });
		       egui::ScrollArea::vertical().show(ui, |ui|{
			   ui.label("111111");
			   ui.label("111111");
			   ui.label("111111");
			   ui.label("111111");
		       });
		   });
	       egui::CentralPanel::default()
		   // .show(ui, |ui|{
		   .show_inside(ui, |ui|{
		       egui::ScrollArea::vertical().show(ui, |ui|{
			   ui.text_edit_multiline(&mut "11111111");
			   render_selected_text(ui, docl);
		       });
	       });
	       egui::SidePanel::right("notes")
                   .resizable(true)
                   .default_width(200.0)
                   .show_inside(ui, |ui|{
		       ui.heading("Notes of Books");
		       egui::ScrollArea::vertical().show(ui, |ui|{
			   ui.label("111111");
			   ui.label("222222");
			   ui.label("111111");
			   ui.label("111111");
		       });
		   });
	   });
}

pub fn render_selected_text(ui:&mut egui::Ui,docl:&mut DocLabeled){
    
    // ui.text_edit_multiline(&mut docl.rendering().as_str());
    ui.label(docl.rendering());
    ui.label("WWWWWWWWWWWWWWWWWWW");

}


#[allow(clippy::ptr_arg)] // false positive
pub fn password_ui(ui: &mut egui::Ui, password: &mut String) -> egui::Response {
    // This widget has its own state — show or hide password characters (`show_plaintext`).
    // In this case we use a simple `bool`, but you can also declare your own type.
    // It must implement at least `Clone` and be `'static`.
    // If you use the `persistence` feature, it also must implement `serde::{Deserialize, Serialize}`.

    // Generate an id for the state
    let state_id = ui.id().with("show_plaintext");

    // Get state for this widget.
    // You should get state by value, not by reference to avoid borrowing of [`Memory`].
    let mut show_plaintext = ui.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

    // Process ui, change a local copy of the state
    // We want TextEdit to fill entire space, and have button after that, so in that case we can
    // change direction to right_to_left.
    let result = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        // Toggle the `show_plaintext` bool with a button:
        let response = ui
            .add(egui::SelectableLabel::new(show_plaintext, "👁"))
            .on_hover_text("Show/hide password");

        if response.clicked() {
            show_plaintext = !show_plaintext;
        }

        // Show the password field:
        ui.add_sized(
            ui.available_size(),
            egui::TextEdit::singleline(password).password(!show_plaintext),
        );
    });

    // Store the (possibly changed) state:
    ui.data_mut(|d| d.insert_temp(state_id, show_plaintext));

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, …) and maybe show a tooltip:
    result.response
}

// A wrapper that allows the more idiomatic usage pattern: `ui.add(…)`
/// Password entry field with ability to toggle character hiding.
///
/// ## Example:
/// ``` ignore
/// ui.add(password(&mut my_password));
/// ```
pub fn password(password: &mut String) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| password_ui(ui, password)
}

// pub fn url_to_file_source_code() -> String {
//     format!("https://github.com/emilk/egui/blob/master/{}", file!())
// }
