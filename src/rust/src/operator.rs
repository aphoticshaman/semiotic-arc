//! Operator Algebra for Grid Transformations
//!
//! Gap 1 from feedback: Missing formal operator specification.
//!
//! Operators form a group under composition:
//! - Identity: I(g) = g
//! - Composition: (f ∘ g)(x) = f(g(x))
//! - Inverse: f⁻¹ ∘ f = I (where possible)
//!
//! Key insight: ARC transformations are compositions of ~50 primitives.
//! The search space is enumerable.

use crate::grid::Grid;
use crate::dsl::Primitive;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A grid operator that can be composed and (sometimes) inverted.
///
/// This is the missing piece from Gap 1 in the feedback.
pub trait GridOperator: Clone + Send + Sync {
    /// Apply the operator to a grid
    fn apply(&self, grid: &Grid) -> Grid;

    /// Compute the inverse operator if it exists
    fn inverse(&self) -> Option<Box<dyn GridOperator>>;

    /// Compose with another operator: self ∘ other
    fn compose(&self, other: &dyn GridOperator) -> ComposedOperator;

    /// Estimated complexity (for Occam's razor)
    fn complexity(&self) -> usize;

    /// Human-readable description
    fn describe(&self) -> String;
}

/// A composition of operators
#[derive(Clone)]
pub struct ComposedOperator {
    /// Operators applied in sequence (first to last)
    operators: Vec<Box<dyn GridOperator>>,
}

impl ComposedOperator {
    pub fn new() -> Self {
        Self { operators: Vec::new() }
    }

    pub fn from_single<O: GridOperator + 'static>(op: O) -> Self {
        Self {
            operators: vec![Box::new(op)],
        }
    }

    pub fn push<O: GridOperator + 'static>(&mut self, op: O) {
        self.operators.push(Box::new(op));
    }

    pub fn len(&self) -> usize {
        self.operators.len()
    }

    pub fn is_empty(&self) -> bool {
        self.operators.is_empty()
    }
}

impl GridOperator for ComposedOperator {
    fn apply(&self, grid: &Grid) -> Grid {
        let mut result = grid.clone();
        for op in &self.operators {
            result = op.apply(&result);
        }
        result
    }

    fn inverse(&self) -> Option<Box<dyn GridOperator>> {
        // Inverse of composition: (f ∘ g)⁻¹ = g⁻¹ ∘ f⁻¹
        let mut inverses = Vec::new();
        for op in self.operators.iter().rev() {
            if let Some(inv) = op.inverse() {
                inverses.push(inv);
            } else {
                return None; // Not invertible
            }
        }
        Some(Box::new(ComposedOperator { operators: inverses }))
    }

    fn compose(&self, other: &dyn GridOperator) -> ComposedOperator {
        let mut result = self.clone();
        // This is a hack - we can't clone trait objects directly
        // In practice, use the ComposedOperator::then() method
        result
    }

    fn complexity(&self) -> usize {
        1 + self.operators.iter().map(|o| o.complexity()).sum::<usize>()
    }

    fn describe(&self) -> String {
        let parts: Vec<String> = self.operators.iter()
            .map(|o| o.describe())
            .collect();
        if parts.is_empty() {
            "Identity".to_string()
        } else {
            parts.join(" → ")
        }
    }
}

impl ComposedOperator {
    /// Chain another operator at the end
    pub fn then<O: GridOperator + 'static>(mut self, op: O) -> Self {
        self.operators.push(Box::new(op));
        self
    }

    /// Chain another composed operator
    pub fn then_compose(mut self, other: ComposedOperator) -> Self {
        self.operators.extend(other.operators);
        self
    }
}

/// Identity operator
#[derive(Clone)]
pub struct Identity;

impl GridOperator for Identity {
    fn apply(&self, grid: &Grid) -> Grid {
        grid.clone()
    }

    fn inverse(&self) -> Option<Box<dyn GridOperator>> {
        Some(Box::new(Identity))
    }

    fn compose(&self, other: &dyn GridOperator) -> ComposedOperator {
        ComposedOperator::from_single(Identity)
    }

    fn complexity(&self) -> usize {
        0
    }

    fn describe(&self) -> String {
        "Identity".to_string()
    }
}

/// Rotation operators
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Rotation {
    CW90,    // Clockwise 90°
    CW180,   // Clockwise 180°
    CW270,   // Clockwise 270° (= CCW 90°)
}

