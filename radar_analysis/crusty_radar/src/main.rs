use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use memchr::memmem::Finder;
use std::{thread, time::Duration};
fn static_scene_map(frames: &[Vec<u32>]) {

    const WIDTH: usize = 16;
    const HEIGHT: usize = 20;

    const PALETTE: &[u8] = b".'\"^,:;Il!i><~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhaoo*#MW&8%B@$`";

    let mut acc = vec![0f32; WIDTH * HEIGHT];

    let n = frames.len() as f32;

    // integrate all frames
    for frame in frames {
        for i in 0..acc.len() {
            acc[i] += frame[i] as f32;
        }
    }

    // average
    for v in &mut acc {
        *v /= n;
    }

    // compute global mean
    let mean = acc.iter().sum::<f32>() / acc.len() as f32;

    // normalize
    for v in &mut acc {
        *v /= mean;
    }

    // find max for scaling
    let max = acc.iter().cloned().fold(0.0, f32::max);

    println!("\n=== Static Radar Reflectivity Map ===\n");

    for doppler in 0..HEIGHT {

        for range in 0..WIDTH {

            let v = acc[doppler * WIDTH + range] / max;

            let idx = (v * (PALETTE.len() - 1) as f32) as usize;

            print!("{}", PALETTE[idx] as char);
        }

        println!();
    }

    println!();
}
fn render_video(frames: &[Vec<u32>]) {

    for frame in frames {

        // clear screen + move cursor to top
        print!("\x1B[2J\x1B[H");

        render_heatmap(frame);

        thread::sleep(Duration::from_millis(80));
    }
}
const DATA_BIN_SIZE:usize = 1280;
const RANGE_GATE_M: f32 = 0.7;

fn render_heatmap(frame: &[u32]) {

    const PALETTE: &[u8] = b".'\"^,:;Il!i><~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhaoo*#MW&8%B@$`";

;

    let mut buf = [0f32; 320];

    // log compression
    for i in 0..320 {
        buf[i] = (frame[i] as f32 + 1.0).ln();
    }

    // mean subtraction
    let mean: f32 = buf.iter().sum::<f32>() / 320.0;

    for v in &mut buf {
        *v -= mean;
        if *v < 0.0 {
            *v = 0.0;
        }
    }

    let max = buf.iter().cloned().fold(0.0, f32::max).max(1e-6);

    // print range axis
    print!("     ");
    for r in 0..16 {
        if r % 2 == 0 {
            let dist = r as f32 * RANGE_GATE_M;
            print!("{:4.1}", dist);
        } else {
            print!("    ");
        }
    }
    println!();

    // render map
    for doppler in 0..20 {

        print!("{:02} | ", doppler);

        for range in 0..16 {

            let v = buf[doppler * 16 + range] / max;

            let idx = (v * (PALETTE.len() - 1) as f32) as usize;

            print!("{}", PALETTE[idx] as char);
        }

        println!();
    }

    println!("---------------------------");
}
fn reshape_frame(values: Vec<u8>, frame_size: usize){

    let mut frames = Vec::new();

    for frame in values.chunks_exact(frame_size){

        let mut data = Vec::with_capacity(320);

        for chunk in frame.chunks(4){
            let v = u32::from_le_bytes(chunk.try_into().unwrap());
            data.push(v);
        }

        frames.push(data);
    }
    //static_scene_map(&frames)
    render_video(&frames);
}

fn main() {
    let filename = env::args().nth(1).expect("bin file");

    let header = b"\xaa\xbf\x10\x14";
    let tail = b"\xfd\xfc\xdb\xfa";

    let file = File::open(filename).unwrap();
    let mut file = BufReader::new(file);

    let mut data = Vec::new();
    let finder = Finder::new(header);
    let finder_tail= Finder::new(tail);

    loop {

        let buf = file.fill_buf().unwrap();

        if buf.is_empty() {
            break;
        }

        if let Some(i) = finder.find(buf){
            if let Some(t) = finder_tail.find(buf){
                data.extend_from_slice(&buf[i+ header.len()..t]);
                file.consume(i + header.len() + tail.len());
                break;
            }
        }

        data.extend_from_slice(buf);

        let len = buf.len();
        file.consume(len);
    }

//reshape
reshape_frame(data,DATA_BIN_SIZE)
}
