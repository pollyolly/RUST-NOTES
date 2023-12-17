use std::sync::mpsc; //multi producer, single consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); //create channel w/ shared queue 
                                    //return transmitter(txt) and reciever(rx)

    //push data to channel
    tx.send("Send job to do: 1").unwrap(); //send data to channel
    tx.send("Another job: 2").unwrap();

    let worker = thread::spawn(move || {
        loop {
            let job = rx.recv();//reads/pops data from channel
            match job {
                Ok(job) => println!("Job: {}", job),
                Err(_) => break,
            }
        }
    });

    //push more data to the channel
    tx.send("Yet another job").unwrap();

    worker.join().unwrap();
}
