pub enum Op {

}

pub struct RedoLog {
    op: Op,
}

pub struct Pipeline {
    logs: Vec<RedoLog>
}