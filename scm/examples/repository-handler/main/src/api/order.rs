//! `Order` — the entity type stored in the injected `Repository<Order, String>`.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub item: String,
    pub quantity: u32,
}
