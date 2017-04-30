pub struct PascalsTriangle{
    n: u32
}

fn fact(n: u32) -> u32{
    match n {
        0 => 1,
        1 => 1,
        x => fact(x-1) * x
    }
}

fn ncr(n: u32, r: u32) -> u32{
    fact(n) / (fact(n-r) * fact(r))
}

fn generate(i : u32) -> Vec<u32> {
    (0..i+1).map(|x| ncr(i, x)).collect()
}


impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            n: row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.n).map(|x| generate(x)).collect()
    }
}
