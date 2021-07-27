use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use ray_tracing::render::{ppm, Color, Image};
use setup::REPETITION;

use crate::setup::{IMAGE_HEIGHT, IMAGE_WIDTH};

mod setup;

fn do_calculations() -> thread::Result<Vec<Color>> {
    // ProgressBar
    let mp = MultiProgress::new();

    const DRAW_RATE: u64 = 15;

    let sty = ProgressStyle::default_bar().template(
        "{spinner} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {percent}% ~{eta}",
    );

    let set = |pb: &ProgressBar| {
        pb.set_style(sty.clone());
        pb.set_draw_rate(DRAW_RATE);
    };

    let pb_run = mp.add(ProgressBar::new(REPETITION as u64));
    set(&pb_run);

    let pb_curr = mp.add(ProgressBar::new(IMAGE_HEIGHT as u64));
    set(&pb_curr);

    let pb_run1 = pb_run.clone();
    let pb_curr1 = pb_curr.clone();

    let ab = Arc::new(AtomicBool::new(true));
    let ab1 = ab.clone();

    let ticker = thread::spawn(move || {
        let s = Duration::from_millis(1000 / DRAW_RATE);

        while ab1.load(Ordering::Acquire) {
            pb_run1.tick();
            pb_curr1.tick();
            thread::sleep(s);
        }
    });

    let data = thread::spawn(move || {
        let res = setup::run(pb_run.clone(), pb_curr.clone());
        pb_curr.finish();
        pb_run.finish();
        res
    });

    mp.join().map_err(|err| Box::new(err) as _)?;

    ab.store(false, Ordering::Release);

    ticker.join()?;

    data.join()
}

fn main() {
    // Image
    let path = "main";

    println!("Running");

    let data = do_calculations().expect("unable to get the data, due to some error");

    println!("Writing data");
    let img = Image::new(&data, IMAGE_HEIGHT, IMAGE_WIDTH);

    ppm::save(img, path).expect("Something went terribly wrong here");
    println!("Done");
}
