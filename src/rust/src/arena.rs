//! Arena Allocator for Zero-Copy State Management
//!
//! In A* search, we create thousands of Grid clones.
//! Arena allocation lets us recycle memory without syscalls.
//!
//! Key insight: Most states are ephemeral.
//! Only the final path matters.

use std::cell::UnsafeCell;
use std::ptr::NonNull;

/// Size of a single grid allocation (2048 bytes + metadata)
const GRID_ALLOC_SIZE: usize = 2048 + 8;

/// Number of grids per arena chunk
const GRIDS_PER_CHUNK: usize = 1024;

/// Chunk size in bytes
const CHUNK_SIZE: usize = GRID_ALLOC_SIZE * GRIDS_PER_CHUNK;

/// Arena allocator for grid states
pub struct GridArena {
    /// Current chunk being allocated from
    current: UnsafeCell<ArenaChunk>,

    /// Full chunks (kept alive for deallocation)
    full_chunks: UnsafeCell<Vec<ArenaChunk>>,

    /// Statistics
    allocations: UnsafeCell<u64>,
    bytes_used: UnsafeCell<usize>,
}

/// A single arena chunk
struct ArenaChunk {
    /// Raw memory (heap-allocated via Vec to avoid stack overflow)
    data: Vec<u8>,

    /// Next free position
    pos: usize,
}

impl ArenaChunk {
    fn new() -> Self {
        // Allocate on heap via Vec to avoid stack overflow
        let mut data = Vec::with_capacity(CHUNK_SIZE);
        data.resize(CHUNK_SIZE, 0);
        Self { data, pos: 0 }
    }

    fn alloc(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
        // Align position
        let aligned_pos = (self.pos + align - 1) & !(align - 1);

        if aligned_pos + size > CHUNK_SIZE {
            return None;
        }

        let ptr = unsafe { self.data.as_mut_ptr().add(aligned_pos) };
        self.pos = aligned_pos + size;

        NonNull::new(ptr)
    }

    fn reset(&mut self) {
        self.pos = 0;
    }

    fn remaining(&self) -> usize {
        CHUNK_SIZE - self.pos
    }
}

impl GridArena {
    /// Create a new arena
    pub fn new() -> Self {
        Self {
            current: UnsafeCell::new(ArenaChunk::new()),
            full_chunks: UnsafeCell::new(Vec::new()),
            allocations: UnsafeCell::new(0),
            bytes_used: UnsafeCell::new(0),
        }
    }

    /// Allocate memory for a grid
    ///
    /// # Safety
    /// The returned pointer is valid until the arena is reset or dropped.
    pub unsafe fn alloc_grid(&self) -> NonNull<u8> {
        let current = &mut *self.current.get();

        if let Some(ptr) = current.alloc(GRID_ALLOC_SIZE, 8) {
            *self.allocations.get() += 1;
            *self.bytes_used.get() += GRID_ALLOC_SIZE;
            return ptr;
        }

        // Current chunk full - get a new one
        let mut new_chunk = ArenaChunk::new();
        let ptr = new_chunk
            .alloc(GRID_ALLOC_SIZE, 8)
            .expect("fresh chunk should have space");

        // Move current to full list
        let old_chunk = std::mem::replace(current, new_chunk);
        (*self.full_chunks.get()).push(old_chunk);

        *self.allocations.get() += 1;
        *self.bytes_used.get() += GRID_ALLOC_SIZE;

        ptr
    }

    /// Reset the arena (invalidates all allocations)
    ///
    /// # Safety
    /// All pointers from this arena become invalid after reset.
    pub unsafe fn reset(&self) {
        let current = &mut *self.current.get();
        current.reset();

        // Reset all full chunks and move them back
        let full = &mut *self.full_chunks.get();
        for chunk in full.iter_mut() {
            chunk.reset();
        }

        *self.allocations.get() = 0;
        *self.bytes_used.get() = 0;
    }

