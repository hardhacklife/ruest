//! AST du fichier `schema.forge` (DSL type Prisma).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub models: Vec<Model>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub kind: FieldKind,
    pub optional: bool,
    pub list: bool,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldKind {
    Scalar(ScalarType),
    Model(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScalarType {
    String,
    Int,
    Float,
    Boolean,
    DateTime,
    Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Attribute {
    Id,
    Unique,
    Default(DefaultValue),
    Relation(RelationAttr),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefaultValue {
    Uuid,
    Now,
    Literal(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationAttr {
    pub fields: Vec<String>,
    pub references: Vec<String>,
}

impl Schema {
    pub fn model(&self, name: &str) -> Option<&Model> {
        self.models.iter().find(|m| m.name == name)
    }
}

impl Model {
    pub fn scalar_fields(&self) -> impl Iterator<Item = &Field> {
        self.fields
            .iter()
            .filter(|f| matches!(f.kind, FieldKind::Scalar(_)) && !f.list)
    }

    pub fn id_field(&self) -> Option<&Field> {
        self.fields
            .iter()
            .find(|f| f.attributes.iter().any(|a| matches!(a, Attribute::Id)))
    }
}

impl Field {
    pub fn is_relation_list(&self) -> bool {
        matches!(self.kind, FieldKind::Model(_)) && self.list
    }

    pub fn is_relation_scalar(&self) -> bool {
        matches!(self.kind, FieldKind::Model(_))
            && !self.list
            && self
                .attributes
                .iter()
                .any(|a| matches!(a, Attribute::Relation(_)))
    }
}
