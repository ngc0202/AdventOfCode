use super::Solution;
use crate::utils::NomFail;
use itertools::Itertools;

day!(run 17);

type Reg = u64;

#[derive(Debug)]
struct Day17 {
    program: Vec<u8>,
    regs: [Reg; 3],
    ptr: usize,
    out: Vec<u8>,
}

impl<'i> Solution<'i> for Day17 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, NomFail> {
        parse::parse(input)
    }

    fn part1(&mut self) -> String {
        self.execute();
        self.out.iter().join(",")
    }

    fn part2(&mut self) -> u64 {
        let cons = part2::find_constraints(&self.program);
        part2::solve_constraints(&cons).expect("No solution found")
    }
}

mod part2 {
    #[derive(Default)]
    pub struct Constraint {
        pub val: u64,
        pub mask: u64,
    }

    impl Constraint {
        pub fn is_compat(&self, other: &Self) -> bool {
            ((self.val ^ other.val) & self.mask & other.mask) == 0
        }

        pub fn combine(&self, other: &Self) -> Option<Self> {
            self.is_compat(other).then(|| Constraint {
                val: self.val | other.val,
                mask: self.mask | other.mask,
            })
        }
    }

    type ConList = Box<[Constraint]>;

    pub fn solve_constraints(cons: &[ConList]) -> Option<u64> {
        solve_r(cons, Constraint::default())
    }

    fn solve_r(cons: &[ConList], combo: Constraint) -> Option<u64> {
        let Some((cur, cons)) = cons.split_first() else {
            return Some(combo.val);
        };

        cur.iter()
            .find_map(|con| solve_r(cons, con.combine(&combo)?))
    }

    pub fn find_constraints(prog: &[u8]) -> Box<[ConList]> {
        // Initialize with length constraint
        let mut cons: Vec<ConList> = Vec::with_capacity(prog.len() + 1);
        let maxbit = 3 * prog.len();
        cons.push(Box::new([Constraint {
            val: 0,
            mask: !((1u64 << maxbit) - 1),
        }]));

        // Collect constraints from program
        let con_iter = prog.into_iter().enumerate().map(|(i, &o)| {
            (0u64..8)
                .flat_map(|mut bval| {
                    let ashift = (3 * i as u64) + (bval ^ 1);
                    let amask = 7u64 << ashift;
                    let aval = (u64::from(o) ^ bval ^ 5) << ashift;
                    let bmask = 7u64 << (3 * i);
                    bval <<= 3 * i;
                    let a = Constraint {
                        val: aval,
                        mask: amask,
                    };
                    let b = Constraint {
                        val: bval,
                        mask: bmask,
                    };
                    a.combine(&b)
                })
                .collect()
        });

        cons.extend(con_iter);
        cons.into_boxed_slice()
    }
}

impl Day17 {
    pub fn new(rega: Reg, regb: Reg, regc: Reg, program: Vec<u8>) -> Self {
        Self {
            program,
            regs: [rega, regb, regc],
            ptr: 0,
            out: Vec::new(),
        }
    }

    pub fn execute(&mut self) {
        self.ptr = 0;
        self.out.clear();
        let regs = self.regs;

        while let Some(&[op, val]) = self.program.get(self.ptr..self.ptr + 2) {
            self.ptr += 2;
            self.do_op(op, val);
        }

        self.regs = regs;
    }

    pub fn do_op(&mut self, op: u8, val: u8) {
        match op {
            // adv - division on A
            0 => self.regs[0] = self.div(val),

            // bxl - xor on B
            1 => self.regs[1] ^= Reg::from(val),

            // bst - B = combo % 8
            2 => self.regs[2] = self.combo(val) % 8,

            // jnz - jump to val if A!=0
            3 => {
                if self.regs[0] != 0 {
                    self.ptr = usize::from(val);
                }
            }

            // bxc - B ^= C
            4 => self.regs[1] ^= self.regs[2],

            // out - outputs combo % 8
            5 => self.out.push((self.combo(val) % 8) as u8),

            // bdv - division on B
            6 => self.regs[1] = self.div(val),

            // cdv - division on C
            7 => self.regs[2] = self.div(val),

            8.. => panic!("Invalid opcode {op}"),
        }
    }

    fn combo(&self, val: u8) -> Reg {
        match val {
            0..=3 => Reg::from(val),
            4 => self.regs[0],
            5 => self.regs[1],
            6 => self.regs[2],
            7.. => panic!("Invalid operand {val}"),
        }
    }

    fn div(&self, val: u8) -> u64 {
        self.combo(val)
            .try_into()
            .ok()
            .and_then(|c| self.regs[0].checked_shr(c))
            .unwrap_or(0)
    }
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, u8},
        combinator::all_consuming,
        multi::separated_list1,
        sequence::{preceded, tuple},
        Finish, IResult,
    };

    use super::{Day17, Reg};
    use crate::utils::{
        parser::{line, NomInt},
        NomFail,
    };

    fn reg(id: u8) -> impl FnMut(&[u8]) -> IResult<&[u8], Reg> {
        move |input: &[u8]| {
            line(preceded(
                tuple((tag("Register "), tag([id]), tag(": "))),
                Reg::parse_int,
            ))(input)
        }
    }

    fn prog(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
        line(preceded(tag("Program: "), separated_list1(tag(","), u8)))(input)
    }

    pub fn parse(input: &[u8]) -> Result<Day17, NomFail> {
        let (a, b, c, _, prog) =
            all_consuming(tuple((reg(b'A'), reg(b'B'), reg(b'C'), line_ending, prog)))(input)
                .finish()?
                .1;

        Ok(Day17::new(a, b, c, prog))
    }
}
