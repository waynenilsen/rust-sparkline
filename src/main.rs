extern crate sparkline;
extern crate rustc_serialize;
extern crate docopt;

use sparkline::*;

use std::io;
use std::io::BufRead;

use docopt::Docopt;

const USAGE: &'static str = "
sparkr

Usage:
  sparkr [--min=<min>] [--max=<max>] [--theme=<theme>] [--statline] [--gap=<gap>] [<values>...]
  sparkr (-h | --help)
  sparkr --version

Options:
  -h --help       Show this screen.
  --version       Show version.
  --min=<min>     Specify minimum value instead of calculating it.
  --max=<max>     Specify maximum value instead of calculating it.
  --gap=<max>     Gap between symbols [default=1]
  --statline      Show a line of stats after the sparkline.
  --theme=<theme>   What theme to use, 'colour' or 'classic' (default).
  <values>        Just values.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    pub flag_min: Option<f64>,
    pub flag_max: Option<f64>,
    pub flag_gap: Option<usize>,
    pub flag_theme: Option<String>,
    pub flag_statline: bool,
    pub arg_values: Vec<f64>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    //let mut args: Vec<_> = env::args().collect();
    //if args.len() < 2 {
        //println!("{} expects a series of numbers as arguments.", args[0]);
        //std::process::exit(1);
    //}

    //args.remove(0);
    //let good_numbers = parse_numbers(&args);
    
    let mut good_numbers: Vec<_> = args.arg_values;
    if good_numbers.len() == 0 {
        let mut input_numbers : Vec<String> = vec![];
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            match line {
                Ok(l) => {
                    let x : Vec<&str> = l.split(|c: char| c.is_whitespace()).collect();
                    input_numbers.extend(x.iter()
                        .filter(|x| **x != "")
                        .map(|x| x.to_string())
                        );
                },
                Err(_) => {
                    break;
                },
            };
        }
        good_numbers = parse_numbers(&input_numbers);
    }

    let (min, max) = min_max_for_data(&good_numbers, args.flag_min, args.flag_max);

    let theme = match args.flag_theme {
        Some(ref x) if x == "colour" => SparkThemeName::Colour,
        Some(ref x) if x == "classic" => SparkThemeName::Classic,
        Some(ref x) => { println!("Unknown theme {} falling back to classic", x); SparkThemeName::Classic },
        _ => SparkThemeName::Classic,
    };
    let sparky = select_sparkline(theme);

    let gap_vec = match args.flag_gap {
        Some(x) => std::iter::repeat(" ").take(x).collect::<Vec<_>>(),
        None => vec![" "],
    };
    let gap_str: String = gap_vec.into_iter().collect();
    for num in good_numbers.iter() {
        let s = sparky.spark(min, max, *num);
        print!("{}{}", s, gap_str);
    }
    println!("");

    if args.flag_statline {
        println!("min: {}, max: {}, range: {}", min, max, max-min);
    }
}

