use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_schemas::Schemas;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Authentication, User::Table))
                    .add_column(
                        ColumnDef::new(User::TotpSecretEncrypted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::TotpSecretEncrypted)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    TotpSecretEncrypted,
}
