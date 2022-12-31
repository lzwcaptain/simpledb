

use std::sync::{Arc, Mutex};
use crate::errors::{DBError, TableError};
use crate::statement::{Row};
use crate::page::{Page, PAGE_SIZE};

pub(crate) const TABLE_MAX_PAGES: usize = 100;
pub(crate) const ROWS_PER_PAGE: usize = 4096;
pub(crate) const TABLE_MAX_ROWS: usize = 4096;

pub(crate) struct Table {
    pub(crate) pages: [Option<Page>; TABLE_MAX_PAGES],
    pub(crate) num_rows: usize,
}

impl Table {
    pub(crate) fn row_slot<F>(&mut self, row_num: usize, mut operation: F) -> Result<(), DBError>
        where F: FnMut(&mut [u8; Row::ROW_SIZE])
    {
        let page_num = row_num / ROWS_PER_PAGE;
        if self.pages[page_num].is_none() {
            self.pages[page_num] = Some(Page { data: Arc::new(Mutex::new([0; PAGE_SIZE])) });
        }
        let page = self.pages[page_num].as_mut().unwrap();
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * Row::ROW_SIZE;
        let offset = row_num + byte_offset;
        let data = page.get_data();
        if let Ok(mut guard) = data.lock() {
            (guard[offset..offset + Row::ROW_SIZE].as_mut())
                .try_into()
                .map(|x| operation(x))
                .map_err(|_| DBError::TableError(TableError::InternalError("table slice error".to_string())))?
        } else {
            Err(DBError::TableError(TableError::InternalError("lock get error".to_string())))?;
        }
        Ok(())
    }
}