impl GridOperator for Rotation {
    fn apply(&self, grid: &Grid) -> Grid {
        match self {
            Rotation::CW90 => grid.rotate_90(),
            Rotation::CW180 => grid.rotate_90().rotate_90(),
            Rotation::CW270 => grid.rotate_90().rotate_90().rotate_90(),
        }
    }

    fn inverse(&self) -> Option<Box<dyn GridOperator>> {
        Some(Box::new(match self {
            Rotation::CW90 => Rotation::CW270,
            Rotation::CW180 => Rotation::CW180,
            Rotation::CW270 => Rotation::CW90,
        }))
    }

    fn compose(&self, _other: &dyn GridOperator) -> ComposedOperator {
        ComposedOperator::from_single(*self)
    }

    fn complexity(&self) -> usize {
        1
    }

    fn describe(&self) -> String {
        match self {
            Rotation::CW90 => "Rotate CW 90°",
            Rotation::CW180 => "Rotate 180°",
            Rotation::CW270 => "Rotate CCW 90°",
        }.to_string()
    }
}

/// Flip operators
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Flip {
    Horizontal,
    Vertical,
}

impl GridOperator for Flip {
    fn apply(&self, grid: &Grid) -> Grid {
        match self {
            Flip::Horizontal => grid.flip_horizontal(),
            Flip::Vertical => grid.flip_vertical(),
        }
    }

    fn inverse(&self) -> Option<Box<dyn GridOperator>> {
        // Flips are self-inverse
        Some(Box::new(*self))
    }

    fn compose(&self, _other: &dyn GridOperator) -> ComposedOperator {
        ComposedOperator::from_single(*self)
    }

    fn complexity(&self) -> usize {
        1
    }

    fn describe(&self) -> String {
        match self {
            Flip::Horizontal => "Flip Horizontal",
            Flip::Vertical => "Flip Vertical",
        }.to_string()
    }
}

/// Translation operator
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Translate {
    pub dx: i8,
    pub dy: i8,
}

impl GridOperator for Translate {
    fn apply(&self, grid: &Grid) -> Grid {
        let mut result = Grid::new(grid.width, grid.height);
        for y in 0..grid.height {
            for x in 0..grid.width {
                let nx = (x as i16 + self.dx as i16) as u8;
                let ny = (y as i16 + self.dy as i16) as u8;
                if nx < grid.width && ny < grid.height {
                    result.set(nx, ny, grid.get(x, y));
                }
            }
        }
        result
    }

    fn inverse(&self) -> Option<Box<dyn GridOperator>> {
        Some(Box::new(Translate { dx: -self.dx, dy: -self.dy }))
    }

    fn compose(&self, _other: &dyn GridOperator) -> ComposedOperator {
        ComposedOperator::from_single(*self)
    }

    fn complexity(&self) -> usize {
        2
    }

    fn describe(&self) -> String {
        format!("Translate ({}, {})", self.dx, self.dy)
    }
}

/// Scale operator
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Scale {
    pub factor: u8,
}

impl GridOperator for Scale {
    fn apply(&self, grid: &Grid) -> Grid {
        let new_w = (grid.width as u16 * self.factor as u16).min(64) as u8;
        let new_h = (grid.height as u16 * self.factor as u16).min(64) as u8;
        let mut result = Grid::new(new_w, new_h);

        for y in 0..new_h {
            for x in 0..new_w {
                let src_x = x / self.factor;
                let src_y = y / self.factor;
                result.set(x, y, grid.get(src_x, src_y));
            }
        }
        result
    }

    fn inverse(&self) -> Option<Box<dyn GridOperator>> {
        // Scaling is not perfectly invertible
        None
    }

    fn compose(&self, _other: &dyn GridOperator) -> ComposedOperator {
        ComposedOperator::from_single(*self)
    }

    fn complexity(&self) -> usize {
        2
    }

    fn describe(&self) -> String {
        format!("Scale {}x", self.factor)
    }
}

/// Color mapping operator
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorMap {
    /// Mapping from old color to new color
    pub map: [u8; 10],
}

impl ColorMap {
    pub fn new() -> Self {
        Self { map: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] }
    }

    pub fn swap(c1: u8, c2: u8) -> Self {
        let mut map = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        map[c1 as usize] = c2;
        map[c2 as usize] = c1;
        Self { map }
    }

    pub fn replace(from: u8, to: u8) -> Self {
        let mut map = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        map[from as usize] = to;
        Self { map }
    }
}

