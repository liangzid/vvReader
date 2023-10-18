use std::path::Path;
use std::default::{self, Default};
use std::{collections::HashMap, hash::Hash};
use serde;

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Heading{
    pub name:String,
    pub location:usize,
    pub level:u8,
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct RawStructruedText{
    pub heads:Vec<Heading>,
    pub raw_text:String,
}
