#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Speed {
    Kmh, 
    Ms,
    Mph,
    Knots
}

#[allow(dead_code)]
impl Speed {
    pub fn stringify(&self) -> String {
        match self {
            Speed::Kmh => "km/h".to_string(),
            Speed::Ms => "m/s".to_string(),
            Speed::Mph => "mp/h".to_string(),
            Speed::Knots => "kn".to_string()
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Speed::Kmh => "kmh".to_string(),
            Speed::Ms => "ms".to_string(),
            Speed::Mph => "mph".to_string(),
            Speed::Knots => "kn".to_string()
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
#[allow(dead_code)]
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

#[derive(Clone, Debug)]
pub struct Units {
    pub speed: Speed,
    pub temperature: Temperature,
    pub precipitation: Precipitation
}

impl Units {
    pub fn new(speed: Speed, temperature: Temperature, precipitation: Precipitation) -> Self {
        Units { speed, temperature, precipitation }
    }
}