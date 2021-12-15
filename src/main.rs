#[derive(Debug)]
enum InstructionSet {
  PSH,
  ADD,
  POP,
  SET,
  HLT,
}

#[derive(Debug)]
enum Type {
  Op(InstructionSet),
  Number(i32),
}

fn fetch(program: &[Type], ip: usize) -> &Type {
  &program[ip]
}

fn eval(program: &[Type], stack: &mut [i32], ip: &mut i32, sp: &mut i32) -> bool {
  let instr = fetch(program, *ip as usize);
  match instr {
    Type::Op(InstructionSet::HLT) => false,
    Type::Op(InstructionSet::PSH) => {
      *ip += 1;
      let val = fetch(program, *ip as usize);
      match val {
        Type::Number(x) => {
          *sp += 1;
          stack[*sp as usize] = *x
        }
        _ => (),
      }
      true
    }
    Type::Op(InstructionSet::POP) => {
      let val = stack[*sp as usize];
      *sp -= 1;
      println!("popped {}", val);
      true
    }
    Type::Op(InstructionSet::ADD) => {
      let a = stack[*sp as usize];
      *sp -= 1;
      let b = stack[*sp as usize];
      let result = b + a;
      stack[*sp as usize] = result;
      true
    }
    _ => true,
  }
}

fn main() {
  let program = [
    Type::Op(InstructionSet::PSH),
    Type::Number(5),
    Type::Op(InstructionSet::PSH),
    Type::Number(6),
    Type::Op(InstructionSet::ADD),
    Type::Op(InstructionSet::PSH),
    Type::Number(6),
    Type::Op(InstructionSet::ADD),
    Type::Op(InstructionSet::POP),
    Type::Op(InstructionSet::HLT),
  ];

  let mut running = true;

  let mut ip: i32 = 0; // instruction pointer
  let mut sp: i32 = -1; // stack pointer
  let mut stack: [i32; 256] = [0; 256];
  while running {
    running = eval(&program, &mut stack, &mut ip, &mut sp);
    ip += 1;
  }
}
