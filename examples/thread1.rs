use anyhow::{anyhow, Ok, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Message {
    idx: usize,
    value: usize,
}

impl Message {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // 创建producer线程
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx); // 关闭多余的一个tx，否则不退出

    // 创建consumer线程
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Received: {:?}", msg);
        }
        println!("Consumer exit")
    });
    consumer
        .join()
        .map_err(|e| anyhow!("Consumer thread panicked: {:?}", e))?;
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Message>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Message::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        // 模拟随机值退出
        if rand::random::<u8>() % 10 == 0 {
            println!("Producer {} exit", idx);
            break;
        }
    }
    Ok(())
}
