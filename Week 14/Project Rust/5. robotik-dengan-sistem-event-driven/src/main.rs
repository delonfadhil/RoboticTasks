use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
enum Event {
    ObstacleDetected,
    TargetChanged(i32, i32),
    Idle,
}

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    target: (i32, i32),
}

impl Robot {
    fn new() -> Self {
        Robot {
            position: (0, 0),
            target: (5, 5),
        }
    }

    fn move_towards_target(&mut self) {
        if self.position.0 < self.target.0 {
            self.position.0 += 1;
        } else if self.position.0 > self.target.0 {
            self.position.0 -= 1;
        }

        if self.position.1 < self.target.1 {
            self.position.1 += 1;
        } else if self.position.1 > self.target.1 {
            self.position.1 -= 1;
        }

        println!("Robot moved to position: ({}, {})", self.position.0, self.position.1);
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::ObstacleDetected => {
                println!("Obstacle detected! Stopping.");
            }
            Event::TargetChanged(x, y) => {
                println!("Target changed to: ({}, {})", x, y);
                self.target = (x, y);
            }
            Event::Idle => {
                println!("No events. Robot is idle.");
            }
        }
    }
}

fn main() {
    let robot = Arc::new(Mutex::new(Robot::new()));
    let events = Arc::new(Mutex::new(Vec::new()));

    let robot_clone = Arc::clone(&robot);
    let events_clone = Arc::clone(&events);

    // Thread to simulate environment changes
    thread::spawn(move || {
        let mut counter = 0;
        loop {
            {
                let mut events = events_clone.lock().unwrap();
                if counter % 5 == 0 {
                    events.push(Event::ObstacleDetected);
                } else if counter % 7 == 0 {
                    events.push(Event::TargetChanged(10, 10));
                } else {
                    events.push(Event::Idle);
                }
            }
            counter += 1;
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Main thread to handle robot actions
    loop {
        let event_opt = {
            let mut events = events.lock().unwrap();
            events.pop()
        };

        if let Some(event) = event_opt {
            let mut robot = robot.lock().unwrap();
            robot.handle_event(event.clone());

            if matches!(event, Event::TargetChanged(_, _)) {
                while robot.position != robot.target {
                    robot.move_towards_target();
                    thread::sleep(Duration::from_millis(500));
                }
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}
