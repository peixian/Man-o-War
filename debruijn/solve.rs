// Generating de Bruijn sequences
// https://en.wikipedia.org/wiki/De_Bruijn_sequence
// Peixian Wang - July 7, 2016

struct Lyndon {
    alphabet: u32;
    tot_len: u32;
    curr: Vec<uint>;
}

impl Iterator for Lyndon {
    type Item = Vec<uint>;

    fn next(&mut self) -> Vec<uint> {
        self.curr.last_mut() += 1;
        let m = self.curr.len()
        while self.curr.len() < tot_len {
            self.curr.append(curr[0])
        }
        while self.curr.last() == alphabet-1 {
            self.curr.pop()
        }
    }

}
