use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  // Threads
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });
  handle.join().unwrap();
  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
  // Move variables
  let v = vec![1, 2, 3];
  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });
  handle.join().unwrap();
  //
  // Channels
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
   let val = String::from("hi");
   tx.send(val).unwrap();
   // Invalid due borrowing
   // println!("val is {}", val);
 });
 let received = rx.recv().unwrap();
 println!("Got: {}", received);
 //
 // Sending Multiple Values
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];
    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });
  for received in rx {
    println!("Got: {}", received);
  }
  //
  // Creating Multiple Producers by Cloning the Transmitter
  let (tx, rx) = mpsc::channel();
  let tx1 = mpsc::Sender::clone(&tx);
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];
    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });
  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ];
    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });
  for received in rx {
    println!("Got: {}", received);
  }
  //
  // Mutex
  let m = Mutex::new(5);
  {
    let mut num = m.lock().unwrap();
    *num = 6;
  }
  println!("m = {:?}", m);
  //
  // Shared-state
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];
  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }
  for handle in handles {
    handle.join().unwrap();
  }
  println!("Result: {}", *counter.lock().unwrap());
}
