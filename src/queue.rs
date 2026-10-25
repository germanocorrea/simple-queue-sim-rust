use num::Unsigned;
use std::ops::{AddAssign, SubAssign};

use crate::random::RandomGenerator;

struct Interval(f32, f32);

struct Simulation<T: Unsigned> {
    global_time: T,
    random_generator: RandomGenerator,
    queues: Vec<Queue<T>>,
}

impl<T: PartialOrd + SubAssign + AddAssign + Unsigned> Simulation<T> {
    fn new(rand_seed: u128, queues: Vec<Queue<T>>) -> Simulation<T> {
        let rand = RandomGenerator::new(rand_seed);
        return Simulation {
            global_time: T::zero(),
            random_generator: rand,
            queues: queues,
        };
    }
}

struct Queue<T: Unsigned> {
    name: String,
    id: T,
    servers: T,
    waiting: T, // status
    state_times: Vec<f32>,
    service_interval: Interval,
    routes: Vec<Route<T>>,
    finite: Option<FiniteQueue<T>>,
    arrivable: Option<ArrivableQueue>,
}

impl<T: PartialOrd + SubAssign + AddAssign + Unsigned> Queue<T> {
    fn enter(&mut self) {
        self.waiting += T::one();
    }

    fn exit(&mut self) {
        self.waiting -= T::one();
    }

    fn lost(&mut self) {
        match self.finite.as_mut() {
            Some(f) => f.lost(),
            None => {}
        }
    }

    fn has_space(&self) -> bool {
        match self.finite.as_ref() {
            Some(f) => self.waiting < f.capacity,
            None => true,
        }
    }
}

struct FiniteQueue<T> {
    capacity: T,
    lost: T,
}

impl<T: PartialOrd + SubAssign + AddAssign + Unsigned> FiniteQueue<T> {
    fn lost(&mut self) {
        self.lost += T::one();
    }
}

struct ArrivableQueue {
    arrival_interval: Interval,
    first_arrival: f32,
}

struct Route<T> {
    destination_queue_id: T,
    probability: f64,
}

enum EventType {
    ARRIVAL,
    EXIT,
    PASS,
}

struct Event<T> {
    event_type: EventType,
    random_generated: f64,
    time_to_ocurr: f32,
    origin_queue_id: Option<T>,
    destination_queue_id: Option<T>,
}
