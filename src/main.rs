use colored::Colorize;

fn main() {
    println!("{}", format!("ERIK").blue());
    println!("{}", format!("PLEASE HELP ME").on_blue().red());
    println!("{}", format!("I still havent fixed my app storage bug .-.").on_black().white());
}
