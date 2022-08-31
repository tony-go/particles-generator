use::std::alloc::{GlobalAlloc, Layout, System};
use::std::time::{Instant};

/****
 * CustomAllocator
 */

struct ReportingAllocator;

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();

        let time = end - start;
        let bytes_requested = layout.size();

        eprintln!("{}\t{}", bytes_requested, time.as_nanos());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

/***
 * PARTICULES
 */

// TODO

/****
 * EXECUTION
 */

fn main() {
    println!("Hello, world!");
}
