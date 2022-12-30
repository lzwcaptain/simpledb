use std::io::{stdin, stdout, Write};
use crate::input_buffer::InputBuffer;
use crate::meta::do_mata_command;
use crate::statement::Statement;

mod statement;
mod table;
mod meta;
mod input_buffer;
mod errors;


fn main() {
    loop {
        println!("db > ");
        let mut input_buffer = InputBuffer::default();
        input_buffer.read_input().expect("read error");
        if input_buffer.get_buffer().chars().nth(0).unwrap() == '.' {
            match do_mata_command(&input_buffer) {
                Success => continue,
                Unrecognized => {
                    println!("unrecognized");
                    continue;
                }
            }
        }
        let mut statement = Statement::default();
        match statement.prepare(&input_buffer).unwrap() {
            Success => {
                statement.execute_statement().expect("statement error");
                println!("executed");
            }
            Unrecognized => {
                println!("unrecognize");
                continue;
            }
        }
    }
}
