use rand::{
    rngs::ThreadRng,
    seq::SliceRandom,
};

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
#[derive(PartialEq)]
#[derive(Clone)]
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

    fn iter_step(&mut self) -> Option<Finch> {
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
            Lexome::IfNEqu => { None }
            Lexome::IfLess => { None }
            Lexome::Pop => { None }
            Lexome::Push => { None }
            Lexome::SwapStk => { None }
            Lexome::Swap => { None }
            Lexome::ShiftR => { None }
            Lexome::ShiftL => { None }
            Lexome::Inc => { None }
            Lexome::Dec => { None }
            Lexome::Add => { None }
            Lexome::Sub => { None }
            Lexome::Nand => { None }
            Lexome::IO => { None }
            Lexome::HAlloc => { None }
            Lexome::HDivide => { None }
            Lexome::HCopy => { None }
            Lexome::HSearch => { None }
            Lexome::MovHead => { None }
            Lexome::JmpHead => { None }
            Lexome::GetHead => { None }
            Lexome::IfLabel => { None }
            Lexome::SetFlow => { None }
        }
    }
}

pub struct Sim {
    pub space: Vec<Option<Finch>>,
    pub cycle: u128,
    pub new_finches: Vec<Finch>,
    pub rng: ThreadRng,
}
impl Sim {
    pub fn count_pop(&self) -> u128 {
        let mut counter: u128 = 0;
        for x in 0..self.space.len() {
            if self.space[x] != None {
                counter += 1;
            }
        }
        counter
    }
}

// Entry point into simulation.
pub fn run_sim(cycles: u8, sim: &mut Sim) -> () {
    for _ in 0..cycles {
        // Update finch
        for i in 0..sim.space.len() {
            let finch: &mut Option<Finch> = sim.space.get_mut(i).unwrap();
            match finch {
                None => (),
                Some(a) => {
                    match a.iter_step() {
                        None => (),
                        Some(b) => sim.new_finches.push(b)
                    };
                }
            }
        }
        // RNG find a random empty space
        let mut directions:Vec<u8> = vec![0,1,2,3,4,5,6,7];
        for i in 0..sim.new_finches.len() {
            directions.shuffle(&mut sim.rng);
            let mut flag: bool = false;
            for j in 0..directions.len() {
                if match_direct(sim,&i,&directions[j]) {
                    change_direct(sim,&i,&directions[j]);
                    flag = true;
                    break;
                }
            }
            // Override an existing finch.
            if !flag { change_direct(sim,&i,&directions[0]) }
            sim.space[sim.new_finches[i].x_loc + (sim.new_finches[i].y_loc*30)] =
                Some(sim.new_finches[i].clone());
        };
        sim.new_finches.clear();
    }
    sim.cycle += u128::from(cycles);
}

// TODO Testing
fn match_direct(sim: &mut Sim, i: &usize, direction: &u8) -> bool {
    let (x,y): (usize,usize) = (sim.new_finches[*i].x_loc, sim.new_finches[*i].y_loc);
    // 0 UP,1 DOWN,2 LEFT,3 RIGHT,4 TOP LEFT,5 TOP RIGHT,6 BOT LEFT,7 BOT RIGHT
    match direction {
        0 => if sim.new_finches[*i].y_loc == 0  || sim.space[x + ((y-1)*30)] != None {return false}
        1 => if sim.new_finches[*i].y_loc == 29 || sim.space[x + ((y+1)*30)] != None {return false}
        2 => if sim.new_finches[*i].x_loc == 0  || sim.space[(x-1) + (y*30)] != None {return false}
        3 => if sim.new_finches[*i].x_loc == 29 || sim.space[(x+1) + (y*30)] != None {return false}
        4 => if sim.new_finches[*i].y_loc == 0  || sim.new_finches[*i].x_loc == 0   ||
                sim.space[(x-1) + ((y-1)*30)] != None {return false}
        5 => if sim.new_finches[*i].y_loc == 0  || sim.new_finches[*i].x_loc == 29  ||
                sim.space[(x+1) + ((y-1)*30)] != None {return false}
        6 => if sim.new_finches[*i].y_loc == 29 || sim.new_finches[*i].x_loc == 0  ||
                sim.space[(x-1) + ((y+1)*30)] != None {return false}
        7 => if sim.new_finches[*i].y_loc == 29 || sim.new_finches[*i].x_loc == 29 ||
                sim.space[(x+1) + ((y+1)*30)] != None {return false}
        _ => ()
    } true
}

// TODO Testing
fn change_direct(sim: &mut Sim, i: &usize, direction: &u8) {
    match direction {
        0 => sim.new_finches[*i].y_loc -= 1,
        1 => sim.new_finches[*i].y_loc += 1,
        2 => sim.new_finches[*i].x_loc -= 1,
        3 => sim.new_finches[*i].x_loc += 1,
        4 => {sim.new_finches[*i].x_loc -= 1; sim.new_finches[*i].y_loc -= 1}
        5 => {sim.new_finches[*i].x_loc += 1; sim.new_finches[*i].y_loc -= 1}
        6 => {sim.new_finches[*i].x_loc -= 1; sim.new_finches[*i].y_loc += 1}
        7 => {sim.new_finches[*i].x_loc += 1; sim.new_finches[*i].y_loc += 1}
        _ => ()
    }
}