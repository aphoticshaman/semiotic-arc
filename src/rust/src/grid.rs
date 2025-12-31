//! Efficient 64x64 grid representation
//!
//! Design choices:
//! - Fixed 64x64 max size (ARC-AGI-3 constraint)
//! - 4 bits per cell (10 colors + 6 reserved)
//! - Packed storage: 2048 bytes per grid
//! - SIMD-friendly layout for fast symmetry ops

use serde::{Deserialize, Serialize};

/// Maximum grid dimension
pub const MAX_SIZE: usize = 64;

/// A 64x64 grid with 4-bit cells (10 colors)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Grid {
    /// Packed 4-bit cells: 2 cells per byte
    /// Layout: row-major, cells[y * 32 + x/2] contains pixels (x, y) and (x+1, y)
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,

    /// Actual dimensions (may be smaller than 64x64)
    pub width: u8,
    pub height: u8,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            data: vec![0u8; MAX_SIZE * MAX_SIZE / 2],
            width: 0,
            height: 0,
        }
    }
}

impl Grid {
    /// Create empty grid
    pub fn new(width: u8, height: u8) -> Self {
        assert!(width as usize <= MAX_SIZE && height as usize <= MAX_SIZE);
        Self {
            data: vec![0u8; MAX_SIZE * MAX_SIZE / 2],
            width,
            height,
        }
    }

    /// Create from 2D array
    pub fn from_2d(cells: &[&[u8]]) -> Self {
        let height = cells.len().min(MAX_SIZE) as u8;
        let width = cells.get(0).map(|r| r.len()).unwrap_or(0).min(MAX_SIZE) as u8;

        let mut grid = Self::new(width, height);
        for (y, row) in cells.iter().enumerate().take(height as usize) {
            for (x, &cell) in row.iter().enumerate().take(width as usize) {
                grid.set(x as u8, y as u8, cell & 0x0F);
            }
        }
        grid
    }

