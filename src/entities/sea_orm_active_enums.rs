//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "gender")]
pub enum Gender {
    #[sea_orm(string_value = "female")]
    Female,
    #[sea_orm(string_value = "male")]
    Male,
    #[sea_orm(string_value = "unknown")]
    Unknown,
}