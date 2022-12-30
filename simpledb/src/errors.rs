#[derive(Clone,Debug)]
pub(crate) enum DBError{
    InputError(InputError)
}
#[derive(Clone,Debug)]
pub(crate) enum InputError{
    ReadError(String),
    UnrecognizedError(String)
}