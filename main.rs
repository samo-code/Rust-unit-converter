
use clap::Parser;
// use value::*;
use value::{Convert,Convertion};



mod cli;
mod value;

fn main() {
    // loop{
    //     println!("Please select the unit you want to convert");
    //     println!("1. meters to centimeters");
    //     println!("2. meters to kilometers");
    //     println!("3. miles to kilometers");
    //     println!("4. grams to kilograms");
    //     println!("5. pounds to kilograms");

        
    // }
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

        // let meters = cli.value;
        // let m = meters * 100 as f32;

        // println!("{} meters is {} centimeters.",cli.value,m);

    //     match cli.cmd{
    //     cli::Commands::MetersToCms { value }=>{
            
        
    //        println!(" {} meters is {} centimeters",value,value::ConvertImpl::meters_to_cms(&));
    //     },
    //     cli::Commands::MetersToKms{value}=>{
            
    //         println!(" {} kms",value);
    //     },
    //     cli::Commands::MilesToKm { value }=>{
            
    //         println!("{} kms",value)
    //     },
    //     cli::Commands::GramsToKgs { value }=>{
            
    //         println!(" {} kgs",value)
    //     },
    //     cli::Commands::PoundsToKgs { value }=>{
            
    //         println!(" {} kgs",value) 
    //     }
    // }
        
}