    /// Get cell value at (x, y)
    #[inline]
    pub fn get(&self, x: u8, y: u8) -> u8 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        let idx = (y as usize) * (MAX_SIZE / 2) + (x as usize) / 2;
        if x % 2 == 0 {
            self.data[idx] & 0x0F
        } else {
            self.data[idx] >> 4
        }
    }

    /// Set cell value at (x, y)
    #[inline]
    pub fn set(&mut self, x: u8, y: u8, value: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = (y as usize) * (MAX_SIZE / 2) + (x as usize) / 2;
        if x % 2 == 0 {
            self.data[idx] = (self.data[idx] & 0xF0) | (value & 0x0F);
        } else {
            self.data[idx] = (self.data[idx] & 0x0F) | ((value & 0x0F) << 4);
        }
    }

    /// Color histogram (count of each color 0-9)
    pub fn color_histogram(&self) -> [u16; 10] {
        let mut hist = [0u16; 10];
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get(x, y) as usize;
                if c < 10 {
                    hist[c] += 1;
                }
            }
        }
        hist
    }

    /// Find connected components (flood fill)
    pub fn connected_components(&self) -> Vec<Component> {
        let mut visited = [[false; MAX_SIZE]; MAX_SIZE];
        let mut components = Vec::new();

        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                if !visited[y][x] && self.get(x as u8, y as u8) != 0 {
                    let color = self.get(x as u8, y as u8);
                    let mut component = Component {
                        color,
                        cells: Vec::new(),
                        bbox: (x as u8, y as u8, x as u8, y as u8),
                    };

                    // Flood fill
                    let mut stack = vec![(x, y)];
                    while let Some((cx, cy)) = stack.pop() {
                        if cx >= self.width as usize || cy >= self.height as usize {
                            continue;
                        }
                        if visited[cy][cx] {
                            continue;
                        }
                        if self.get(cx as u8, cy as u8) != color {
                            continue;
                        }

                        visited[cy][cx] = true;
                        component.cells.push((cx as u8, cy as u8));

                        // Update bounding box
                        component.bbox.0 = component.bbox.0.min(cx as u8);
                        component.bbox.1 = component.bbox.1.min(cy as u8);
                        component.bbox.2 = component.bbox.2.max(cx as u8);
                        component.bbox.3 = component.bbox.3.max(cy as u8);

                        // Add neighbors
                        if cx > 0 { stack.push((cx - 1, cy)); }
                        if cy > 0 { stack.push((cx, cy - 1)); }
                        stack.push((cx + 1, cy));
                        stack.push((cx, cy + 1));
                    }

                    if !component.cells.is_empty() {
                        components.push(component);
                    }
                }
            }
        }

        components
    }

    /// Bounding box of non-zero cells
    pub fn bounding_box(&self) -> (u8, u8, u8, u8) {
        let mut min_x = self.width;
        let mut min_y = self.height;
        let mut max_x = 0u8;
        let mut max_y = 0u8;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) != 0 {
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
            }
        }

        if max_x < min_x {
            (0, 0, 0, 0) // Empty grid
        } else {
            (min_x, min_y, max_x, max_y)
        }
    }

    /// Rotate grid 90° clockwise
    pub fn rotate_90(&self) -> Self {
        let mut rotated = Self::new(self.height, self.width);
        for y in 0..self.height {
            for x in 0..self.width {
                rotated.set(self.height - 1 - y, x, self.get(x, y));
            }
        }
        rotated
    }

    /// Flip grid horizontally
    pub fn flip_horizontal(&self) -> Self {
        let mut flipped = Self::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                flipped.set(self.width - 1 - x, y, self.get(x, y));
            }
        }
        flipped
    }

    /// Flip grid vertically
    pub fn flip_vertical(&self) -> Self {
        let mut flipped = Self::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                flipped.set(x, self.height - 1 - y, self.get(x, y));
            }
        }
        flipped
    }

    /// Hash for comparison
    pub fn hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        self.data.hash(&mut hasher);
        self.width.hash(&mut hasher);
        self.height.hash(&mut hasher);
        hasher.finish()
    }

    /// Difference from another grid (count of changed cells)
    pub fn diff(&self, other: &Grid) -> u32 {
        let mut count = 0;
        let max_w = self.width.max(other.width);
        let max_h = self.height.max(other.height);

        for y in 0..max_h {
            for x in 0..max_w {
                if self.get(x, y) != other.get(x, y) {
                    count += 1;
                }
            }
        }
        count
    }
}

/// A connected component in the grid
#[derive(Clone, Debug)]
pub struct Component {
    pub color: u8,
    pub cells: Vec<(u8, u8)>,
    pub bbox: (u8, u8, u8, u8), // (min_x, min_y, max_x, max_y)
}

impl Component {
    pub fn size(&self) -> usize {
        self.cells.len()
    }

    pub fn width(&self) -> u8 {
        self.bbox.2 - self.bbox.0 + 1
    }

    pub fn height(&self) -> u8 {
        self.bbox.3 - self.bbox.1 + 1
    }

    pub fn center(&self) -> (u8, u8) {
        (
            (self.bbox.0 + self.bbox.2) / 2,
            (self.bbox.1 + self.bbox.3) / 2,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_basics() {
        let mut grid = Grid::new(10, 10);
        grid.set(5, 5, 3);
        assert_eq!(grid.get(5, 5), 3);
        assert_eq!(grid.get(0, 0), 0);
    }

    #[test]
    fn test_color_histogram() {
        let mut grid = Grid::new(4, 4);
        grid.set(0, 0, 1);
        grid.set(1, 0, 1);
        grid.set(2, 0, 2);

        let hist = grid.color_histogram();
        assert_eq!(hist[0], 13); // Background
        assert_eq!(hist[1], 2);
        assert_eq!(hist[2], 1);
    }

    #[test]
    fn test_rotate() {
        let mut grid = Grid::new(2, 3);
        grid.set(0, 0, 1);
        grid.set(1, 2, 2);

        let rotated = grid.rotate_90();
        assert_eq!(rotated.width, 3);
        assert_eq!(rotated.height, 2);
        assert_eq!(rotated.get(2, 0), 1);
        assert_eq!(rotated.get(0, 1), 2);
    }
}
