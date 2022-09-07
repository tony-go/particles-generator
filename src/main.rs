use ::std::alloc::{GlobalAlloc, Layout, System};
use ::std::time::Instant;

use graphics::{clear, rectangle};
// third-party dependencies
use graphics::math::{add, mul_scalar, Vec2d};
use piston_window::{PistonWindow, WindowSettings};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;

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
 * PARTICLES & WORLD
 */

struct World {
    currrent_turn: u64,
    height: f64,
    width: f64,
    rng: ThreadRng,
    particles: Vec<Box<Particle>>,
}

struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    velocity: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Particle {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;

        let x_velo = 0.0;
        let y_velo = rng.gen_range(-2.0..0.0);

        let x_accel = 0.0;
        let y_accel = rng.gen_range(0.0..0.15);

        Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y].into(),
            velocity: [x_velo, y_velo].into(),
            acceleration: [x_accel, y_accel].into(),
            color: [1.0, 1.0, 1.0, 0.99],
        }
    }

    fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
        self.acceleration = mul_scalar(self.acceleration, 0.7);
        self.color[3] *= 0.995;
    }
}

impl World {
    fn new(width: f64, height: f64) -> World {
        World {
            width,
            height,
            particles: Vec::<Box<Particle>>::new(),
            currrent_turn: 0,
            rng: thread_rng(),
        }
    }

    fn add_particles(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let p = Particle::new(&self);
            let boxed_p = Box::new(p);
            self.particles.push(boxed_p);
        }
    }

    fn remove_particles(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete: Option<usize> = None;

            let particle_iter = self.particles.iter().enumerate();

            for (i, particle) in particle_iter {
                if particle.color[3] < 0.02 {
                    to_delete = Some(i);
                }
                break;
            }

            if let Some(i) = to_delete {
                self.particles.remove(i);
            } else {
                self.particles.remove(0);
            }
        }
    }

    fn update(&mut self) {
        let rand = self.rng.gen_range(-3..=3);

        if rand > 0 {
            self.add_particles(rand);
        } else {
            self.remove_particles(rand);
        }

        self.particles.shrink_to_fit();
        for part in &mut self.particles {
            part.update();
        }
        self.currrent_turn += 1;
    }
}
/****
 * EXECUTION
 */

fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create a window");

    let mut world = World::new(width, height);
    world.add_particles(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for part in &mut world.particles {
                let size = [part.position[0], part.position[1], part.width, part.height];
                rectangle(part.color, size, ctx.transform, renderer);
            }
        });
    }
}
