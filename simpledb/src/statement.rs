use std::borrow::BorrowMut;


use std::mem::size_of;


use text_io::{scan};
use crate::errors::{DBError, TableError};
use crate::errors::InputError::ReadError;

use crate::input_buffer::InputBuffer;
use crate::statement::PrepareResult::{Success, Unrecognized};
use crate::statement::StatementType::{Insert, Select};
use crate::table::{Table, TABLE_MAX_ROWS};

pub(crate) enum PrepareResult {
    Success,
    Unrecognized,
}

#[derive(Copy, Clone)]
pub(crate) enum StatementType {
    Insert,
    Select,
}

#[derive(Clone, Default)]
pub(crate) struct Statement {
    type_: Option<StatementType>,
    row: Row,
}

#[repr(C)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Row {
    pub(crate) id: u32,
    pub(crate) username: [u8; Row::COLUMN_USERNAME_SIZE],
    pub(crate) email: [u8; Row::COLUMN_EMAIL_SIZE],
}

impl Default for Row {
    fn default() -> Self {
        Self {
            id: 0,
            username: (0..Row::COLUMN_USERNAME_SIZE).map(|_| 0).collect::<Vec<_>>().try_into().unwrap(),
            email: (0..Row::COLUMN_EMAIL_SIZE).map(|_| 0).collect::<Vec<_>>().try_into().unwrap(),
        }
    }
}

// impl Default for Statement {
//     fn default() -> Self {
//         Self { type_: None }
//     }
// }

impl Statement {
    pub(crate) fn prepare(&mut self, input_buffer: &InputBuffer) -> Result<PrepareResult, DBError> {
        if input_buffer.get_buffer().len() < 6 {
            Err(DBError::InputError(ReadError("less  than 6".to_string())))?;
        }
        if input_buffer.get_buffer()[..6].eq("insert") {
            self.type_ = Some(Insert);
            let mut username = "".to_string();
            let mut email = "".to_string();
            scan!("select {} {} {}",self.row.id,username,email);
            self.row.username.copy_from_slice(username.as_bytes());
            self.row.email.copy_from_slice(email.as_bytes());
            Ok(Success)
        } else if input_buffer.get_buffer()[..6].eq("select") {
            self.type_ = Some(Select);
            Ok(Success)
        } else {
            Ok(Unrecognized)
        }
    }
    pub(crate) fn row_buffer() -> [u8; Row::ROW_SIZE] {
        return [0; Row::ROW_SIZE];
    }
    pub(crate) fn execute_statement(&mut self,table:&mut Table) -> Result<(), DBError> {
        if let Some(type_) = self.type_ {
            match type_ {
                Insert => {
                    self.execute_insert(self.)
                }
                Select => {
                    println!("select ;");
                }
                _ => {
                    unimplemented!();
                }
            }
        } else {
            println!("statement empty");
        }
        Ok(())
    }
    pub(crate) fn execute_insert(&mut self, table: &mut Table) -> Result<(), DBError> {
        if table.pages.len() >= TABLE_MAX_ROWS {
            Err(DBError::TableError(TableError::TableFullError))?;
        }
        // let mut buffer = Self::row_buffer();
        // self.row.serialize(table.pages.last()
        //     .ok_or(DBError::TableError(TableError::TableLackError))?
        //     .ok_or(DBError::TableError(PageLackError))?
        //     .get_data().lock()
        //     .map(|x|x.deref_mut().try_into().map_err(|_|DBError::TableError(TableError::InternalError("try_into error".to_string()))))?
        //     .map_err(|_|DBError::TableError(TableError::InternalError("lcck get error".to_string())))?);
        table.row_slot(table.num_rows, |x| self.row.serialize(x));
        // self.row.serialize(table.row_slot(table.num_rows)?);
        table.num_rows += 1;
        Ok(())
    }
    pub(crate) fn execute_select(&mut self, table: &mut Table) -> Result<(), DBError> {
        for idx in 0..table.num_rows {
            let page = &table.pages[idx];
            if let Some(_page) = page {
                // let row = Row::deserialize(table.row_slot(idx)?)?;
                let mut row = Row::default();
                table.row_slot(idx, |x| { Row::deserialize(x, row.borrow_mut()); });
                row.print();
            }
        }
        Ok(())
    }
}

impl Row {
    const COLUMN_USERNAME_SIZE: usize = 32;
    const COLUMN_EMAIL_SIZE: usize = 256;
    const COLUMN_ID_SIZE: usize = size_of::<u32>();
    const ID_OFFSET: usize = 0;
    const USERNAME_OFFSET: usize = Self::ID_OFFSET + Self::COLUMN_ID_SIZE;
    const EMAIL_OFFSET: usize = Self::USERNAME_OFFSET + Self::COLUMN_USERNAME_SIZE;
    pub(crate) const ROW_SIZE: usize = Self::COLUMN_USERNAME_SIZE + Self::COLUMN_EMAIL_SIZE + Self::COLUMN_ID_SIZE;

    fn serialize(&self, dst: &mut [u8; Self::ROW_SIZE]) {
        dst[Self::ID_OFFSET..Self::USERNAME_OFFSET].copy_from_slice(self.id.to_be_bytes().as_slice());
        dst[Self::USERNAME_OFFSET..Self::EMAIL_OFFSET].copy_from_slice(self.username.as_slice());
        dst[Self::EMAIL_OFFSET..].copy_from_slice(self.email.as_slice());
    }
    fn deserialize(from: &mut [u8; Self::ROW_SIZE], row: &mut Self) -> Result<(), DBError> {
        *row = Self {
            id: u32::from_be_bytes(from[Self::ID_OFFSET..Self::USERNAME_OFFSET].try_into().map_err(|_| DBError::SerializeError("deserialize error".to_string()))?),
            username: from[Self::USERNAME_OFFSET..Self::EMAIL_OFFSET].try_into().map_err(|_| DBError::SerializeError("deserialize error".to_string()))?,
            email: from[Self::EMAIL_OFFSET..].try_into().map_err(|_| DBError::SerializeError("deserialize error".to_string()))?,
        };
        Ok(())
    }
    pub(crate) fn print(&self) -> Result<(), DBError> {
        println!("({},{}, {})", self.id, String::from_utf8(self.username.to_vec()).map_err(|_| DBError::SerializeError("can't build from utf8".to_string()))?, String::from_utf8(self.email.to_vec()).map_err(|_| DBError::SerializeError("can't build from utf8".to_string()))?);
        Ok(())
    }
}

mod tests {
    use std::borrow::BorrowMut;
    use rand::Rng;
    use crate::statement::Row;

    fn gen_row() -> Row {
        let mut rng = rand::thread_rng();
        let email = (0..Row::COLUMN_EMAIL_SIZE).map(|_| rng.gen()).collect::<Vec<_>>().try_into().unwrap();
        Row {
            id: rng.gen(),
            username: rng.gen(),
            email: email,
        }
    }

    #[test]
    fn test_serialize() {
        struct Test {
            row: Row,
            buffer: [u8; Row::ROW_SIZE],
        }
        for _ in 1..10 {
            let row = gen_row();
            let mut buffer = [0; Row::ROW_SIZE];
            row.serialize(&mut buffer);
            let mut testRow = Row::default();
            Row::deserialize(&mut buffer, testRow.borrow_mut()).unwrap();
            assert_eq!(row, testRow);
        }
    }
}
