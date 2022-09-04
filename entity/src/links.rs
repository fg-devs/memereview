//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2



use sea_orm :: entity :: prelude :: * ; use super :: sea_orm_active_enums :: RestrictionType ;

# [derive (Clone , Debug , PartialEq , DeriveEntityModel)] # [sea_orm (table_name = "links")] pub struct Model { # [sea_orm (primary_key)] pub id : i32 , # [sea_orm (unique)] pub created_by : i64 , # [sea_orm (unique)] pub submissions_channel_id : i64 , pub review_channel_id : i64 , pub restriction_type : RestrictionType , }

# [derive (Copy , Clone , Debug , EnumIter , DeriveRelation)] pub enum Relation { # [sea_orm (belongs_to = "super::users::Entity" , from = "Column::CreatedBy" , to = "super::users::Column::Id" , on_update = "Cascade" , on_delete = "Cascade" ,)] Users , }

impl Related < super :: users :: Entity > for Entity { fn to () -> RelationDef { Relation :: Users . def () } }

impl ActiveModelBehavior for ActiveModel { }