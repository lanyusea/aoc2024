use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::time::Instant;

fn ntm(target :f64) -> f64 {
    let tolerance:f64 = 1e-10; // Tolerance for convergence
    let mut x:f64 = 5.0; // Initial guess

    loop {
        let next_x:f64 = (x * x + target) / (2.0 * x);
        if (next_x - x).abs() < tolerance {
            break;
        }
        x = next_x;
    }

    x
}
fn main() {
    // Create a channel for communication between threads
    let (tx, rx) = mpsc::channel();

    // Spawn the thread to generate random characters
    let tx_clone = tx.clone();
    let handle_generator = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        let (mut start, mut duration): (Instant, Duration);
        for _ in 0..100 { // Generate 100 random characters
            let random_num: f64 = rng.gen_range(0.0..100.0);
            start = Instant::now();
            let result = ntm(random_num);
            duration = start.elapsed();
            tx_clone.send((random_num, result, duration)).unwrap();
        }
    });

    // Spawn the thread to save characters to a file
    let handle_saver = thread::spawn(move || {
        let mut file = File::create("output.txt").unwrap();
        while let Ok((random_, result_, duration_)) = rx.recv() {
            writeln!(file, "The sqrt of {} is {} after {:?} calculation", random_, result_, duration_).unwrap();
        }
    });

    // Wait for both threads to finish
    handle_generator.join().unwrap();
    drop(tx); // Drop the sender to signal the receiver to stop
    handle_saver.join().unwrap();
}