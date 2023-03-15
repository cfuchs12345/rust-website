use actix_web::HttpRequest;
use actix_web::{get, post, web,  HttpResponse, Responder};
use entities::project::ProjectTuple;
use serde::{Deserialize, Serialize};

use services::businessareaservice;
use services::clientservice;
use services::personservice;
use services::projectlistservice;
use services::projectservice;
use entities::projectlist::ProjectList;
use services::roleservice;
use services::technologyservice;
use crate::AppData;

#[derive(Default, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct JsonErrorResult {
    error: String
}





#[get("/list")]
pub(crate) async fn list_projects(data: web::Data<AppData>,  req: HttpRequest) -> impl Responder {
    if ! api_key_successfully_validated(&data, &req) {
        return HttpResponse::Unauthorized().finish();
    }

    let projects_result = projectservice::get_all(&data.app_data_conn).await;

    let result:Result<&Vec<ProjectTuple>, serde_json::Value>;
    let projects: Vec<ProjectTuple>;

    match projects_result {
        Ok(tmp) => {
            projects = tmp;

            result = Ok(&projects);
        },
        e => { 
                let error = JsonErrorResult {
                error: format!("{:?}", e)
            };
            result = Err(serde_json::to_value(error).unwrap());
        }
    }

    match result {
        Ok(result) => HttpResponse::Ok().content_type("application/json").json(result),
        Err(e) => HttpResponse::InternalServerError().json(e)
    }
}




#[get("/show/{id}")]
pub(crate) async fn show_project(data: web::Data<AppData>, req: HttpRequest, path: web::Path<i16>) -> impl Responder {
    if ! api_key_successfully_validated(&data, &req) {
        return HttpResponse::Unauthorized().finish();
    }

    let id = path.into_inner();

    let result:Result<&ProjectTuple, serde_json::Value>;

    let found_projects_result = projectservice::get(id, &data.app_data_conn).await;
    let found_projects:Vec<ProjectTuple>;

    match found_projects_result {
        Ok(e) => {
            found_projects = e;
            match found_projects.first() {
                Some(found_project) => {
                    result = Ok(found_project);
                },
                None => {
                    let error = JsonErrorResult {
                        error: format!("Project {} not found", id)
                    };
                    result = Err(serde_json::to_value(error).unwrap()); 
                }
            }                
        },
        e => {
            let error = JsonErrorResult {
                error: format!("{:?}", e)
            };
            result = Err(serde_json::to_value(error).unwrap()); 
        }
    };

    match result {
        Ok(result) => HttpResponse::Ok().content_type("application/json").json(result),
        Err(e) => HttpResponse::InternalServerError().json(e)
    }
}

#[post("/savelist")]
pub(crate) async fn save_projects(data: web::Data<AppData>,  req: HttpRequest, projectlist: web::Json<ProjectList>) -> impl Responder {
    if ! api_key_successfully_validated(&data, &req) {
        return HttpResponse::Unauthorized().finish();
    }

    let result:Result<(), serde_json::Value>;

    match projectservice::delete_all(&data.app_data_conn).await {
        Ok(_tmp) => {
            businessareaservice::delete_all(&data.app_data_conn).await.unwrap();
            clientservice::delete_all(&data.app_data_conn).await.unwrap();
            roleservice::delete_all(&data.app_data_conn).await.unwrap();
            personservice::delete_all(&data.app_data_conn).await.unwrap();
            technologyservice::delete_all(&data.app_data_conn).await.unwrap();

            match projectlistservice::save_list_as_new(&data.app_data_conn, &projectlist).await {
                Ok(_tmp) => {
                    result = Ok(());
                },
                e => {
                    let error = JsonErrorResult {
                        error: format!("{:?}", e)
                    };
                    result = Err(serde_json::to_value(error).unwrap()); 
                }
            };
        },
        e => { 
            let error = JsonErrorResult {
                error: format!("{:?}", e)
            };
            result = Err(serde_json::to_value(error).unwrap()); 
        }
    };
    log::error!("result {:?}", result);
    match result {
        Ok(_result) => HttpResponse::Ok().content_type("application/json").json("OK"),
        Err(e) => HttpResponse::InternalServerError().json(e)
    }
}


fn api_key_successfully_validated(data: &web::Data<AppData>, req: &HttpRequest) -> bool {
    let api_key_header = req.headers().get("X-API-KEY");
    
    match api_key_header {
        Some(header_value)  => {

            match header_value.to_str() {
                Ok(header_value_as_str) => {
                    match data.app_data_config.get_string("api_key") {
                        Ok(reference_key) => if reference_key == header_value_as_str {
                            log::debug!("provided API key matches reference key. Successfully authentication successful");
                            true
                        }
                        else {
                            log::error!("provided api key does not match reference key from env {:?}", reference_key);
                            false
                        }
                        _e => {
                            log::error!("No api key in .env file, Cannot validate incoming api key");
                            false
                        }
                    }
                },
                e => {
                    log::error!("header value X-API-KEY seems to contain invalid ASCII characters {:?}", e);
                    false
                }
            }
        },
        None => false
    }
}