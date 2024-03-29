use std::cmp::Ordering;

#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use bamboo_common_backend_macros::*;
#[cfg(feature = "frontend")]
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Debug, Eq, PartialEq, Clone, Default, Copy)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveActiveEnum),
    sea_orm(
        rs_type = "String",
        db_type = "Enum",
        enum_name = "final_fantasy.district"
    )
)]
pub enum HousingDistrict {
    #[default]
    #[cfg_attr(feature = "backend", sea_orm(string_value = "the_lavender_beds"))]
    TheLavenderBeds,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "mist"))]
    Mist,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "the_goblet"))]
    TheGoblet,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "shirogane"))]
    Shirogane,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "empyreum"))]
    Empyreum,
}

impl HousingDistrict {
    pub fn get_name(self) -> String {
        match self {
            HousingDistrict::TheLavenderBeds => "the_lavender_beds",
            HousingDistrict::Mist => "mist",
            HousingDistrict::TheGoblet => "the_goblet",
            HousingDistrict::Shirogane => "shirogane",
            HousingDistrict::Empyreum => "empyreum",
        }
        .to_string()
    }
}

impl ToString for HousingDistrict {
    fn to_string(&self) -> String {
        match self {
            HousingDistrict::TheLavenderBeds => "Lavendelbeete",
            HousingDistrict::Mist => "Dorf des Nebels",
            HousingDistrict::TheGoblet => "Kelchkuppe",
            HousingDistrict::Shirogane => "Shirogane",
            HousingDistrict::Empyreum => "Empyreum",
        }
        .to_string()
    }
}

impl From<String> for HousingDistrict {
    fn from(value: String) -> Self {
        match value.as_str() {
            "the_lavender_beds" => HousingDistrict::TheLavenderBeds,
            "mist" => HousingDistrict::Mist,
            "the_goblet" => HousingDistrict::TheGoblet,
            "shirogane" => HousingDistrict::Shirogane,
            "empyreum" => HousingDistrict::Empyreum,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for HousingDistrict {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HousingDistrict {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_name().cmp(&other.get_name())
    }
}

#[derive(Serialize, Deserialize, EnumIter, Debug, Eq, PartialEq, Clone, Default, Copy)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveActiveEnum),
    sea_orm(
        rs_type = "String",
        db_type = "Enum",
        enum_name = "final_fantasy.housing_type"
    )
)]
pub enum HousingType {
    #[default]
    #[cfg_attr(feature = "backend", sea_orm(string_value = "private"))]
    Private,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "free_company"))]
    FreeCompany,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "shared_apartment"))]
    SharedApartment,
}

impl HousingType {
    pub fn get_name(self) -> String {
        match self {
            HousingType::Private => "private",
            HousingType::FreeCompany => "free_company",
            HousingType::SharedApartment => "shared_appartment",
        }
        .to_string()
    }
}

impl ToString for HousingType {
    fn to_string(&self) -> String {
        match self {
            HousingType::Private => "Private Unterkunft",
            HousingType::FreeCompany => "Unterkunft einer Freien Gesellschaft",
            HousingType::SharedApartment => "Wohngemeinschaft",
        }
        .to_string()
    }
}

impl From<String> for HousingType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "private" => HousingType::Private,
            "free_company" => HousingType::FreeCompany,
            "shared_appartment" => HousingType::SharedApartment,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for HousingType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HousingType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_name().cmp(&other.get_name())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "character_housing", schema_name = "final_fantasy")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub district: HousingDistrict,
    pub housing_type: HousingType,
    pub ward: i16,
    pub plot: i16,
    pub character_id: i32,
}

impl PartialOrd for Model {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Model {
    fn cmp(&self, other: &Self) -> Ordering {
        self.district
            .cmp(&other.district)
            .then(self.ward.cmp(&other.ward))
            .then(self.plot.cmp(&other.plot))
    }
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::character::Entity",
        from = "Column::CharacterId",
        to = "super::character::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Character,
}

#[cfg(feature = "backend")]
impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Character.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(
        character_id: i32,
        district: HousingDistrict,
        housing_type: HousingType,
        ward: i16,
        plot: i16,
    ) -> Self {
        Self {
            id: i32::default(),
            district,
            housing_type,
            ward,
            plot,
            character_id,
        }
    }
}
