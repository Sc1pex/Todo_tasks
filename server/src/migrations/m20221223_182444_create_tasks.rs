use sea_orm_migration::prelude::*;

use crate::migrations::m20220101_000001_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Task::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Task::UserId).uuid().not_null())
                    .col(ColumnDef::new(Task::Name).string().not_null())
                    .col(ColumnDef::new(Task::Desc).string().not_null())
                    .col(
                        ColumnDef::new(Task::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Task::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-user-id")
                            .from(User::Table, User::Id)
                            .to(Task::Table, Task::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Task {
    Table,
    Id,
    UserId,
    Name,
    Desc,
    CreatedAt,
    UpdatedAt,
}
