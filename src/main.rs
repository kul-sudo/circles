use macroquad::prelude::{
    draw_circle, draw_text, is_key_pressed, measure_text, next_frame, screen_height, screen_width,
    set_fullscreen, vec2, KeyCode, Vec2, ORANGE, WHITE, YELLOW,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{
    f32::consts::PI,
    sync::LazyLock,
    time::{Duration, Instant},
};

static INIT_POS: LazyLock<Vec2> =
    LazyLock::new(|| vec2(screen_width() / 2.0, screen_height() / 2.0));

const PARTICLES_N: usize = 100;

const PARTICLE_RADIUS: f32 = 25.0;
const FONT_SIZE: u16 = 80;
static MAX_PARTICLE_RADIUS: f32 = 500.0 - PARTICLE_RADIUS;

enum ParticlesPlacementStrategy {
    Trigonometrical,
    WhileLoopBased,
}

fn put_particles(
    strategy: ParticlesPlacementStrategy,
    particles: &mut Vec<Vec2>,
    rng: &mut StdRng,
) -> Duration {
    particles.clear();

    let start = Instant::now();

    for _ in 0..PARTICLES_N {
        match strategy {
            ParticlesPlacementStrategy::Trigonometrical => {
                let random_radius = (rng.gen_range(0.0..1.0) as f32).sqrt() * MAX_PARTICLE_RADIUS;
                let random_angle = rng.gen_range(0.0..2.0 * PI);
                let adjusted_pos = vec2(
                    INIT_POS.x + random_radius * random_angle.cos(),
                    INIT_POS.y + random_radius * random_angle.sin(),
                );

                particles.push(adjusted_pos)
            }
            ParticlesPlacementStrategy::WhileLoopBased => {
                let mut random_pos;

                loop {
                    random_pos = vec2(
                        rng.gen_range(
                            INIT_POS.x - MAX_PARTICLE_RADIUS..INIT_POS.x + MAX_PARTICLE_RADIUS,
                        ),
                        rng.gen_range(
                            INIT_POS.y - MAX_PARTICLE_RADIUS..INIT_POS.y + MAX_PARTICLE_RADIUS,
                        ),
                    );

                    if random_pos.distance(*INIT_POS) < MAX_PARTICLE_RADIUS {
                        break;
                    }
                }

                particles.push(random_pos);
            }
        }
    }

    return start.elapsed();
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut rng = StdRng::from_rng(&mut rand::thread_rng()).unwrap();

    let mut particles = Vec::with_capacity(PARTICLES_N);

    set_fullscreen(true);
    next_frame().await;

    let mut time = None;

    loop {
        if is_key_pressed(KeyCode::E) {
            time = Some(put_particles(
                ParticlesPlacementStrategy::Trigonometrical,
                &mut particles,
                &mut rng,
            ));
        }

        if is_key_pressed(KeyCode::R) {
            time = Some(put_particles(
                ParticlesPlacementStrategy::WhileLoopBased,
                &mut particles,
                &mut rng,
            ));
        }

        if let Some(time) = time {
            let text = format!("{}ns", time.as_nanos());
            let measured = measure_text(&text, None, FONT_SIZE, 1.0);
            draw_text(&text, 0.0, measured.height, FONT_SIZE as f32, WHITE);
        }

        draw_circle(
            INIT_POS.x,
            INIT_POS.y,
            MAX_PARTICLE_RADIUS + PARTICLE_RADIUS,
            YELLOW,
        );

        for particle_pos in &particles {
            draw_circle(particle_pos.x, particle_pos.y, PARTICLE_RADIUS, ORANGE);
        }

        next_frame().await
    }
}
