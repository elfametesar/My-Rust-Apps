use std::env;

fn human(mut size: f64) {
    for unit in [' ', 'K', 'M', 'G', 'T', 'P', 'E', 'Z', 'Y' ] {
        if size < 1024.0 {
            println!("{}{}", size, unit);
            return;
        }
        size = (size / 1024.0 * 10.0).trunc() / 10.0;
   }
}

fn main() {
    let args: Vec<f64> = env::args()
        .filter(|a| a.parse::<f64>().is_ok())
        .map(|b| b.parse().unwrap()).collect();
    for arg in args.iter() {
        human(*arg);
    }
    
}
