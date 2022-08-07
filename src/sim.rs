
// LEXOME
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Lexome {
    NopA,
    NopB,
    NopC,
    IfNEqu,
    IfLess,
    Pop,
    Push,
    SwapStk,
    Swap,
    ShiftR,
    ShiftL,
    Inc,
    Dec,
    Add,
    Sub,
    Nand,
    IO,
    HAlloc,
    HDivide,
    HCopy,
    HSearch,
    MovHead,
    JmpHead,
    GetHead,
    IfLabel,
    SetFlow,
}

impl Lexome {
    // TODO Convert Verbose to short form and vice versa.

}

// FINCH
pub struct Finch {
    // CPU
    pub lexome: Vec<Lexome>,
    pub inst_h: usize,
    pub read_h: usize,
    pub writ_h: usize,
    pub flow_h: usize,
    pub regi_1: u32,
    pub regi_2: u32,
    pub regi_3: u32,
    pub stac_1: Vec<u32>,
    pub stac_2: Vec<u32>,
    pub i_buff: u32,
    pub o_buff: u32,

    // Info
    pub x_loc: usize,
    pub y_loc: usize,
    pub age: u128,
    pub inputs: Vec<u32>,
}
impl Finch {
    fn current_inst(&self) -> Lexome {
        self.lexome[self.inst_h]
    }
    fn inc_ih(&mut self) {
        if self.inst_h + 1 == self.lexome.len() {
            self.inst_h = 0
        }
        else {
            self.inst_h += 1
        }

    }
    fn iter(&mut self) -> Option<Finch> {
        return match self.execute() {
            None => {
                self.inc_ih();
                self.age += 1;
                None
            }
            Some(f) => {
                self.inc_ih();
                self.age += 1;
                Some(f)
            }
        }
    }
    fn execute(&mut self) -> Option<Finch> {
        return match self.current_inst() {
            Lexome::NopA => { None }
            Lexome::NopB => { None }
            Lexome::NopC => { None }
            Lexome::IfNEqu => {}
            Lexome::IfLess => {}
            Lexome::Pop => {}
            Lexome::Push => {}
            Lexome::SwapStk => {}
            Lexome::Swap => {}
            Lexome::ShiftR => {}
            Lexome::ShiftL => {}
            Lexome::Inc => {}
            Lexome::Dec => {}
            Lexome::Add => {}
            Lexome::Sub => {}
            Lexome::Nand => {}
            Lexome::IO => {}
            Lexome::HAlloc => {}
            Lexome::HDivide => {}
            Lexome::HCopy => {}
            Lexome::HSearch => {}
            Lexome::MovHead => {}
            Lexome::JmpHead => {}
            Lexome::GetHead => {}
            Lexome::IfLabel => {}
            Lexome::SetFlow => {}
        }
    }
}

pub struct Sim {
    pub space: Vec<Finch>,
    pub cycle: u128,
    pub new_finches: Vec<Finch>,
}


pub fn run_sim(cycles: u8, sim: &mut Sim) -> () {
    for _ in 0..cycles {
        // Update finch
        for i in 0..sim.space.len() {
            match sim.space[i].iter() {
                None => (),
                Some(finch) => sim.new_finches.push(finch),
            };
        }
        // TODO Add finches to new locations.
        for i in 0..sim.new_finches.len() {

        };
        sim.new_finches.clear();
    }
    sim.cycle += u128::from(cycles);
}