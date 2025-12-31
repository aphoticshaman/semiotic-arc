//! GF(2) Linear Algebra - The Mathematical Kill Shot
//!
//! Lights Out games are LINEAR over GF(2).
//! This isn't heuristic search. This is EXACT SOLUTION.
//!
//! While others fumble with A* on exponential space,
//! we solve in O(n³) guaranteed.
//!
//! The devious insight: Most "puzzle games" have algebraic structure.
//! Find the structure, collapse the search space.

use std::ops::{BitAnd, BitOr, BitXor, Not};

/// A bit vector for GF(2) computations
/// Supports up to 4096 bits (64×64 grid)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitVec {
    /// 64 words × 64 bits = 4096 bits
    data: [u64; 64],
    len: usize,
}

impl BitVec {
    /// Create a zero vector of given length
    pub fn zeros(len: usize) -> Self {
        assert!(len <= 4096);
        Self { data: [0; 64], len }
    }

    /// Create from a grid state (flatten to bits)
    pub fn from_grid_nonzero(grid: &crate::grid::Grid) -> Self {
        let len = (grid.width as usize) * (grid.height as usize);
        let mut v = Self::zeros(len);

        for y in 0..grid.height {
            for x in 0..grid.width {
                let idx = (y as usize) * (grid.width as usize) + (x as usize);
                if grid.get(x, y) != 0 {
                    v.set(idx, true);
                }
            }
        }
        v
    }

    /// Get bit at index
    #[inline(always)]
    pub fn get(&self, idx: usize) -> bool {
        if idx >= self.len {
            return false;
        }
        let word = idx / 64;
        let bit = idx % 64;
        (self.data[word] >> bit) & 1 == 1
    }

    /// Set bit at index
    #[inline(always)]
    pub fn set(&mut self, idx: usize, val: bool) {
        if idx >= self.len {
            return;
        }
        let word = idx / 64;
        let bit = idx % 64;
        if val {
            self.data[word] |= 1 << bit;
        } else {
            self.data[word] &= !(1 << bit);
        }
    }

    /// Toggle bit at index
    #[inline(always)]
    pub fn toggle(&mut self, idx: usize) {
        if idx >= self.len {
            return;
        }
        let word = idx / 64;
        let bit = idx % 64;
        self.data[word] ^= 1 << bit;
    }

    /// Count set bits
    pub fn popcount(&self) -> usize {
        let words = (self.len + 63) / 64;
        self.data[..words]
            .iter()
            .map(|w| w.count_ones() as usize)
            .sum()
    }

    /// XOR with another vector (GF(2) addition)
    pub fn xor(&self, other: &Self) -> Self {
        let mut result = self.clone();
        let words = (self.len.max(other.len) + 63) / 64;
        for i in 0..words {
            result.data[i] ^= other.data[i];
        }
        result.len = self.len.max(other.len);
        result
    }

    /// XOR-assign
    pub fn xor_assign(&mut self, other: &Self) {
        let words = (self.len.max(other.len) + 63) / 64;
        for i in 0..words {
            self.data[i] ^= other.data[i];
        }
    }

    /// Check if zero
    pub fn is_zero(&self) -> bool {
        let words = (self.len + 63) / 64;
        self.data[..words].iter().all(|&w| w == 0)
    }

    /// Find first set bit
    pub fn first_set(&self) -> Option<usize> {
        let words = (self.len + 63) / 64;
        for i in 0..words {
            if self.data[i] != 0 {
                let bit = self.data[i].trailing_zeros() as usize;
                let idx = i * 64 + bit;
                if idx < self.len {
                    return Some(idx);
                }
            }
        }
        None
    }
}

/// GF(2) Matrix for linear systems
#[derive(Clone, Debug)]
pub struct GF2Matrix {
    /// Row vectors
    rows: Vec<BitVec>,
    /// Number of columns
    cols: usize,
}

