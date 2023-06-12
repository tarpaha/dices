use std::{thread, time};
use std::sync::mpsc;
use rand::RngCore;

extern crate num_cpus;

#[derive(Debug)]
struct Data {
    wins: u128,
    total: u128
}

fn main() {
    let n_threads = num_cpus::get();
    println!("Running on {} threads", n_threads);

    let (tx, rx) = mpsc::channel();
    
    for _ in 0..n_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            play(tx);
        });
    }
    
    let mut global_wins = 0;
    let mut  global_total = 0;
   
    let mut wins = 0; let mut total = 0; let mut count = 0;
    let mut start = time::Instant::now();
    loop {
        let data = rx.recv().unwrap();
        
        wins += data.wins;
        total += data.total;
        count += 1;
        
        if count == n_threads {
            global_wins += wins;
            global_total += total;
            
            println!(
                "{:.7}, per second = {}, total = {}",
                (global_wins as f64)/(global_total as f64),
                total * 1000 / start.elapsed().as_millis(),
                global_total);

            wins = 0; total = 0; count = 0;
            start = time::Instant::now();
        }
    }
}

fn play(tx: mpsc::Sender<Data>) {
    let mut rng = rand::thread_rng();
    let mut total = 1000000;
    loop {
        let mut wins = 0;
        let start = time::Instant::now();
        for _ in 0..total {
            let pyramidal_peter_score =
                (1 + rng.next_u32() % 4) +
                (1 + rng.next_u32() % 4) +
                (1 + rng.next_u32() % 4) +
                (1 + rng.next_u32() % 4) +
                (1 + rng.next_u32() % 4) +
                (1 + rng.next_u32() % 4) +
                (1 + rng.next_u32() % 4) +
                (1 + rng.next_u32() % 4) +
                (1 + rng.next_u32() % 4);
                    
            let cubic_colin_score =
                (1 + rng.next_u32() % 6) +
                (1 + rng.next_u32() % 6) +
                (1 + rng.next_u32() % 6) +
                (1 + rng.next_u32() % 6) +
                (1 + rng.next_u32() % 6) +
                (1 + rng.next_u32() % 6);
            
            if pyramidal_peter_score > cubic_colin_score {
                wins += 1;
            }
        }
        let elapsed = start.elapsed().as_millis();
        tx.send(Data { wins, total }).unwrap();
        total = total * 1000 / elapsed;
    }
}