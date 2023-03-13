use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::common;

#[derive(Clone, Hash, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "technology")]

pub struct Model {
    #[serde(default)]
    #[sea_orm(primary_key)]
    pub id: i16,
    #[sea_orm(column_type = "Text")]
    pub name: String
}

impl common::DBEntity for Model {
    fn get_id(&self) -> &i16 {
        &self.id
    }
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::Id",
        to = "super::project::Column::Id"
    )]
    Technology,
}


impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        super::project_technology::Relation::Project.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::project_technology::Relation::Technology.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {} 