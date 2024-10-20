use std::{sync::mpsc, thread, time::Duration};

use anyhow::{anyhow, Result};

const NUM_PRODUCERS: i32 = 10;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Received: {:?}", msg);
            println!("Received idx: {}", msg.idx);
            println!("Received value: {}", msg.value);
        }
    });

    let _ = consumer
        .join()
        .map_err(|e| anyhow!("Failed to join consumer thread {:?}", e));

    Ok(())
}

fn producer(i: i32, tx: mpsc::Sender<Message>) -> Result<()> {
    loop {
        let value = rand::random::<i32>();
        tx.send(Message::new(i, value))?;
        thread::sleep(Duration::from_secs(1));
        if rand::random::<u8>() % 5 == 0 {
            println!("Producer {} is done", i);
            break;
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Message {
    idx: i32,
    value: i32,
}

impl Message {
    fn new(idx: i32, value: i32) -> Self {
        Self { idx, value }
    }
}
