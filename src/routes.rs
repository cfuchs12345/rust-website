use std::collections::HashMap;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use services::projectservice;
use tera::Context;

use crate::{AppData, LANGUAGE};

pub(crate) async fn index(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    set_language_for_page(req); 
    
    let mut ctx = Context::new();
    ctx.insert("name", "index.html");
    let rendered = match data.app_data_templates.render("tera/index.html.tera", &ctx) {
        Ok(t) => t,
        Err(e) => e.to_string()
    };
    HttpResponse::Ok().body(rendered)
}


pub(crate) async fn about_me(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    set_language_for_page(req);

    let mut ctx = Context::new();
    ctx.insert("name", "aboutMe.html");    

    let rendered = match data.app_data_templates.render("tera/aboutMe.html.tera", &ctx) {
        Ok(t) => t,
        Err(e) => e.to_string()
    };
    HttpResponse::Ok().body(rendered)
}

pub(crate) async fn about_page(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    set_language_for_page(req);

    let mut ctx = Context::new();
    ctx.insert("name", "aboutPage.html");
    let rendered = match data.app_data_templates.render("tera/aboutPage.html.tera", &ctx) {
        Ok(t) => t,
        Err(e) => e.to_string()
    };
    HttpResponse::Ok().body(rendered)
}

pub(crate) async fn skills(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    set_language_for_page(req);

    let mut ctx = Context::new();
    ctx.insert("name", "skills.html");
    
    let rendered = match data.app_data_templates.render("tera/skills.html.tera", &ctx) {
        Ok(t) => t,
        Err(e) => e.to_string()
    };
    HttpResponse::Ok().body(rendered)
}

pub(crate) async fn projects(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    set_language_for_page(req);

    let projects = projectservice::get_all(&data.app_data_conn).await.unwrap();
    
    
    let mut ctx = Context::new();
    ctx.insert("name", "projects.html");
    ctx.insert("projects", &projects);
    let rendered = match data.app_data_templates.render("tera/projects.html.tera", &ctx) {
        Ok(t) => t,
        Err(e) => e.to_string()
    };
    HttpResponse::Ok().body(rendered)
}

pub(crate) async fn contact(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    set_language_for_page(req);

    let mut ctx = Context::new();
    ctx.insert("name", "projects.html");
    let rendered = match data.app_data_templates.render("tera/contact.html.tera", &ctx) {
        Ok(t) => t,
        Err(e) => e.to_string()
    };
    HttpResponse::Ok().body(rendered)
}

pub(crate) async fn privateprojects(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    set_language_for_page(req);

    let mut ctx = Context::new();
    ctx.insert("name", "privateprojects.html");
    let rendered = match data.app_data_templates.render("tera/privateprojects.html.tera", &ctx) {
        Ok(t) => t,
        Err(e) => e.to_string()
    };
    HttpResponse::Ok().body(rendered)
}

pub(crate) async fn legal_info(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    set_language_for_page(req);

    let mut ctx = Context::new();
    ctx.insert("name", "projects.html");
    let rendered = match data.app_data_templates.render("tera/legalInfo.html.tera", &ctx) {
        Ok(t) => t,
        Err(e) => e.to_string()
    };
    HttpResponse::Ok().body(rendered)
}

fn set_language_for_page(req: HttpRequest) {
    let query = split_query_string( req.query_string());
    let language_param = query.get("language");

    let language_header = get_language_from_header(&req);
    LANGUAGE.with(
        |cell| {
            let mut language_ref = cell.borrow_mut();

            match language_param {
                Some(lang) => language_ref.replace(lang.to_string()),
                None => language_ref.replace(language_header)
            }
        }
    );
}

fn get_language_from_header(req: &HttpRequest) -> String {
    let header = req.headers().get("accept-language");

    if let Some(val) = header {
        let formatted =  format!("{:?}", val).replace('\"'," ");
        
        if formatted.to_lowercase().starts_with("de") {
            return "de".to_string();
        }
    }
  
    "en".to_string()
}


fn split_query_string(string: &str) -> HashMap<&str, &str> {
    if string.is_empty()|| ! string.contains('=') {
        return HashMap::new();
    }
    string.split(',').map(|s| s.split_at(s.find('=').unwrap())).map(|(key, val)| (key, &val[1..])).collect()
}