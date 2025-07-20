use clap::Parser;
use value::{Convert,Convertion};

mod cli;
mod value;

fn main() {
    
        let args = cli::Cli::parse();

        match args.command{
            cli::Commands::MetersToCms{value} =>{
                let meters = Convert::meters_to_cms(value);
               
                println!("{} m is {} cm",value,meters);
            },
            cli::Commands::MetersToKms { value }=>{
                let meters = Convert::meters_to_kms(value);
                println!("{} m is {} km",value,meters);
            },
            cli::Commands::MilesToKm { value }=>{
                let miles = Convert::miles_to_km(value);
                println!("{} miles is {} km",value,miles);

            },
            cli::Commands::GramsToKgs { value }=>{
                let grams = Convert::grams_to_kgs(value);
                println!("{} g is {} kgs",value,grams);
            },
            cli::Commands::PoundsToKgs { value }=>{
                let pounds = Convert::pounds_to_kgs(value);
                println!("{} lbs is {} kgs",value,pounds);
            },
        }

}
