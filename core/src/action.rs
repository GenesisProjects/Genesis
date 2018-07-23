#[derive(Debug)]
pub struct PermissionLevel {
    actor: String,
    permission: String
}

#[derive(Debug)]
pub struct Action {
    account: String,
    action_anem: String,
    authorization: Vec<PermissionLevel>,
    data: String
}