impl GF2Matrix {
    /// Create zero matrix
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows: (0..rows).map(|_| BitVec::zeros(cols)).collect(),
            cols,
        }
    }

    /// Create identity matrix
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.rows[i].set(i, true);
        }
        m
    }

    /// Get element
    pub fn get(&self, row: usize, col: usize) -> bool {
        self.rows.get(row).map(|r| r.get(col)).unwrap_or(false)
    }

    /// Set element
    pub fn set(&mut self, row: usize, col: usize, val: bool) {
        if let Some(r) = self.rows.get_mut(row) {
            r.set(col, val);
        }
    }

    /// Number of rows
    pub fn nrows(&self) -> usize {
        self.rows.len()
    }

    /// Number of columns
    pub fn ncols(&self) -> usize {
        self.cols
    }

    /// Row echelon form using Gaussian elimination
    /// Returns (row echelon form, transformation matrix)
    pub fn row_echelon(&self) -> (Self, Self) {
        let mut m = self.clone();
        let mut t = Self::identity(self.nrows());

        let mut pivot_row = 0;

        for col in 0..self.cols {
            // Find pivot
            let mut found = None;
            for row in pivot_row..m.nrows() {
                if m.get(row, col) {
                    found = Some(row);
                    break;
                }
            }

            let Some(pivot_idx) = found else {
                continue;
            };

            // Swap rows
            if pivot_idx != pivot_row {
                m.rows.swap(pivot_row, pivot_idx);
                t.rows.swap(pivot_row, pivot_idx);
            }

            // Eliminate below
            for row in (pivot_row + 1)..m.nrows() {
                if m.get(row, col) {
                    let pivot_vec = m.rows[pivot_row].clone();
                    m.rows[row].xor_assign(&pivot_vec);

                    let t_pivot = t.rows[pivot_row].clone();
                    t.rows[row].xor_assign(&t_pivot);
                }
            }

            pivot_row += 1;
        }

        (m, t)
    }

    /// Reduced row echelon form (Gauss-Jordan)
    pub fn rref(&self) -> (Self, Self) {
        let (mut m, mut t) = self.row_echelon();

        // Back-substitute
        for col in (0..self.cols).rev() {
            // Find pivot row for this column
            let mut pivot_row = None;
            for row in 0..m.nrows() {
                if m.get(row, col) {
                    // Check this is the leading 1
                    let mut is_leading = true;
                    for c in 0..col {
                        if m.get(row, c) {
                            is_leading = false;
                            break;
                        }
                    }
                    if is_leading {
                        pivot_row = Some(row);
                        break;
                    }
                }
            }

            let Some(pr) = pivot_row else {
                continue;
            };

            // Eliminate above
            for row in 0..pr {
                if m.get(row, col) {
                    let pivot_vec = m.rows[pr].clone();
                    m.rows[row].xor_assign(&pivot_vec);

                    let t_pivot = t.rows[pr].clone();
                    t.rows[row].xor_assign(&t_pivot);
                }
            }
        }

        (m, t)
    }

    /// Compute rank
    pub fn rank(&self) -> usize {
        let (ref_, _) = self.row_echelon();
        ref_.rows.iter().filter(|r| !r.is_zero()).count()
    }

    /// Solve Ax = b over GF(2)
    /// Returns None if no solution, Some(solution) if solvable
    pub fn solve(&self, b: &BitVec) -> Option<BitVec> {
        // Augmented matrix [A|b]
        let mut aug = Self::zeros(self.nrows(), self.cols + 1);
        for row in 0..self.nrows() {
            aug.rows[row] = self.rows[row].clone();
            aug.rows[row].len = self.cols + 1;
            aug.set(row, self.cols, b.get(row));
        }

        // RREF
        let (ref_, _) = aug.rref();

        // Check consistency
        for row in 0..ref_.nrows() {
            // If row is [0 0 ... 0 | 1], no solution
            let mut all_zero = true;
            for col in 0..self.cols {
                if ref_.get(row, col) {
                    all_zero = false;
                    break;
                }
            }
            if all_zero && ref_.get(row, self.cols) {
                return None;
            }
        }

        // Extract solution (basic solution, free variables = 0)
        let mut x = BitVec::zeros(self.cols);

        // Find pivot columns
        let mut pivot_cols = vec![None; ref_.nrows()];
        for row in 0..ref_.nrows() {
            for col in 0..self.cols {
                if ref_.get(row, col) {
                    pivot_cols[row] = Some(col);
                    break;
                }
            }
        }

        // Back-substitute from RREF
        for row in (0..ref_.nrows()).rev() {
            if let Some(col) = pivot_cols[row] {
                let mut val = ref_.get(row, self.cols);
                for c in (col + 1)..self.cols {
                    if ref_.get(row, c) && x.get(c) {
                        val = !val;
                    }
                }
                x.set(col, val);
            }
        }

        Some(x)
    }
}

