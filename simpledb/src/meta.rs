use crate::input_buffer::InputBuffer;

pub(crate) enum MetaCommandResult{
    SUCCESS,
    UNRECOGNIZED
}

pub(crate) fn do_mata_command(input_buffer:& InputBuffer)->MetaCommandResult{
    if input_buffer.get_buffer().trim()==".exit"{
        println!("exit success");
        MetaCommandResult::SUCCESS
    }else{
        println!("unrecognized");
        MetaCommandResult::UNRECOGNIZED
    }
}