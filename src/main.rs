mod packets;
mod sniffer;
mod traits;

use std::{io, panic, thread};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use crate::packets::datalink::ethernet::EthernetFrame;
use crate::traits::Layer;

fn main() {
    panic::set_hook(Box::new(custom_panic_handler));
    let mut live = sniffer::LiveCapture{
        interfaces: vec![],
        captured_packets: Arc::new(Mutex::new(vec![vec![]])),
        stop: Arc::new(Default::default()),
    };
    let mut live2 = live.clone();
    thread::spawn(move || {
        live2.capture();
    });
    let mut input = String::new();
    loop {
        input.clear();  // Clear the previous input.

        print!("Please enter a random packet NUMBER (or type 'stop' to exit): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input == "stop" {
            live.stop.store(true, Ordering::Relaxed);
            break;
        }
        let trimmed_input : i32 = input.trim().parse::<i32>().unwrap();
        if let Ok(lock) = live.captured_packets.lock(){
            if let Some((outside, inside)) = find_id(&lock, trimmed_input){
                let layer_obj : &dyn Layer = &lock[outside][inside];
                println!("{:?}", layer_obj.get_summary());
            }
        }
    }
}

fn custom_panic_handler(info: &panic::PanicInfo) {
    // Handle the panic, e.g., log it or perform some cleanup.
    println!("Panic occurred: {:?}", info);
}

fn find_id(vectors: &Vec<Vec<EthernetFrame>>, id_to_find: i32) -> Option<(usize, usize)> {
    vectors.iter().enumerate().find_map(|(i, vector)| {
        vector.iter().position(|id| id.id == id_to_find).map(|j| (i, j))
    })
}