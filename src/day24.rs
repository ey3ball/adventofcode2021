use dynasmrt::{dynasm, DynasmApi};
use std::mem;

#[aoc_generator(day24)]
pub fn parse(input: &str) -> Vec<Program> {
    input
        .replace("inp", "#inp")
        .split("#")
        .map(|code_block| {
            Program::translate(code_block)
        })
        .collect()
}

#[aoc(day24, part1)]
pub fn part1(input: &[Program]) -> usize {
    let mut state: State = (0, 0, 0, 0);
    input[0].run(&mut state, 0);
    println!("{:?}", state);
    0
}

macro_rules! aluasm {
    ($ops:ident $($t:tt)*) => {
        dynasm!($ops
            ; .arch x64
            ; .alias w, r8
            ; .alias x, r9
            ; .alias y, r10
            ; .alias z, r11
            ; .alias _tmp, r12
            ; .alias state, rdi
            ; .alias _in, rsi
            $($t)*
        )
    }
}

type State = (i64, i64, i64, i64);

pub struct Program {
    code: dynasmrt::ExecutableBuffer,
    start: dynasmrt::AssemblyOffset,
}

impl Program {
    // Map register characters (x,y,z) to hardware register
    // Dynamic encoding is used
    fn reg_dyn(regname: &str) -> u8 {
        match regname {
            "w" => 8,
            "x" => 9,
            "y" => 10,
            "z" => 11,
            _ => panic!("Unsupported regname")
        }
    }

    fn is_imm(regname: &str) -> bool {
        !(regname == "w" || regname == "x" || regname == "y" || regname == "z")
    }

    fn translate(input: &str) -> Program {
        let mut ops = dynasmrt::x64::Assembler::new().unwrap();

        let start = ops.offset();

        // Prologue
        aluasm!(ops
            ; mov w, [state]
            ; mov x, [state + 8]
            ; mov y, [state + 16]
            ; mov z, [state + 24]
        );

        input
            .lines()
            .for_each(|l| {
                let op: Vec<&str> = l.split(' ').collect();
                match op[0] {
                    "inp" => {
                        aluasm!(ops ; mov Rq(Self::reg_dyn(op[1])), _in);
                    }
                    "mul" => {
                        if Self::is_imm(op[2]) {
                            let v: i32 = op[2].parse().unwrap();
                            aluasm!(ops
                                ; imul Rq(Self::reg_dyn(op[1])), Rq(Self::reg_dyn(op[1])), v
                            );
                        } else {
                            aluasm!(ops
                                ; imul Rq(Self::reg_dyn(op[1])), Rq(Self::reg_dyn(op[2]))
                            );
                        }
                    }
                    "div" | "mod" => {
                        if !Self::is_imm(op[2]) {
                            panic!("Expected immediate divisor");
                        }
                        let v: i32 = op[2].parse().unwrap();

                        aluasm!(ops
                            ; mov rax, Rq(Self::reg_dyn(op[1]))
                            ; cqo
                            ; mov _tmp, v
                            ; div _tmp
                        );

                        if op[0] == "div" {
                            aluasm!(ops ; mov Rq(Self::reg_dyn(op[1])), rax)
                        } else {
                            aluasm!(ops ; mov Rq(Self::reg_dyn(op[1])), rdx)
                        }
                    }
                    "add" => {
                        if Self::is_imm(op[2]) {
                            let v: i32 = op[2].parse().unwrap();
                            aluasm!(ops
                                ; add Rq(Self::reg_dyn(op[1])), v
                            );
                        } else {
                            aluasm!(ops
                                ; add Rq(Self::reg_dyn(op[1])), Rq(Self::reg_dyn(op[2]))
                            );
                        }
                    }
                    "eql" => {
                        if Self::is_imm(op[2]) {
                            let v: i32 = op[2].parse().unwrap();
                            aluasm!(ops
                                ; cmp Rq(Self::reg_dyn(op[1])), v
                            );
                        } else {
                            aluasm!(ops
                                ; cmp Rq(Self::reg_dyn(op[1])), Rq(Self::reg_dyn(op[2]))
                            );
                        }
                        aluasm!(ops
                            ; mov Rq(Self::reg_dyn(op[1])), 0
                            ; sete Rb(Self::reg_dyn(op[1]))
                        );
                    }
                    _ => panic!("Unsupported instruction {}", op[0])
                }
            });

        // Epilogue
        aluasm!(ops
            ; mov [state], w
            ; mov [state + 8], x
            ; mov [state + 16], y
            ; mov [state + 24], z
            // ; mov rax, _in // debug
            ; ret
        );

        let code = ops.finalize().unwrap();
        Program {
            code,
            start
        }
    }

    fn run(&self, s: &mut State, i: i64) -> u64 {
        let f: extern "sysv64" fn(&mut State, i64) -> u64 = unsafe { mem::transmute(self.code.ptr(self.start)) };
        f(s, i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playground() {
        let program = playground();
        let mut s: State = (0, 0, 0, 0);
        let a = program.run(&mut s, 0);
        println!("{}", a);
        println!("{:?}", s);
        assert_eq!(s, (0, 1, 2, 3))
    }

    fn playground() -> Program {
        let mut ops = dynasmrt::x64::Assembler::new().unwrap();

        let start = ops.offset();
        println!("Gen test sequences");
        aluasm!(ops
            ; mov w, [state]
            ; mov x, [state + 8]
            ; mov y, [state + 16]
            ; mov z, [state + 24]
        );

        let v: i32 = 2;
        aluasm!(ops
            ; add w, 0
            ; add x, 1
            ; add Rq(10), v
            ; add z, 3
        );

        aluasm!(ops
            ; mov [state], w
            ; mov [state + 8], x
            ; mov [state + 16], y
            ; mov [state + 24], z
            ; mov rax, _in
            ; ret
        );

        let code = ops.finalize().unwrap();
        Program {
            code,
            start,
        }
    }

    #[test]
    fn test_basic_state() {
        let input = "\
inp x\n\
mul x -1\n\
mul z x\n\
add y 2\n\
mul x y\n";

        let program = Program::translate(input);

        let mut s: State = (0, 0, 0, 0);
        program.run(&mut s, 2);
        assert_eq!(s, (0, -4, 2, 0));
    }

    #[test]
    fn test_eql() {
        let input = "\
mul z 3\n\
eql z x\n";

        let program = Program::translate(input);

        let mut s: State = (0, 30, 0, 10);
        program.run(&mut s, 0);

        assert_eq!(s, (0, 30, 0, 1));
    }

    #[test]
    fn test_div() {
        let input = "\
inp w\n\
add z w\n\
mod z 2\n\
div w 2\n\
add y w\n\
mod y 2\n\
div w 2\n\
add x w\n\
mod x 2\n\
div w 2\n\
mod w 2\n";

        let program = Program::translate(input);

        let mut s: State = (0, 0, 0, 0);
        program.run(&mut s, 6);
        assert_eq!(s, (0, 1, 1, 0));
    }
}
