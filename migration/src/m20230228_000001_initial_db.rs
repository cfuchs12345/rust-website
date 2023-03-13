use sea_orm_migration::prelude::*;


pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230228_000001_initial_db"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let _db = manager.get_connection();

        manager
            .create_table(
                Table::create()
                    .table(Client::Table)
                    .col(
                        ColumnDef::new(Client::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Client::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Businessarea::Table)
                    .col(
                        ColumnDef::new(Businessarea::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Businessarea::NameDe)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Businessarea::NameEn)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

            manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .col(
                        ColumnDef::new(Role::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Role::NameDe)
                        .string()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Role::NameEn)
                        .string()
                        .not_null(),
                    )

                    .to_owned(),
            )
            .await.unwrap();

            manager
            .create_table(
                Table::create()
                    .table(Technology::Table)
                    .col(
                        ColumnDef::new(Technology::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Technology::Name)
                        .string()
                        .not_null(),
                    )
                    .to_owned(),
            )
            .await.unwrap();

            manager
            .create_table(
                Table::create()
                    .table(Person::Table)
                    .col(
                        ColumnDef::new(Person::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Person::Name)
                        .string()
                        .not_null(),
                    )
                    .to_owned(),
            )
            .await.unwrap();

            manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .col(
                        ColumnDef::new(Project::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Project::SummaryDe)
                        .string()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Project::SummaryEn)
                        .string()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Project::DescriptionDe)
                        .string()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Project::DescriptionEn)
                        .string()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Project::Duration)
                        .string()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Project::From)
                        .string()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Project::To)
                        .string()
                        .not_null(),
                    )
                    .to_owned(),
            )
            .await.unwrap();
            

            manager
            .create_table(
                Table::create()
                    .table(ProjectClient::Table)
                    .col(
                        ColumnDef::new(ProjectClient::ProjectId)
                        .integer()
                        .not_null()                        
                    )
                    .col(
                        ColumnDef::new(ProjectClient::ClientId)
                        .integer()
                        .not_null()                        
                    )
                   
                    .to_owned(),
            )
            .await.unwrap();

            manager
            .create_table(
                Table::create()
                    .table(ProjectRole::Table)
                    .col(
                        ColumnDef::new(ProjectRole::ProjectId)
                        .integer()
                        .not_null()
                        
                    )
                    .col(
                        ColumnDef::new(ProjectRole::RoleId)
                        .integer()
                        .not_null()
                        
                    )
                   
                    .to_owned(),
            )
            .await.unwrap();

            manager
            .create_table(
                Table::create()
                    .table(ProjectPerson::Table)
                    .col(
                        ColumnDef::new(ProjectPerson::ProjectId)
                        .integer()
                        .not_null()
                        
                    )
                    .col(
                        ColumnDef::new(ProjectPerson::PersonId)
                        .integer()
                        .not_null()
                        
                    )
                   
                    .to_owned(),
            )
            .await.unwrap();

            manager
            .create_table(
                Table::create()
                    .table(ProjectTechnology::Table)
                    .col(
                        ColumnDef::new(ProjectTechnology::ProjectId)
                        .integer()
                        .not_null()
                        
                    )
                    .col(
                        ColumnDef::new(ProjectTechnology::TechnologyId)
                        .integer()
                        .not_null()
                        
                    )
                   
                    .to_owned(),
            )
            .await.unwrap();

            manager
            .create_table(
                Table::create()
                    .table(ProjectBusinessarea::Table)
                    .col(
                        ColumnDef::new(ProjectBusinessarea::ProjectId)
                        .integer()
                        .not_null()
                        
                    )
                    .col(
                        ColumnDef::new(ProjectBusinessarea::BusinessareaId)
                        .integer()
                        .not_null()
                        
                    )
                   
                    .to_owned(),
            )
            .await.unwrap();
        Ok(())
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Client::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Businessarea::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Technology::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Person::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await?;
        manager
        .drop_table(Table::drop().table(ProjectClient::Table).to_owned())
        .await?;
        manager
        .drop_table(Table::drop().table(ProjectRole::Table).to_owned())
        .await?;
        manager
        .drop_table(Table::drop().table(ProjectPerson::Table).to_owned())
        .await?;
        manager
        .drop_table(Table::drop().table(ProjectBusinessarea::Table).to_owned())
        .await?;
        manager
        .drop_table(Table::drop().table(ProjectTechnology::Table).to_owned())
        .await?;
        manager
        .drop_table(Table::drop().table(ProjectBusinessarea::Table).to_owned())
        .await
    

    }
}



#[derive(Iden)]
pub enum Client {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum Businessarea { 
    Table,
    Id,
    NameDe,
    NameEn,
}

#[derive(Iden)]
pub enum Role {
    Table,
    Id,
    NameDe,
    NameEn
}

#[derive(Iden)]
pub enum Technology {
    Table,
    Id,
    Name
}

#[derive(Iden)]
pub enum Person {
    Table,
    Id,
    Name
}


#[derive(Iden)]
pub enum Project {
    Table,
    Id,
    SummaryDe,
    SummaryEn,
    DescriptionDe,
    DescriptionEn,
    Duration,
    From,
    To
}


#[derive(Iden)]
pub enum ProjectClient {
    Table,
    ProjectId,
    ClientId
}

#[derive(Iden)]
pub enum ProjectRole {
    Table,
    ProjectId,
    RoleId
}

#[derive(Iden)]
pub enum ProjectPerson {
    Table,
    ProjectId,
    PersonId
}

#[derive(Iden)]
pub enum ProjectBusinessarea {
    Table,
    ProjectId,
    BusinessareaId
}

#[derive(Iden)]
pub enum ProjectTechnology {
    Table,
    ProjectId,
    TechnologyId
}
