use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260801_103755_add_todo_relations"
    }
}

const TODO_USER_FK: &str = "fk_todo_user";
const TODO_CATEGORY_FK: &str = "fk_todo_category";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Todos::Table)
                    .add_column(uuid(Todos::UserId).not_null())
                    .add_column(uuid(Todos::CategoryId).null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name(TODO_USER_FK)
                            .from_tbl(Todos::Table)
                            .from_col(Todos::UserId)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name(TODO_CATEGORY_FK)
                            .from_tbl(Todos::Table)
                            .from_col(Todos::CategoryId)
                            .to_tbl(Category::Table)
                            .to_col(Category::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Todos::Table)
                    .drop_foreign_key(Alias::new(TODO_USER_FK))
                    .drop_foreign_key(Alias::new(TODO_CATEGORY_FK))
                    .drop_column(Todos::UserId)
                    .drop_column(Todos::CategoryId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Todos {
    Table,
    UserId,
    CategoryId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
}
