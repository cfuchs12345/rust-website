use std::{cell::RefCell, collections::HashMap};

use ::config::Config;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer, Result};

use sea_orm::DatabaseConnection;
use tera::Tera;

mod config;
mod database;
mod project_routes;
mod routes;

#[derive(Debug, Clone)]
pub struct AppData {
    pub app_data_templates: Tera,
    pub app_data_conn: DatabaseConnection,
    pub app_data_config: Config,
}

#[macro_use]
extern crate lazy_static;

// this loads the translations from the json files and makes it accessible via a static reference
// the file is not reloaded
lazy_static! {
    static ref TRANSLATIONS: serde_json::Value = config::get_translations().unwrap();
}
// this holds the accepted language coming from the client request for each individual thread
// only needs to be available for the page redending happening after the request
// no need to store the information per client
thread_local!(static LANGUAGE: RefCell<Option<String>> = RefCell::new(None));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init();

    let config = config::get_config();
    let conn = database::connect().await.unwrap();

    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let bind_address = config.get_string("bind_address");

    tera.register_function("get_translation", get_translation_value);
    tera.register_function("get_language", get_language_value);

    database::migrate(&conn).await;

    let app_data = AppData {
        app_data_templates: tera,
        app_data_conn: conn,
        app_data_config: config,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_data.clone()))
            .service(
                web::scope("/projects")
                    .service(project_routes::list_projects)
                    .service(project_routes::show_project)
                    .service(project_routes::save_projects),
            )
            .service(web::resource("/").route(web::get().to(routes::index)))
            .service(web::resource("/index.html").route(web::get().to(routes::index)))
            .service(web::resource("/aboutMe.html").route(web::get().to(routes::about_me)))
            .service(web::resource("/aboutPage.html").route(web::get().to(routes::about_page)))
            .service(web::resource("/skills.html").route(web::get().to(routes::skills)))
            .service(web::resource("/projects.html").route(web::get().to(routes::projects)))
            .service(
                web::resource("/privateprojects.html")
                    .route(web::get().to(routes::privateprojects)),
            )
            .service(web::resource("/contact.html").route(web::get().to(routes::contact)))
            .service(web::resource("/legalInfo.html").route(web::get().to(routes::legal_info)))
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind(bind_address.unwrap())?
    .run()
    .await
}

fn get_language_value(_args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let language = get_language();

    Ok(tera::to_value(language).unwrap())
}

fn get_translation_value(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let language = get_language();
    match args.get("key") {
        Some(key) => {
            let key_str = key.as_str().unwrap();
            let mut key_split = key_str.split('.'); 

            let root_key = key_split.next().unwrap(); // first is normally the root node "translation"

            let root_value = TRANSLATIONS.get(root_key).unwrap();
            let translation = get_translation_from_file(&mut key_split, language, root_value);

            Ok(tera::to_value(
                translation.unwrap_or(format!("no translation found for key {}", key_str)),
            )
            .unwrap())
        }
        None => Ok(tera::to_value("no translation found").unwrap()),
    }
}

fn get_language() -> String {
    LANGUAGE.with(|cell| -> String {
        let language = cell.borrow();

        match language.as_deref() {
            Some(language) => {
                language.to_string()
            },
            None => "en".to_string()
        }
    })
}

fn get_translation_from_file(
    key_split: &mut std::str::Split<char>,
    language: String,
    json_value: &serde_json::value::Value,
) -> Option<String> {
    let current_key_opt = key_split.next();

    match current_key_opt {
        Some(current_key) => {
            let next_value = json_value.get(current_key);

            match next_value {
                Some(next_value) => get_translation_from_file(key_split, language, next_value),
                None => {
                    log::error!(
                        "value for current key {:?} not found in translation file",
                        current_key
                    );
                    None
                }
            }
        }
        None => {
            let val_for_lang_opt = json_value.get(language);

            match val_for_lang_opt {
                Some(val_for_lang) => {
                    if val_for_lang.is_array() {
                        let array = val_for_lang.as_array().unwrap();
                        let mut vec: Vec<String> = Vec::new();

                        array.iter().fold(&mut vec, |vec, val| {
                            vec.push(val.as_str().unwrap().to_string());
                            vec
                        });

                        Some(vec.join("\n"))
                    } else {
                        Some(val_for_lang.as_str().unwrap().to_string())
                    }
                }
                None => None,
            }
        }
    }
}
