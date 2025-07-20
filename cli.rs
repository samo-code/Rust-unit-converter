use clap::{Parser,Subcommand};

#[derive(Parser)]
#[command(name = "Unit-converter")]
#[command(version = "1.0")]
#[command(about = "A simple CLI unit converter")]
#[command(long_about ="none")]
pub struct Cli{
    
    #[command(subcommand)]
    pub command:Commands,
}

#[derive(Subcommand)]
pub enum Commands{
   
    MetersToCms{
        #[arg(short,long)]
        value:f32,
    },
    MetersToKms{
        #[arg(short,long)]
        value:f32,
    },
    MilesToKm{
        #[arg(short,long)]
        value:f32,
    },
    GramsToKgs{
        #[arg(short,long)]
        value:f32,
    },
    PoundsToKgs{
        #[arg(short,long)]
        value:f32,
    }
}