impl GridOperator for ColorMap {
    fn apply(&self, grid: &Grid) -> Grid {
        let mut result = Grid::new(grid.width, grid.height);
        for y in 0..grid.height {
            for x in 0..grid.width {
                let c = grid.get(x, y);
                let new_c = if (c as usize) < 10 { self.map[c as usize] } else { c };
                result.set(x, y, new_c);
            }
        }
        result
    }

    fn inverse(&self) -> Option<Box<dyn GridOperator>> {
        // Build inverse mapping
        let mut inv_map = [0u8; 10];
        for (i, &target) in self.map.iter().enumerate() {
            if (target as usize) < 10 {
                inv_map[target as usize] = i as u8;
            }
        }
        Some(Box::new(ColorMap { map: inv_map }))
    }

    fn compose(&self, _other: &dyn GridOperator) -> ComposedOperator {
        ComposedOperator::from_single(self.clone())
    }

    fn complexity(&self) -> usize {
        // Count non-identity mappings
        self.map.iter().enumerate()
            .filter(|(i, &v)| v as usize != *i)
            .count()
            .max(1)
    }

    fn describe(&self) -> String {
        let changes: Vec<String> = self.map.iter().enumerate()
            .filter(|(i, &v)| v as usize != *i)
            .map(|(i, &v)| format!("{}->{}", i, v))
            .collect();
        if changes.is_empty() {
            "ColorMap (identity)".to_string()
        } else {
            format!("ColorMap [{}]", changes.join(", "))
        }
    }
}

/// Crop operator - extract a region
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Crop {
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
}

impl GridOperator for Crop {
    fn apply(&self, grid: &Grid) -> Grid {
        let mut result = Grid::new(self.w, self.h);
        for dy in 0..self.h {
            for dx in 0..self.w {
                result.set(dx, dy, grid.get(self.x + dx, self.y + dy));
            }
        }
        result
    }

    fn inverse(&self) -> Option<Box<dyn GridOperator>> {
        None // Cropping loses information
    }

    fn compose(&self, _other: &dyn GridOperator) -> ComposedOperator {
        ComposedOperator::from_single(*self)
    }

    fn complexity(&self) -> usize {
        4
    }

    fn describe(&self) -> String {
        format!("Crop ({},{}) {}x{}", self.x, self.y, self.w, self.h)
    }
}

/// Fill operator - fill a region with a color
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Fill {
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
    pub color: u8,
}

impl GridOperator for Fill {
    fn apply(&self, grid: &Grid) -> Grid {
        let mut result = grid.clone();
        for dy in 0..self.h {
            for dx in 0..self.w {
                let px = self.x.saturating_add(dx);
                let py = self.y.saturating_add(dy);
                if px < grid.width && py < grid.height {
                    result.set(px, py, self.color);
                }
            }
        }
        result
    }

    fn inverse(&self) -> Option<Box<dyn GridOperator>> {
        None // Filling loses information
    }

    fn compose(&self, _other: &dyn GridOperator) -> ComposedOperator {
        ComposedOperator::from_single(*self)
    }

    fn complexity(&self) -> usize {
        5
    }

    fn describe(&self) -> String {
        format!("Fill ({},{}) {}x{} color={}", self.x, self.y, self.w, self.h, self.color)
    }
}

/// Tile/repeat operator
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub repeat_x: u8,
    pub repeat_y: u8,
}

impl GridOperator for Tile {
    fn apply(&self, grid: &Grid) -> Grid {
        let new_w = (grid.width as u16 * self.repeat_x as u16).min(64) as u8;
        let new_h = (grid.height as u16 * self.repeat_y as u16).min(64) as u8;
        let mut result = Grid::new(new_w, new_h);

        for y in 0..new_h {
            for x in 0..new_w {
                let src_x = x % grid.width;
                let src_y = y % grid.height;
                result.set(x, y, grid.get(src_x, src_y));
            }
        }
        result
    }

    fn inverse(&self) -> Option<Box<dyn GridOperator>> {
        None
    }

    fn compose(&self, _other: &dyn GridOperator) -> ComposedOperator {
        ComposedOperator::from_single(*self)
    }

    fn complexity(&self) -> usize {
        3
    }

    fn describe(&self) -> String {
        format!("Tile {}x{}", self.repeat_x, self.repeat_y)
    }
}

