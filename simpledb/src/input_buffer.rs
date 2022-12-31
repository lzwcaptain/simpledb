
use std::io;

use crate::errors::DBError;
use crate::errors::InputError::ReadError;

#[derive(Clone, Debug, Default)]
pub(crate) struct InputBuffer {
    input_buffer: String,
    input_length: usize,
}

impl InputBuffer {
    pub(crate) fn read_input(&mut self) -> Result<(), DBError> {

        let stdin = io::stdin();
        let res = stdin.read_line(&mut self.input_buffer).map_err(|x| { DBError::InputError(ReadError(x.to_string())) })?;
        self.input_length = res;
        Ok(())
    }
    pub(crate) fn get_buffer(&self) -> &str {
        self.input_buffer.as_str()
    }
}