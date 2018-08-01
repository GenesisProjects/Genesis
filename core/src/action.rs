use common::address::Address;
///
///
///
#[derive(Debug, Clone)]
pub struct Action {
    pub balance: u32,
    pub addr: Address
}

#[derive(Debug)]
pub struct PermissionLevel {
    actor: String,
    permission: String
}