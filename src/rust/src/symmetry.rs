//! Symmetry detection and canonicalization
//!
//! Layer 1 of the architecture: reduce hypothesis space by exploiting symmetries.
//!
//! The Dihedral group D4 acts on grids:
//! - Identity (e)
//! - Rotate 90° (r)
//! - Rotate 180° (r²)
//! - Rotate 270° (r³)
//! - Flip horizontal (f)
//! - Flip vertical (fr²)
//! - Flip diagonal (fr)
//! - Flip anti-diagonal (fr³)
//!
//! Canonicalization picks the lexicographically smallest representative.
//! This reduces hypothesis space by up to 8x.

use crate::grid::Grid;
use bitflags::bitflags;

bitflags! {
    /// Detected symmetries of a grid
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct SymmetryFlags: u8 {
        const NONE = 0;
        const ROTATE_180 = 1 << 0;
        const ROTATE_90 = 1 << 1;  // Implies ROTATE_180
        const FLIP_H = 1 << 2;
        const FLIP_V = 1 << 3;
        const FLIP_DIAG = 1 << 4;
        const FLIP_ANTI = 1 << 5;
    }
}

/// Canonicalize grid by picking lexicographically smallest D4 representative
///
/// Returns (canonical_grid, transform_applied)
pub fn canonicalize(grid: &Grid) -> Grid {
    let transforms = generate_d4_transforms(grid);

    // Find lexicographically smallest
    transforms
        .into_iter()
        .min_by(|a, b| compare_grids(a, b))
        .unwrap_or_else(|| grid.clone())
}

/// Generate all 8 D4 transforms of a grid
fn generate_d4_transforms(grid: &Grid) -> Vec<Grid> {
    let mut transforms = Vec::with_capacity(8);

    // Identity
    transforms.push(grid.clone());

    // Rotations
    let r90 = grid.rotate_90();
    let r180 = r90.rotate_90();
    let r270 = r180.rotate_90();

    transforms.push(r90);
    transforms.push(r180);
    transforms.push(r270);

    // Flips
    transforms.push(grid.flip_horizontal());
    transforms.push(grid.flip_vertical());

    // Diagonal flips (transpose and anti-transpose)
    transforms.push(transpose(grid));
    transforms.push(anti_transpose(grid));

    transforms
}

/// Transpose grid (flip along main diagonal)
fn transpose(grid: &Grid) -> Grid {
    let mut t = Grid::new(grid.height, grid.width);
    for y in 0..grid.height {
        for x in 0..grid.width {
            t.set(y, x, grid.get(x, y));
        }
    }
    t
}

/// Anti-transpose grid (flip along anti-diagonal)
fn anti_transpose(grid: &Grid) -> Grid {
    let mut t = Grid::new(grid.height, grid.width);
    for y in 0..grid.height {
        for x in 0..grid.width {
            t.set(
                grid.height - 1 - y,
                grid.width - 1 - x,
                grid.get(x, y),
            );
        }
    }
    t
}

/// Compare two grids lexicographically
fn compare_grids(a: &Grid, b: &Grid) -> std::cmp::Ordering {
    // First compare dimensions
    match (a.width, a.height).cmp(&(b.width, b.height)) {
        std::cmp::Ordering::Equal => {}
        other => return other,
    }

    // Then compare cell by cell
    for y in 0..a.height {
        for x in 0..a.width {
            match a.get(x, y).cmp(&b.get(x, y)) {
                std::cmp::Ordering::Equal => {}
                other => return other,
            }
        }
    }

    std::cmp::Ordering::Equal
}

/// Detect which symmetries a grid has
pub fn detect_symmetries(grid: &Grid) -> SymmetryFlags {
    let mut flags = SymmetryFlags::NONE;

    // Check 180° rotation symmetry
    let r180 = grid.rotate_90().rotate_90();
    if grids_equal(grid, &r180) {
        flags |= SymmetryFlags::ROTATE_180;
    }

    // Check 90° rotation symmetry (only if square)
    if grid.width == grid.height {
        let r90 = grid.rotate_90();
        if grids_equal(grid, &r90) {
            flags |= SymmetryFlags::ROTATE_90;
        }
    }

    // Check horizontal flip symmetry
    let flip_h = grid.flip_horizontal();
    if grids_equal(grid, &flip_h) {
        flags |= SymmetryFlags::FLIP_H;
    }

    // Check vertical flip symmetry
    let flip_v = grid.flip_vertical();
    if grids_equal(grid, &flip_v) {
        flags |= SymmetryFlags::FLIP_V;
    }

    // Check diagonal symmetry (only if square)
    if grid.width == grid.height {
        let t = transpose(grid);
        if grids_equal(grid, &t) {
            flags |= SymmetryFlags::FLIP_DIAG;
        }

        let at = anti_transpose(grid);
        if grids_equal(grid, &at) {
            flags |= SymmetryFlags::FLIP_ANTI;
        }
    }

    flags
}

/// Check if two grids are equal
fn grids_equal(a: &Grid, b: &Grid) -> bool {
    if a.width != b.width || a.height != b.height {
        return false;
    }

    for y in 0..a.height {
        for x in 0..a.width {
            if a.get(x, y) != b.get(x, y) {
                return false;
            }
        }
    }

    true
}

/// Color permutation canonicalization
///
/// Maps colors to canonical order (most frequent = 1, second = 2, etc.)
/// Background (0) stays 0.
pub fn canonicalize_colors(grid: &Grid) -> Grid {
    let hist = grid.color_histogram();

    // Sort non-background colors by frequency (descending)
    let mut color_order: Vec<(u8, u16)> = (1..10)
        .filter(|&c| hist[c as usize] > 0)
        .map(|c| (c, hist[c as usize]))
        .collect();

    color_order.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    // Build mapping: old_color -> new_color
    let mut mapping = [0u8; 10];
    for (new_color, (old_color, _)) in color_order.iter().enumerate() {
        mapping[*old_color as usize] = (new_color + 1) as u8;
    }

    // Apply mapping
    let mut canonical = Grid::new(grid.width, grid.height);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let old = grid.get(x, y);
            canonical.set(x, y, mapping[old as usize]);
        }
    }

    canonical
}

/// Full canonicalization: symmetry + color permutation
pub fn full_canonicalize(grid: &Grid) -> Grid {
    let spatial = canonicalize(grid);
    canonicalize_colors(&spatial)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetry_detection() {
        // Symmetric 3x3 grid
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, 1); grid.set(1, 0, 2); grid.set(2, 0, 1);
        grid.set(0, 1, 2); grid.set(1, 1, 3); grid.set(2, 1, 2);
        grid.set(0, 2, 1); grid.set(1, 2, 2); grid.set(2, 2, 1);

        let flags = detect_symmetries(&grid);
        assert!(flags.contains(SymmetryFlags::FLIP_H));
        assert!(flags.contains(SymmetryFlags::FLIP_V));
    }

    #[test]
    fn test_canonicalization_consistent() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, 1);
        grid.set(2, 2, 2);

        // All transforms should canonicalize to same result
        let c1 = canonicalize(&grid);
        let c2 = canonicalize(&grid.rotate_90());
        let c3 = canonicalize(&grid.flip_horizontal());

        assert_eq!(c1.hash(), c2.hash());
        assert_eq!(c1.hash(), c3.hash());
    }
}
