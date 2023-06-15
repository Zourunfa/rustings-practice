// threads3.rs
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    /*
    这里需要使用 tx.clone() 方法的原因是，mpsc::Sender 类型是不可复制（non-copyable）的。这意味着，如果我们直接将 tx 变量传递给 send_tx 函数，那么在函数中使用 tx 变量时，
    就会将其所有权转移给函数，导致在函数返回后无法再次使用 tx 变量。
     */
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    // mpsc::channel() 函数创建了一个通道，并返回两个端点：一个发送者（Sender）和一个接收者（Receiver）
    //将发送者和接收者分别赋值给了 tx 和 rx 变量。
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
