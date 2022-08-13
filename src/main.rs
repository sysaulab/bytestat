//! # Bytestat
//!
//! Bytestat measure randomness of data. Data is processed one byte at a time, 
//! the distribution and interval of each byte is measured. Five metrics are used to measure different aspects
//! of the set. The final score is between 0.0 and 100.0. Good quality random data should score 100 when rounded.

use std::{io::Read};
use libbytestat::Bytestat;

fn main() {

  let mut stats = Bytestat::new();
  let mut counter:u128 = 0;
  let percent = 256 * 4096;

  for x in std::io::stdin().bytes() {
    match x {
        Ok(data) => {
          stats.analyze(data);
          counter += 1;
        },
        Err(err) => {
          eprintln!("{:?}", err);
        }
    }
  }
  /*
  println!("DNZ {:.2}", stats.get_score_non_zero());
  println!("DUN {:.2}", stats.get_score_unique());
  println!("DAM {:.2}", stats.get_score_amplitude());
  println!("ICO {:.2}", stats.get_score_interval_continuity());
  println!("IAM {:.2}", stats.get_score_interval_amplitude());
  */

  println!("{}", counter );
  println!("{}", (percent * 100) );
  println!("{}{:.0}%", if counter < (percent * 100) {"~"} else {""}, stats.get_score() );

}