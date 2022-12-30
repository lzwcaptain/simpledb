use std::fmt::format;
use crate::errors::DBError;
use crate::errors::InputError::ReadError;
use crate::input_buffer::InputBuffer;
use crate::statement::PrepareResult::{Success, Unrecognized};
use crate::statement::StatementType::{Insert, Select};

pub(crate) enum PrepareResult {
    Success,
    Unrecognized,
}

#[derive(Copy, Clone)]
pub(crate) enum StatementType {
    Insert,
    Select,
}

#[derive(Clone,Default)]
pub(crate) struct Statement {
    type_: Option<StatementType>,
}

// impl Default for Statement {
//     fn default() -> Self {
//         Self { type_: None }
//     }
// }

impl Statement {
    pub(crate) fn prepare(&mut self, input_buffer: &InputBuffer) -> Result<PrepareResult, DBError> {
        if input_buffer.get_buffer().len()<6{
            Err(DBError::InputError(ReadError("less  than 6".to_string())))?;
        }
        if input_buffer.get_buffer()[..6].eq("insert") {
            self.type_ = Some(Insert);
            Ok(Success)
        } else if input_buffer.get_buffer()[..6].eq("select") {
            self.type_ = Some(Select);
            Ok(Success)
        } else {
            Ok(Unrecognized)
        }
    }
    pub(crate) fn execute_statement(&mut self) -> Result<(), DBError> {
        if let Some(type_) = self.type_ {
            match type_ {
                Insert => {
                    println!("insert ;");
                }
                Select => {
                    println!("select ;");
                }
                _ => {
                    unimplemented!();
                }
            }
        }else{
            println!("statement empty");
        }
        Ok(())
    }
}