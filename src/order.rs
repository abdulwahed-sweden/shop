//! Order model and admin configuration.

use rust_decimal::Decimal;
use rustio_admin::{Error, Model, ModelAdmin, Row, RustioAdmin, Value};

#[derive(RustioAdmin)]
pub struct Order {
    pub id: i64,
    pub customer_id: i64,
    pub total: Decimal,
    #[rustio(choices = ["pending", "paid", "shipped", "cancelled"])]
    pub status: String,
}

impl Model for Order {
    const TABLE: &'static str = "orders";
    const COLUMNS: &'static [&'static str] = &["id", "customer_id", "total", "status"];
    const INSERT_COLUMNS: &'static [&'static str] = &["customer_id", "total", "status"];

    fn id(&self) -> i64 {
        self.id
    }

    fn from_row(row: Row<'_>) -> Result<Self, Error> {
        Ok(Self {
            id: row.get_i64("id")?,
            customer_id: row.get_i64("customer_id")?,
            total: row.get_decimal("total")?,
            status: row.get_string("status")?,
        })
    }

    fn insert_values(&self) -> Vec<Value> {
        vec![
            self.customer_id.into(),
            self.total.into(),
            self.status.clone().into(),
        ]
    }
}

// Admin list-page configuration. Each method overrides a default.
impl ModelAdmin for Order {
    fn list_display() -> &'static [&'static str] {
        &["id", "customer_id", "total", "status"]
    }

    fn list_filter() -> &'static [&'static str] {
        &["status"]
    }

    fn search_fields() -> &'static [&'static str] {
        &[]
    }

    fn ordering() -> &'static [&'static str] {
        &["-id"]
    }
}