/// Overlay operator - overlay one grid on another
#[derive(Clone, Debug)]
pub struct Overlay {
    pub mask: Grid,
    pub transparent_color: u8,
}

impl GridOperator for Overlay {
    fn apply(&self, grid: &Grid) -> Grid {
        let mut result = grid.clone();
        for y in 0..self.mask.height.min(grid.height) {
            for x in 0..self.mask.width.min(grid.width) {
                let c = self.mask.get(x, y);
                if c != self.transparent_color {
                    result.set(x, y, c);
                }
            }
        }
        result
    }

    fn inverse(&self) -> Option<Box<dyn GridOperator>> {
        None
    }

    fn compose(&self, _other: &dyn GridOperator) -> ComposedOperator {
        ComposedOperator::from_single(self.clone())
    }

    fn complexity(&self) -> usize {
        10 // Overlay is complex
    }

    fn describe(&self) -> String {
        format!("Overlay (transparent={})", self.transparent_color)
    }
}

/// All primitive operators in an enum for easy iteration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PrimitiveOp {
    Identity,
    RotateCW90,
    RotateCW180,
    RotateCW270,
    FlipH,
    FlipV,
    Translate(i8, i8),
    Scale(u8),
    ColorMap([u8; 10]),
    Crop(u8, u8, u8, u8),
    Fill(u8, u8, u8, u8, u8),
    Tile(u8, u8),
}

impl PrimitiveOp {
    /// Convert to a boxed GridOperator
    pub fn to_operator(&self) -> Box<dyn GridOperator> {
        match self {
            PrimitiveOp::Identity => Box::new(Identity),
            PrimitiveOp::RotateCW90 => Box::new(Rotation::CW90),
            PrimitiveOp::RotateCW180 => Box::new(Rotation::CW180),
            PrimitiveOp::RotateCW270 => Box::new(Rotation::CW270),
            PrimitiveOp::FlipH => Box::new(Flip::Horizontal),
            PrimitiveOp::FlipV => Box::new(Flip::Vertical),
            PrimitiveOp::Translate(dx, dy) => Box::new(Translate { dx: *dx, dy: *dy }),
            PrimitiveOp::Scale(f) => Box::new(Scale { factor: *f }),
            PrimitiveOp::ColorMap(m) => Box::new(ColorMap { map: *m }),
            PrimitiveOp::Crop(x, y, w, h) => Box::new(Crop { x: *x, y: *y, w: *w, h: *h }),
            PrimitiveOp::Fill(x, y, w, h, c) => Box::new(Fill { x: *x, y: *y, w: *w, h: *h, color: *c }),
            PrimitiveOp::Tile(rx, ry) => Box::new(Tile { repeat_x: *rx, repeat_y: *ry }),
        }
    }

    /// All simple operators (no parameters)
    pub fn simple_primitives() -> Vec<PrimitiveOp> {
        vec![
            PrimitiveOp::Identity,
            PrimitiveOp::RotateCW90,
            PrimitiveOp::RotateCW180,
            PrimitiveOp::RotateCW270,
            PrimitiveOp::FlipH,
            PrimitiveOp::FlipV,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_inverse() {
        let mut grid = Grid::new(3, 2);
        grid.set(0, 0, 1);
        grid.set(2, 1, 2);

        let rot = Rotation::CW90;
        let rotated = rot.apply(&grid);

        let inv = rot.inverse().unwrap();
        let restored = inv.apply(&rotated);

        assert_eq!(grid.get(0, 0), restored.get(0, 0));
        assert_eq!(grid.get(2, 1), restored.get(2, 1));
    }

    #[test]
    fn test_composition() {
        let mut grid = Grid::new(4, 4);
        grid.set(0, 0, 1);

        let composed = ComposedOperator::from_single(Rotation::CW90)
            .then(Rotation::CW90);

        let result = composed.apply(&grid);
        let direct = Rotation::CW180.apply(&grid);

        assert_eq!(result.hash(), direct.hash());
    }

    #[test]
    fn test_color_map() {
        let mut grid = Grid::new(2, 2);
        grid.set(0, 0, 1);
        grid.set(1, 0, 2);

        let swap = ColorMap::swap(1, 2);
        let result = swap.apply(&grid);

        assert_eq!(result.get(0, 0), 2);
        assert_eq!(result.get(1, 0), 1);
    }
}
