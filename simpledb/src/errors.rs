#[derive(Clone,Debug)]
pub(crate) enum DBError{
    InputError(InputError),
    SerializeError(String),
    TableError(TableError)
}
#[derive(Clone,Debug)]
pub(crate) enum InputError{
    ReadError(String),
    UnrecognizedError(String)
}
#[derive(Clone,Debug)]
pub(crate) enum TableError{
    TableFullError,
    TableLackError,
    PageLackError,
    InternalError(String),
}