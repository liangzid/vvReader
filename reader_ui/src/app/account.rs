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

use crate::app::communicate::*;

use crate::app::password::password_ui;

pub fn render_login_windows(ctx:&Context,lang:&String,
			      is_open_login:&mut bool,
			      is_open_signup:&mut bool,
			      email:&mut String,
			      pwd:&mut String,
			      login_state:&mut i8,
			      user_type:&mut String,
			      activation_state:&mut String,
){
            let tt_login = match lang.as_str() {
                "zh" => "登录，以同步您的私有信息",
                _ => "Login to sync your information!",
            };
            egui::Window::new(tt_login)
                .default_width(300.0)
                .open(is_open_login)
                .show(ctx, |ui| {
                    // ui.heading("Login to your account!");
                    ui.horizontal(|ui| {
                        ui.label("Email:");
                        ui.text_edit_singleline(email);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Password:");
                        password_ui(ui,pwd)
                    });
                    match lang.as_str() {
                        "zh" => ui.small("不少于8个字符，仅数字、字母与特殊符号。"),
                        _ => ui.small("no less than 8 characters."),
                    };
                    ui.horizontal(|ui| {
                        let tt_fgt = match lang.as_str() {
                            "zh" => "忘记密码？",
                            _ => "I forget the password",
                        };
                        if ui.button(tt_fgt).clicked() {
                            let _ = 1;
                        }
                        let tt_lgi = match lang.as_str() {
                            "zh" => "登录",
                            _ => "Login.",
                        };
                        if ui.button(tt_lgi).clicked() {
                            let _x = 1;

                            let rt = tokio::runtime::Builder::new_current_thread()
                                .enable_all()
                                .build()
                                .unwrap();
                            let mut res = (
                                "".to_owned(),
                                "0".to_owned(),
                                "nothing".to_owned(),
                                "not_activate".to_owned(),
                            );
                            rt.block_on(async{
                                // println!("email:{}，pwd:{}",&email,&pwd);
                                res=query_login(&email,
						&pwd).await;
                            });
                            if res.0 == "Ok" {
                                *login_state = res.1.parse().unwrap();
                                *user_type = res.2;
                                *activation_state = res.3;
                            } else if res.0 == "pwd_error" {
                                ui.label("Incorrect emails or passwords.");
                            } else {
                                ui.label("Incorrect emails or passwords.");
                            }
                        }
                        let tt_sgu = match lang.as_str() {
                            "zh" => "注册账号",
                            _ => "No account? Sign Up",
                        };
                        if ui.button(tt_sgu).clicked() {
                            // *is_open_login=false;
                            *is_open_signup = true;
                        }
                    });
                });
}



pub fn render_signup_windows(ctx:&Context,lang:&String,
			      is_open_login:&mut bool,
			      is_open_signup:&mut bool,
			      email:&mut String,
			      pwd:&mut String,
			      pwd2:&mut String,
			      login_state:&mut i8,
			      user_type:&mut String,
			      activation_state:&mut String,){
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
}
