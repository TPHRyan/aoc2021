pub enum BingoTrigger {
    COLUMN(usize),
    ROW(usize),
}

pub struct Bingo {
    pub final_number: u32,
    pub triggered_by: BingoTrigger,
}
