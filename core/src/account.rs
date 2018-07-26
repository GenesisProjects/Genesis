///
///
///
#[derive(Debug, Clone)]
pub struct Account {
    balance: u32
}

impl Account {
    /// return the balance associated with this account.
    pub fn balance(&self) -> u32 { self.balance }
}