/// Build the toggle matrix for a Lights Out game
pub fn build_lights_out_matrix(
    width: usize,
    height: usize,
    pattern: TogglePattern,
) -> GF2Matrix {
    let n = width * height;
    let mut m = GF2Matrix::zeros(n, n);

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;

            // What cells does clicking (x, y) toggle?
            match pattern {
                TogglePattern::Single => {
                    m.set(idx, idx, true);
                }
                TogglePattern::Cross => {
                    // Center
                    m.set(idx, idx, true);
                    // Up
                    if y > 0 {
                        m.set(idx, (y - 1) * width + x, true);
                    }
                    // Down
                    if y + 1 < height {
                        m.set(idx, (y + 1) * width + x, true);
                    }
                    // Left
                    if x > 0 {
                        m.set(idx, y * width + (x - 1), true);
                    }
                    // Right
                    if x + 1 < width {
                        m.set(idx, y * width + (x + 1), true);
                    }
                }
                TogglePattern::Neighborhood3x3 => {
                    for dy in -1i32..=1 {
                        for dx in -1i32..=1 {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if nx >= 0 && (nx as usize) < width && ny >= 0 && (ny as usize) < height
                            {
                                let nidx = (ny as usize) * width + (nx as usize);
                                m.set(idx, nidx, true);
                            }
                        }
                    }
                }
                TogglePattern::Row => {
                    for tx in 0..width {
                        m.set(idx, y * width + tx, true);
                    }
                }
                TogglePattern::Column => {
                    for ty in 0..height {
                        m.set(idx, ty * width + x, true);
                    }
                }
            }
        }
    }

    m
}

/// Toggle patterns for Lights Out variants
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TogglePattern {
    Single,
    Cross,
    Neighborhood3x3,
    Row,
    Column,
}

/// Solve a Lights Out puzzle
/// Returns the click positions needed to clear all lights
pub fn solve_lights_out(
    grid: &crate::grid::Grid,
    pattern: TogglePattern,
) -> Option<Vec<(u8, u8)>> {
    let width = grid.width as usize;
    let height = grid.height as usize;

    // Build the toggle matrix
    let matrix = build_lights_out_matrix(width, height, pattern);

    // Target: current state (we want to toggle it to all zeros)
    let target = BitVec::from_grid_nonzero(grid);

    // Solve
    let solution = matrix.solve(&target)?;

    // Extract click positions
    let mut clicks = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if solution.get(idx) {
                clicks.push((x as u8, y as u8));
            }
        }
    }

    Some(clicks)
}

/// Check if a Lights Out puzzle is solvable
pub fn is_lights_out_solvable(
    grid: &crate::grid::Grid,
    pattern: TogglePattern,
) -> bool {
    solve_lights_out(grid, pattern).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn test_bitvec_operations() {
        let mut v = BitVec::zeros(100);
        v.set(0, true);
        v.set(63, true);
        v.set(64, true);
        v.set(99, true);

        assert!(v.get(0));
        assert!(v.get(63));
        assert!(v.get(64));
        assert!(v.get(99));
        assert!(!v.get(50));

        assert_eq!(v.popcount(), 4);
    }

    #[test]
    fn test_gf2_solve() {
        // Simple 2x2 system: x1 + x2 = 1, x1 = 0
        let mut m = GF2Matrix::zeros(2, 2);
        m.set(0, 0, true);
        m.set(0, 1, true);
        m.set(1, 0, true);

        let mut b = BitVec::zeros(2);
        b.set(0, true);
        b.set(1, false);

        let sol = m.solve(&b).unwrap();
        // x1 = 0, x2 = 1
        assert!(!sol.get(0));
        assert!(sol.get(1));
    }

    #[test]
    fn test_lights_out_3x3() {
        // Create a 3x3 grid with center lit
        let mut grid = Grid::new(3, 3);
        grid.set(1, 1, 1);

        let clicks = solve_lights_out(&grid, TogglePattern::Cross);
        assert!(clicks.is_some());

        // Clicking center should turn it off
        let clicks = clicks.unwrap();
        assert!(clicks.contains(&(1, 1)));
    }

    #[test]
    fn test_matrix_rank() {
        let mut m = GF2Matrix::zeros(3, 3);
        m.set(0, 0, true);
        m.set(1, 1, true);
        m.set(2, 2, true);
        assert_eq!(m.rank(), 3);

        // Dependent row
        m.rows[2] = m.rows[0].xor(&m.rows[1]);
        assert_eq!(m.rank(), 2);
    }
}
