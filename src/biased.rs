use piston_window::{PistonWindow, WindowSettings};
use plotters::prelude::*;
use plotters_piston::draw_piston_window;

use std::{
    collections::vec_deque::VecDeque,
    thread::{self, sleep_ms},
    time::Duration,
};

use rand_core::{OsRng, RngCore};
use std::{collections::BTreeMap, u8, usize};

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Fast introduction to modulo biases", [450, 300])
            .samples(128)
            .build()
            .unwrap();

    let mut epoch = 0; // frame counter

    const NUM_SAMPLES: u32 = 15_000;

    let mut rng = OsRng;
    let mut buf: [u8; 1] = [0; 1];

    let max = std::u8::MAX as u32;

    // We want to sample in the range [0, upper)
    let mut upper: u8 = 100;

    loop {
        if upper >= u8::MAX {
            break;
        }

        let mut bad_point_counter: BTreeMap<u32, u32> = BTreeMap::new();
        for num in 0..max {
            bad_point_counter.insert(num, 0);
        }
        let mut bad_state: VecDeque<BTreeMap<u32, u32>> =
            VecDeque::with_capacity(NUM_SAMPLES as usize); // Cool effect I found
                                                           //
        let incr = upper as u32 + 50;
        let y_scale_max = (NUM_SAMPLES / upper as u32) * 2;

        for i in 0..NUM_SAMPLES {
            rng.fill_bytes(&mut buf);
            let raw = u8::from_be_bytes(buf);
            let bad_sampled_number = raw % upper;

            let bad_counter = bad_point_counter
                .entry(bad_sampled_number as u32)
                .or_insert(0);
            *bad_counter += 1;

            if i % incr == 0 {
                bad_state.push_back(bad_point_counter.clone());
            }
        }

        println!("SAMPLING DONE!");
        loop {
            if epoch >= NUM_SAMPLES {
                thread::sleep(Duration::from_secs(2));
                break;
            }

            draw_piston_window(&mut window, |b| {
                let root = b.into_drawing_area();
                root.fill(&WHITE)?;

                let title = format!("Sampling frequency per number (mod {})", upper);

                let mut cc = ChartBuilder::on(&root)
                    .margin(10u32)
                    .caption(title, ("sans-serif", 30u32))
                    .x_label_area_size(40u32)
                    .y_label_area_size(50u32)
                    .build_cartesian_2d((0..max).into_segmented(), 0u32..y_scale_max)
                    .expect("failed to setup chart");

                println!("\tsetting up mesh");
                cc.configure_mesh()
                    .disable_x_mesh()
                    .bold_line_style(&WHITE.mix(0.3))
                    .y_desc("Count")
                    .x_desc("Bucket")
                    .axis_desc_style(("sans-serif", 15u32))
                    .draw()
                    .expect("failed to setup chart");

                let b_p_c = bad_state.pop_front().unwrap();

                println!("\tload sample data at epoch={epoch}");
                cc.draw_series(
                    Histogram::vertical(&cc)
                        .margin(1u32)
                        .style(RED.mix(0.9).filled())
                        .data(b_p_c),
                )
                .expect("failed to draw series");

                cc.configure_series_labels()
                    .background_style(&WHITE.mix(0.8))
                    .border_style(&BLACK)
                    .draw()?;

                epoch += incr;
                Ok(())
            })
            .unwrap();
        }

        upper += 12;
        epoch = 0;
    }
}
