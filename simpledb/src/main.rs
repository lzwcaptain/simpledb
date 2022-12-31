#[macro_use]
extern crate text_io;

use crate::input_buffer::InputBuffer;
use crate::meta::do_mata_command;
use crate::statement::Statement;

mod statement;
mod table;
mod meta;
mod input_buffer;
mod errors;
mod page;


fn main() {
    loop {
        println!("db > ");
        let mut input_buffer = InputBuffer::default();
        input_buffer.read_input().expect("read error");
        if input_buffer.get_buffer().chars().nth(0).unwrap() == '.' {
            match do_mata_command(&input_buffer) {
                _Success => continue,
                _Unrecognized => {
                    println!("unrecognized");
                    continue;
                }
            }
        }
        let mut statement = Statement::default();
        match statement.prepare(&input_buffer).unwrap() {
            _Success => {
                statement.execute_statement().expect("statement error");
                println!("executed");
            }
            _Unrecognized => {
                println!("unrecognize");
                continue;
            }
        }
    }
}
