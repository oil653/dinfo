use crate::units as Unit;

// Planned methods: fmt::Display for all types
pub struct Precipitation {
    pub combined: u16,
    pub rain: u16,
    pub showers: u16,
    pub snowfall: u16,
    pub unit: Unit::Precipitation
}

// Planned methods: to_string(), to_emoji()
pub struct WeatherCode {
    code: u8
}

// Planned methods: fmt::Display for all the types
pub struct Wind {
    pub speed: u16,
    pub direction: f32,
    pub unit: Unit::Speed
}

// Planned methods: fmt::Display
pub struct Temperature {
    value: f32,
    unit: Unit::Temperature
}

/// The current weather returned by the api
pub struct CurrentWeather {
    /// Current temp
    temperature: Temperature,
    /// Current apparent (feels like) temp
    apparent_temp: Temperature,
    /// Humidity 0-100%
    humidity: u8,
    /// True if it's daytime
    is_day: bool,
    /// Precipitation: combined - rain - showers - snowfall
    precipitation: Precipitation,
    /// Weather code, 0-100
    weather_code: WeatherCode,
    /// Cloud cover 0-100%
    cloud_clover: u8,
    /// Wind speed / direction with units
    wind: Wind
}