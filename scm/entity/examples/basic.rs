//! Basic `Entity` usage example.

use edge_application_entity::{Entity, EntityError, IdRequest, IdResponse};

struct OrderLine {
    id: u64,
    #[allow(dead_code)]
    quantity: u32,
}

impl Entity for OrderLine {
    type Id = u64;
    fn id(&self, _req: IdRequest) -> Result<IdResponse<'_, u64>, EntityError> {
        Ok(IdResponse { id: &self.id })
    }
}

fn main() -> Result<(), EntityError> {
    let line = OrderLine {
        id: 1,
        quantity: 10,
    };
    println!("order line id: {}", line.id(IdRequest)?.id);
    Ok(())
}
