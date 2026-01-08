pub enum Speed {
    Kmh, 
    Ms,
    Mph,
    Knots
}

impl Speed {
    pub fn to_string(&self) -> String {
        match self {
            Speed::Kmh => "km/h".to_string(),
            Speed::Ms => "m/s".to_string(),
            Speed::Mph => "mp/h".to_string(),
            Speed::Knots => "kn".to_string()
        }
    }
}


pub enum Temperature {
    Celsius,
    Fahrenheit
}

impl Temperature {
    pub fn to_string(&self) -> String {
        match self {
            Temperature::Celsius => "°C".to_string(),
            Temperature::Fahrenheit => "°F".to_string()
        }
    }
}

pub enum Precipitation {
    Mm,
    Inch
}

impl Precipitation {
    pub fn to_string(&self) -> String {
        match self {
            Precipitation::Inch => "inch".to_string(),
            Precipitation::Mm => "mm".to_string(),
        }
    }
}