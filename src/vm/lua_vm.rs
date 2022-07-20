use super::{
    binary_chunk::Prototype,
    instruction::{Instruction, InstructionOperation},
    lua_state::{LuaApi, LuaState, LuaVm},
    op_code::OpCodeEnum,
};

pub fn load_main(prototype: Prototype) {
    let n_regs = prototype.max_statck_size as i32;
    let mut state = LuaState::new((n_regs as usize) + 8, prototype);

    state.set_top(n_regs);
    loop {
        let pc = state.get_pc();
        let instruction: Instruction = state.fetch();
        let op_code = OpCodeEnum::try_from(instruction.op_code()).unwrap();
        match op_code {
            OpCodeEnum::OpReturn => break,
            _code => {
                instruction.execute(&mut state);
                println!("pc: {:?}, op name: {:?}", pc + 1, instruction.op_name());
            }
        }
    }
    println!("{:#?}", state.stack);
}

#[cfg(test)]
mod tests {
    use crate::vm::reader::LuaChunkReader;

    use super::*;

    fn read_prototype_fixture(filename: &'static str) -> Prototype {
        let cur_dir = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let chunk_file = cur_dir + "/fixtures/" + filename;
        let file = std::fs::File::open(chunk_file).unwrap();
        let mut buf = Vec::new();
        std::io::Read::read_to_end(&mut std::io::BufReader::new(file), &mut buf);

        let mut reader = LuaChunkReader::new(buf);

        reader.check_header();
        reader.read_byte();
        let proto = reader.read_function_prototype("".to_string()).unwrap();
        proto
    }

    #[test]
    fn test_move_instruction() {
        let proto = read_prototype_fixture("loop.luac");
        // 1       [1]     VARARGPREP      0
        // 2       [2]     JMP             -1      ; to 2
        // 3       [2]     RETURN          0 1 1   ; 0 out
        load_main(proto);
    }

    #[test]
    fn test_declare_a_variable() {
        let proto = read_prototype_fixture("var.luac");
        // 1       [1]     VARARGPREP      0
        // 2       [1]     LOADNIL         0 1     ; 2 out
        // 3       [1]     RETURN          2 1 1   ; 0 out
        load_main(proto);
    }

    #[test]
    fn test_add_2_integer_program() {
        let proto = read_prototype_fixture("add-2-int.luac");
        // 1       [1]     VARARGPREP      0
        // 2       [1]     LOADI           0 1
        // 3       [1]     LOADI           1 2
        // 4       [1]     LOADNIL         2 0     ; 1 out
        // 5       [1]     ADD             2 0 1
        // 6       [1]     MMBIN           0 1 6   ; __add
        // 7       [1]     RETURN          3 1 1   ; 0 out
        load_main(proto);
    }

    #[test]
    fn test_add_2_float_program() {
        let proto = read_prototype_fixture("add-2-float.luac");
        // 1       [1]     VARARGPREP      0
        // 2       [1]     LOADK           0 0     ; 2.2
        // 3       [1]     LOADK           1 1     ; 3.3
        // 4       [1]     LOADNIL         2 0     ; 1 out
        // 5       [2]     ADD             2 0 1
        // 6       [2]     MMBIN           0 1 6   ; __add
        // 7       [2]     RETURN          3 1 1   ; 0 out
        load_main(proto);
    }

    #[test]
    fn test_len_instruction() {
        let proto = read_prototype_fixture("len.luac");
        // 1       [1]     VARARGPREP      0
        // 2       [1]     LOADK           0 0     ; "123"
        // 3       [1]     LOADNIL         1 0     ; 1 out
        // 4       [2]     LEN             1 0
        // 5       [2]     RETURN          2 1 1   ; 0 out
        load_main(proto);
    }

    #[test]
    fn test_concat_instruction() {
        let proto = read_prototype_fixture("concat.luac");

        // 1       [1]     VARARGPREP      0
        // 2       [1]     LOADK           0 0     ; "str"
        // 3       [1]     LOADNIL         1 0     ; 1 out
        // 4       [2]     MOVE            2 0
        // 5       [2]     MOVE            3 0
        // 6       [2]     CONCAT          2 2
        // 7       [2]     MOVE            1 2
        // 8       [2]     RETURN          2 1 1   ; 0 out
        load_main(proto);
    }

    #[test]
    fn test_sum_of_even_number_in_100() {
        let proto = read_prototype_fixture("even-number-in-100.luac");
        //    1       [1]     VARARGPREP      0
        //     2       [1]     LOADI           0 0
        //     3       [2]     LOADI           1 5
        //     4       [2]     LOADI           2 100
        //     5       [2]     LOADI           3 1
        //     6       [2]     FORPREP         1 6     ; exit to 14
        //     7       [3]     MODK            5 4 0   ; 3
        //     8       [3]     MMBINK          4 0 9 0 ; __mod 3
        //     9       [3]     EQI             5 0 0
        //     10      [3]     JMP             2       ; to 13
        //     11      [4]     ADD             0 0 4
        //     12      [4]     MMBIN           0 4 6   ; __add
        //     13      [2]     FORLOOP         1 7     ; to 7
        //     14      [6]     RETURN          1 1 1   ; 0 out
        load_main(proto);
    }
}
