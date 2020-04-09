pub type PhaseNum = u16;
pub type PhaseLength = PhaseNum;

fn used_recursion(l1:PhaseNum, l2:PhaseNum, combis: &mut Vec<Vec<PhaseNum>>, mut combi: Vec<PhaseNum>) {
    if l1 == 0 {
        combi.reverse(); // must be in increasing order
        combis.push(combi);
        return;
    }
    for p in ((l1-1)..(l2)).rev() {
        let mut combi_part = combi.clone();
        combi_part.push(p);
        used_recursion(l1-1, p, combis, combi_part);
    }
}

fn get_used(l1:PhaseNum, l2:PhaseNum) -> Vec<Vec<PhaseNum>>  {
    assert!(l1 <= l2);
    let mut combis: Vec<Vec<PhaseNum>> = Vec::new();
    let empty_combi: Vec<PhaseNum> = Vec::new();
    used_recursion(l1, l2, &mut combis, empty_combi);
    combis
}

fn get_unused(l2: PhaseNum, used: &Vec<PhaseNum>) -> Vec<PhaseNum> {
    let mut unused: Vec<PhaseNum> = (0..l2).collect();
    for u in used.iter().rev() { // iteration relies on decreasing order 
        unused.remove(*u as usize);
    }
    unused
}


pub struct PhaseCombinations {
    pos: PhaseNum,
    used: Vec<Vec<PhaseNum>>,
    /// size: number of phases in a job with most phases
    size: PhaseNum,
}

impl PhaseCombinations {
    pub fn new(l1: PhaseNum, l2: PhaseNum) -> PhaseCombinations{
        PhaseCombinations {
            pos: 0,
            used: get_used(l1, l2),
            size: l2,
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Combination {
    pub used_idxs: Vec<PhaseNum>,
    pub unused_idxs: Vec<PhaseNum>,
}

impl Iterator for PhaseCombinations {
    type Item = Combination;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < (self.used.len() as PhaseNum) {
            let used = self.used[self.pos as usize].clone();
            let unused = get_unused(self.size, &self.used[self.pos as usize]);
            self.pos += 1;
            let ret = Combination {
                used_idxs: used,
                unused_idxs: unused,
            };
            Some(ret)
        }
        else {
            None
        }
    }

}

