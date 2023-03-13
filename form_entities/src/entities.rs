use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ClientWithBusinessArea {
    pub id: String,
    pub name: String,
    pub businessarea_id: i16
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProjectWithRelatedEntites {
    pub id: String,
    pub summary_de: String,
    pub summary_en: String,
    pub description_de: String,
    pub description_en: String,
    pub client_ids: String,
    pub businessarea_ids: String,
    pub person_ids: String,
    pub role_ids: String,
    pub technology_ids: String,
    pub duration: String,
    pub from: String,
    pub to: String,
}
