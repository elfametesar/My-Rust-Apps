use std::env;

fn human(mut size: f64) {
    for unit in [' ', 'K', 'M', 'G', 'T', 'P', 'E', 'Z', 'Y' ] {
        if size < 1024_f64 {
            let result = format!("{:.2}", size);
            if result.ends_with("00") { 
                println!("{}{}", &result[..result.len()-3], unit)
            } else { println!("{}{}", &result[..result.len()-1], unit) } 
            return
        }
        size = size/1024_f64;
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
