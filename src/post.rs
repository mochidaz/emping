use std::collections::HashMap;

use chrono::DateTime;
use yew::{Component, ComponentLink, Html, html, ShouldRender};

use crate::generator::vec_of_posts;
use crate::parser;
use crate::utils::{config, get_files};

#[derive(Debug)]
pub enum Layout {
    Default,
    Main,
    Page,
    Post,
}

enum Msg {
    Post,
    Delete,
}

#[derive(Debug)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub layout: Layout,
    pub permalink: String,
    pub date: String
}

impl Post {
    pub fn new(
        title: String,
        content: String,
        layout: Layout,
        permalink: String,
        date: String)
        -> Self {
       Self {
           title,
           content,
           layout,
           permalink,
           date
       }
    }
}