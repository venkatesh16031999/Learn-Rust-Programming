use crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread::{self, JoinHandle};

enum ChildThreadMsg {
    ProcessSum(i32, i32),
    ProcessSub(i32, i32),
    Quit,
}

enum MainThreadMsg {
    ResultSum(i32),
    ResultSub(i32),
    Quit,
}

fn spawn_child_thread(
    main_sender: Sender<MainThreadMsg>,
    child_receiver: Receiver<ChildThreadMsg>,
) -> JoinHandle<()> {
    thread::spawn(move || loop {
        match child_receiver.recv() {
            Ok(process) => match process {
                ChildThreadMsg::ProcessSum(a, b) => {
                    let sum_result = a + b;
                    if let Ok(()) = main_sender.try_send(MainThreadMsg::ResultSum(sum_result)) {
                        println!("Summation process is in-progress");
                    } else {
                        println!("Summation process is failed");
                    }
                }
                ChildThreadMsg::ProcessSub(a, b) => {
                    let sub_result = a - b;
                    if let Ok(()) = main_sender.try_send(MainThreadMsg::ResultSub(sub_result)) {
                        println!("Subtraction process is in-progress");
                    } else {
                        println!("Subtraction process is failed");
                    }
                }
                ChildThreadMsg::Quit => {
                    main_sender.send(MainThreadMsg::Quit).expect("Exception 4");
                    println!("Mathematic operation are completed, wait for the result:");
                    break;
                }
            },
            Err(error) => println!("Process quited due to {:?}", error),
        }
    })
}

fn main() {
    let (child_sender, child_receiver) = unbounded();
    let (main_sender, main_receiver) = unbounded();

    let child_thread = spawn_child_thread(main_sender, child_receiver);

    child_sender
        .send(ChildThreadMsg::ProcessSum(10, 20))
        .expect("Exception 1");
    child_sender
        .send(ChildThreadMsg::ProcessSub(100, 20))
        .expect("Exception 2");
    child_sender
        .send(ChildThreadMsg::Quit)
        .expect("Exception 3");

    loop {
        match main_receiver.recv() {
            Ok(result) => match result {
                MainThreadMsg::ResultSum(sum) => {
                    println!("Summation Result : {:?}", sum);
                }
                MainThreadMsg::ResultSub(sub) => {
                    println!("Subtraction Result : {:?}", sub);
                }
                MainThreadMsg::Quit => {
                    println!("Process Completed");
                    break;
                }
            },
            Err(_) => println!("Something Went Wrong"),
        }
    }

    child_thread.join().expect("Exception 4");
}
