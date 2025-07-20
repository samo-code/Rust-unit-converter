use clap::Parser;

#[derive(Parser,Debug)]
pub struct Convert{
    pub value:f32,
    
}

pub trait Convertion{
     fn meters_to_cms(value:f32) -> f32;
     fn meters_to_kms(value:f32) -> f32;
     fn miles_to_km(value:f32) -> f32;
     fn grams_to_kgs(value:f32) -> f32;
     fn pounds_to_kgs(value:f32) -> f32;

    

}

impl Convertion for Convert{

    fn meters_to_cms(value:f32)->f32{
        
        return value * 100 as f32;
    }

    fn meters_to_kms(value:f32)->f32{
        
        return value / 1000 as f32;
    }
    fn miles_to_km(value:f32)->f32{
        
        return value * 1.60 as f32;
    }
    fn grams_to_kgs(value:f32)->f32{
        
        return value / 1000 as f32;
    }
    fn pounds_to_kgs(value:f32)->f32{
        
        return value * 0.453 as f32;
    }
    
    
}

