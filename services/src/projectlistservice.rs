use std::collections::{HashSet, HashMap};
use entities::businessarea;
use entities::project::ProjectAndDependencies;
use ::entities::projectlist::ProjectList;
use ::entities::{role, person, technology, client};

use crate::{projectservice, clientservice, businessareaservice, roleservice, personservice, technologyservice};

use sea_orm::{DbConn, DbErr};




pub async fn save_list_as_new(db: &DbConn, projectlist: &ProjectList) -> Result<(), DbErr> {
    let list = &projectlist.list;

    let mut clients:HashSet<client::Model> = HashSet::new();
    let mut businessareas:HashSet<businessarea::Model> = HashSet::new();
    let mut roles:HashSet<role::Model> = HashSet::new();
    let mut persons:HashSet<person::Model> = HashSet::new();
    let mut technologies:HashSet<technology::Model> = HashSet::new();
 
    list.iter().for_each(|tuple| {
        let client_temp:HashSet<client::Model> = HashSet::from_iter(tuple.1.clone());
        let businessarea_tmp:HashSet<businessarea::Model> = HashSet::from_iter(tuple.2.clone());
        let roles_temp:HashSet<role::Model> = HashSet::from_iter(tuple.3.clone());
        let person_temp:HashSet<person::Model> = HashSet::from_iter(tuple.4.clone());
        let technology_temp:HashSet<technology::Model> = HashSet::from_iter(tuple.5.clone());
        businessareas.extend(businessarea_tmp);
        clients.extend(client_temp);
        roles.extend(roles_temp);
        persons.extend(person_temp);
        technologies.extend(technology_temp);
    });
    
    let clients_map = save_clients_and_get_map(db, clients).await;
    let businessarea_map = save_businessareas_and_get_map(db, businessareas).await;
    let roles_map = save_roles_and_get_map(db, roles).await;
    let persons_map = save_persons_and_get_map(db, persons).await;
    let technologies_map = save_technologies_and_get_map(db, technologies).await;

    
    
    
    for list_entry in list {
        let mut new_project = ProjectAndDependencies {
            project: list_entry.0.clone(),
            clients: list_entry.1.iter().map( |c| -> i16 {
                let res = clients_map.get(c.name.as_str());
                res.unwrap().to_owned()
            }).collect(),      
            businessareas: list_entry.2.iter().map(|b| -> i16 {
                let res = businessarea_map.get(b.name_de.as_str());
                res.unwrap().to_owned()
            }).collect(),
            roles:list_entry.3.iter().map( |r| -> i16 {
                let res = roles_map.get(r.name_de.as_str());
                res.unwrap().to_owned()
            }).collect(), 
            persons:list_entry.4.iter().map( |p| -> i16 {
                let res = persons_map.get(p.name.as_str());
                res.unwrap().to_owned()
            }).collect(), 
            technologies:list_entry.5.iter().map( |t| -> i16 {
                let res = technologies_map.get(t.name.as_str());
                res.unwrap().to_owned()
            }).collect(), 
        };
        new_project.project.id = 0; // force save as new
        projectservice::save(db, new_project).await?;
    }
    

    Ok(())
}


async fn save_businessareas_and_get_map(db: &DbConn,businessareas:HashSet<businessarea::Model>  ) -> HashMap<String, i16>{
    let mut map:HashMap<String, i16> = HashMap::new();

    log::error!("businessareas to save {:?}", businessareas);


    for mut businessarea in businessareas {
        log::warn!("saving {:?}", businessarea);
        businessarea.id = 0; // set to 0 since we want to force an insert as new entity
        let saved_name_and_id = businessareaservice::save(db, businessarea).await;

        match saved_name_and_id {
            Ok(name_and_id) => { map.insert(name_and_id.0, name_and_id.1); },
            e => log::error!("Error while saving business_area: {:?}", e)
        }
    }
    map
}

async fn save_clients_and_get_map(db: &DbConn,clients:HashSet<client::Model>  ) -> HashMap<String, i16>{
    let mut map:HashMap<String, i16> = HashMap::new();

    for mut client in clients {
        client.id = 0;// set to 0 since we want to force an insert as new entity

        let saved_name_and_id = clientservice::save(db, client).await;
        
        match saved_name_and_id {
            Ok(name_and_id) => { map.insert(name_and_id.0, name_and_id.1); },
            e => {log::error!("error while saving clients: {:?}", e)}
        }
    }
    map
}

async fn save_roles_and_get_map(db: &DbConn,roles:HashSet<role::Model>  ) -> HashMap<String, i16>{
    let mut map:HashMap<String, i16> = HashMap::new();
    for mut role in roles {
        role.id = 0;// set to 0 since we want to force an insert as new entity

        let saved_name_and_id = roleservice::save(db, role).await;

        match saved_name_and_id {
            Ok(name_and_id) => { map.insert(name_and_id.0, name_and_id.1); },
            _ => {}
        }
    }
    map
}

async fn save_persons_and_get_map(db: &DbConn,persons:HashSet<person::Model>  ) -> HashMap<String, i16>{
    let mut map:HashMap<String, i16> = HashMap::new();
    for mut person in persons {
        person.id = 0;// set to 0 since we want to force an insert as new entity

        let saved_name_and_id = personservice::save(db, person).await;

        match saved_name_and_id {
            Ok(name_and_id) => { map.insert(name_and_id.0, name_and_id.1); },
            _ => {}
        }
    }
    map
}

async fn save_technologies_and_get_map(db: &DbConn,technologies:HashSet<technology::Model>  ) -> HashMap<String, i16>{
    let mut map:HashMap<String, i16> = HashMap::new();
    for mut technology in technologies {
        technology.id = 0;// set to 0 since we want to force an insert as new entity

        let saved_name_and_id = technologyservice::save(db, technology).await;

        match saved_name_and_id {
            Ok(name_and_id) => { map.insert(name_and_id.0, name_and_id.1); },
            _ => {}
        }
    }
    map
}