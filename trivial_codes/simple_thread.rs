use std::thread;

fn main()
{
    let handle = thread::spawn(|| {
        println!("Hello, world");
    });

    handle.join().unwrap();
}
