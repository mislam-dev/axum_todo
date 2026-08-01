pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260731_184725_create_category_table;
mod m20260731_194203_create_uer_table;
mod m20260801_103755_add_todo_relations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260731_184725_create_category_table::Migration),
            Box::new(m20260731_194203_create_uer_table::Migration),
            Box::new(m20260801_103755_add_todo_relations::Migration),
        ]
    }
}
