pub struct Cell {
    column: i32,
    row: i32,
    block: i32,
    candidate_bits: i32,
}

impl Eq for Cell { }

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.candidate_bits != other.candidate_bits {
            return self.candidate_bits.cmp(&other.candidate_bits).reverse();
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.column == other.column && self.row == other.row && self.block == other.block && self.candidate_bits == other.candidate_bits
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}