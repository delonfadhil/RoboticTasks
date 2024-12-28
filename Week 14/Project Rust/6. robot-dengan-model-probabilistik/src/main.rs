use rand::Rng;
use std::f64::consts::PI;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Debug)]
struct Particle {
    x: f64,
    y: f64,
    theta: f64,
    weight: f64,
}

impl Particle {
    fn new(x: f64, y: f64, theta: f64, weight: f64) -> Self {
        Particle { x, y, theta, weight }
    }

    fn move_particle(&mut self, delta_dist: f64, delta_theta: f64) {
        self.theta += delta_theta;
        self.theta %= 2.0 * PI;
        self.x += delta_dist * self.theta.cos();
        self.y += delta_dist * self.theta.sin();
    }

    fn update_weight(&mut self, sensor_data: (f64, f64), map: &dyn Fn(f64, f64) -> f64) {
        let expected_measurement = map(self.x, self.y);
        let sensor_measurement = sensor_data.0;
        let variance = sensor_data.1;

        // Gaussian probability calculation
        let diff = sensor_measurement - expected_measurement;
        self.weight = (-diff * diff / (2.0 * variance)).exp() / (2.0 * PI * variance).sqrt();
    }
}

struct Robot {
    particles: Vec<Particle>,
    map: Box<dyn Fn(f64, f64) -> f64>,
}

impl Robot {
    fn new(num_particles: usize, map: Box<dyn Fn(f64, f64) -> f64>) -> Self {
        let mut rng = rand::thread_rng();
        let particles = (0..num_particles)
            .map(|_| Particle::new(rng.gen_range(0.0..10.0), rng.gen_range(0.0..10.0), rng.gen_range(0.0..2.0 * PI), 1.0 / num_particles as f64))
            .collect();
        Self { particles, map }
    }

    fn move_particles(&mut self, delta_dist: f64, delta_theta: f64) {
        for particle in &mut self.particles {
            let noise_dist: f64 = rand::thread_rng().gen_range(-0.1..0.1);
            let noise_theta: f64 = rand::thread_rng().gen_range(-0.05..0.05);
            particle.move_particle(delta_dist + noise_dist, delta_theta + noise_theta);
        }
    }

    fn update_weights(&mut self, sensor_data: (f64, f64)) {
        for particle in &mut self.particles {
            particle.update_weight(sensor_data, &self.map);
        }

        let total_weight: f64 = self.particles.iter().map(|p| p.weight).sum();
        for particle in &mut self.particles {
            particle.weight /= total_weight;
        }
    }

    fn resample(&mut self) {
        let mut rng = rand::thread_rng();
        let weights: Vec<f64> = self.particles.iter().map(|p| p.weight).collect();
        let mut new_particles = Vec::new();
        for _ in 0..self.particles.len() {
            let random_weight: f64 = rng.gen_range(0.0..1.0);
            let mut cumulative_weight = 0.0;
            for particle in &self.particles {
                cumulative_weight += particle.weight;
                if cumulative_weight >= random_weight {
                    new_particles.push(Particle::new(particle.x, particle.y, particle.theta, 1.0 / self.particles.len() as f64));
                    break;
                }
            }
        }
        self.particles = new_particles;
    }

    fn best_estimate(&self) -> Particle {
        self.particles
            .iter()
            .max_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap())
            .unwrap()
            .to_owned()
    }
}

fn main() {
    let map = |x: f64, y: f64| {
        // Simplistic map function that returns the distance to (5, 5) as measurement
        ((x - 5.0).powi(2) + (y - 5.0).powi(2)).sqrt()
    };

    let mut robot = Robot::new(1000, Box::new(map));
    
    for step in 0..10 {
        // Simulate movement
        robot.move_particles(1.0, 0.1);

        // Simulate sensor measurement with some noise
        let sensor_data = (5.0, 0.5); // Example measurement: distance to goal, variance

        // Update weights based on sensor data
        robot.update_weights(sensor_data);

        // Resample particles
        robot.resample();

        // Get the best estimate
        let best_particle = robot.best_estimate();
        println!("Step {}: Best estimate at ({:.2}, {:.2})", step, best_particle.x, best_particle.y);
    }
}
