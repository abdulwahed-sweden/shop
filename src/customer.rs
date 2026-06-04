//! Customer model and admin configuration.

use rustio_admin::{Error, Model, ModelAdmin, Row, RustioAdmin, Value};

#[derive(RustioAdmin)]
pub struct Customer {
    pub id: i64,
    pub full_name: String,
    #[rustio(format = "email")]
    pub email: String,
    #[rustio(format = "phone")]
    pub phone: String,
}

impl Model for Customer {
    const TABLE: &'static str = "customers";
    const COLUMNS: &'static [&'static str] = &["id", "full_name", "email", "phone"];
    const INSERT_COLUMNS: &'static [&'static str] = &["full_name", "email", "phone"];

    fn id(&self) -> i64 {
        self.id
    }

    fn from_row(row: Row<'_>) -> Result<Self, Error> {
        Ok(Self {
            id: row.get_i64("id")?,
            full_name: row.get_string("full_name")?,
            email: row.get_string("email")?,
            phone: row.get_string("phone")?,
        })
    }

    fn insert_values(&self) -> Vec<Value> {
        vec![
            self.full_name.clone().into(),
            self.email.clone().into(),
            self.phone.clone().into(),
        ]
    }
}

// Admin list-page configuration. Each method overrides a default.
impl ModelAdmin for Customer {
    fn list_display() -> &'static [&'static str] {
        &["id", "full_name", "email", "phone"]
    }

    fn list_filter() -> &'static [&'static str] {
        &[]
    }

    fn search_fields() -> &'static [&'static str] {
        &["full_name", "email", "phone"]
    }

    fn ordering() -> &'static [&'static str] {
        &["-id"]
    }
}