    /// Get allocation statistics
    pub fn stats(&self) -> ArenaStats {
        unsafe {
            ArenaStats {
                allocations: *self.allocations.get(),
                bytes_used: *self.bytes_used.get(),
                chunks_allocated: (*self.full_chunks.get()).len() + 1,
            }
        }
    }
}

impl Default for GridArena {
    fn default() -> Self {
        Self::new()
    }
}

/// Arena statistics
#[derive(Clone, Copy, Debug)]
pub struct ArenaStats {
    pub allocations: u64,
    pub bytes_used: usize,
    pub chunks_allocated: usize,
}

/// Handle to an arena-allocated grid
///
/// SAFETY: Must not outlive the arena
pub struct ArenaGrid<'a> {
    ptr: NonNull<u8>,
    width: u8,
    height: u8,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> ArenaGrid<'a> {
    /// Create a new arena-allocated grid
    ///
    /// # Safety
    /// The arena must outlive this grid
    pub unsafe fn new(arena: &'a GridArena, width: u8, height: u8) -> Self {
        let ptr = arena.alloc_grid();

        // Zero-initialize the data portion
        std::ptr::write_bytes(ptr.as_ptr(), 0, 2048);

        // Write metadata
        let meta_ptr = ptr.as_ptr().add(2048) as *mut u64;
        *meta_ptr = ((width as u64) << 8) | (height as u64);

        Self {
            ptr,
            width,
            height,
            _marker: std::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub fn width(&self) -> u8 {
        self.width
    }

    #[inline(always)]
    pub fn height(&self) -> u8 {
        self.height
    }

    #[inline(always)]
    fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), 2048) }
    }

    #[inline(always)]
    fn data_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), 2048) }
    }

    #[inline(always)]
    pub fn get(&self, x: u8, y: u8) -> u8 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        let idx = (y as usize) * 32 + (x as usize) / 2;
        let byte = self.data()[idx];
        if x % 2 == 0 {
            byte & 0x0F
        } else {
            byte >> 4
        }
    }

    #[inline(always)]
    pub fn set(&mut self, x: u8, y: u8, value: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = (y as usize) * 32 + (x as usize) / 2;
        let data = self.data_mut();
        if x % 2 == 0 {
            data[idx] = (data[idx] & 0xF0) | (value & 0x0F);
        } else {
            data[idx] = (data[idx] & 0x0F) | ((value & 0x0F) << 4);
        }
    }

    /// Clone into the same arena
    pub unsafe fn clone_in(&self, arena: &'a GridArena) -> Self {
        let ptr = arena.alloc_grid();

        // Copy all data including metadata
        std::ptr::copy_nonoverlapping(self.ptr.as_ptr(), ptr.as_ptr(), GRID_ALLOC_SIZE);

        Self {
            ptr,
            width: self.width,
            height: self.height,
            _marker: std::marker::PhantomData,
        }
    }
}

/// Compact state for hash maps (uses less memory)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompactState {
    /// FNV-1a hash of the grid
    hash: u64,
    /// Dimensions packed into u16
    dims: u16,
}

impl CompactState {
    pub fn from_grid(grid: &crate::grid::Grid) -> Self {
        Self {
            hash: grid.hash(),
            dims: ((grid.width as u16) << 8) | (grid.height as u16),
        }
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_alloc() {
        let arena = GridArena::new();

        unsafe {
            let mut g1 = ArenaGrid::new(&arena, 5, 5);
            g1.set(2, 2, 3);
            assert_eq!(g1.get(2, 2), 3);

            let g2 = ArenaGrid::new(&arena, 10, 10);
            assert_eq!(g2.get(0, 0), 0);

            let stats = arena.stats();
            assert_eq!(stats.allocations, 2);
        }
    }

    #[test]
    fn test_arena_reset() {
        let arena = GridArena::new();

        unsafe {
            for _ in 0..100 {
                let _ = ArenaGrid::new(&arena, 64, 64);
            }

            let stats_before = arena.stats();
            assert_eq!(stats_before.allocations, 100);

            arena.reset();

            let stats_after = arena.stats();
            assert_eq!(stats_after.allocations, 0);
        }
    }
}
