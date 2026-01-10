use crate::units as Unit;
pub use crate::units::Units;

use public_ip_address;
use open_meteo_rs::{self, Client, forecast::Options};
use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub struct Precipitation {
    combined: f32,
    rain: f32,
    showers: f32,
    snowfall: f32,
    unit: Unit::Precipitation
}

impl Precipitation {
    pub fn new(combined: f32, rain: f32, showers: f32, snowfall: f32, unit: Unit::Precipitation) -> Precipitation {
        Precipitation { combined, rain, showers, snowfall, unit }
    }

    pub fn combined_to_string(&self) -> String {
        format!("{}{}", self.combined, self.unit.to_string())
    }
    
    pub fn rain_to_string(&self) -> String {
        format!("{}{}", self.rain, self.unit.to_string())
    }
    
    pub fn showers_to_string(&self) -> String {
        format!("{}{}", self.showers, self.unit.to_string())
    }
    
    pub fn snowfall_to_string(&self) -> String {
        format!("{}{}", self.snowfall, self.unit.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct Wind {
    speed: f32,
    direction: f32,
    unit: Unit::Speed
}

impl Wind {
    pub fn new(speed: f32, direction: f32, unit: Unit::Speed) -> Self {
        Self { speed, direction, unit }
    }

    pub fn speed_stringify(&self) -> String {
        format!("{}{}", self.speed, self.unit.stringify())
    }

    pub fn direction_stringify(&self) -> String {
        let normalized = self.direction % 360.0;
        let normalized = if normalized < 0.0 { normalized + 360.0 } else { normalized };
        
        match normalized {
            d if d >= 337.5 || d < 22.5 => "N".to_string(),
            d if d < 67.5 => "NE".to_string(),
            d if d < 112.5 => "E".to_string(),
            d if d < 157.5 => "SE".to_string(),
            d if d < 202.5 => "S".to_string(),
            d if d < 247.5 => "SW".to_string(),
            d if d < 292.5 => "W".to_string(),
            _ => "NW".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Temperature {
    value: f64,
    unit: Unit::Temperature
}

impl Temperature {
    pub fn new(value: f64, unit: Unit::Temperature) -> Self {
        Self {value, unit}
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit.to_string())
    }
}

/// The current weather returned by the api
pub struct CurrentWeather {
    /// Current temp
    pub temperature: Temperature,
    /// Current apparent (feels like) temp
    pub apparent_temp: Temperature,
    /// Humidity 0-100%
    pub humidity: u8,
    /// True if it's daytime
    pub is_day: bool,
    /// Precipitation: combined - rain - showers - snowfall
    pub precipitation: Precipitation,
    /// Weather code, 0-100
    pub weather_code: WeatherCode,
    /// Cloud cover 0-100%
    pub cloud_cover: u8,
    /// Wind speed / direction with units
    pub wind: Wind
}

impl CurrentWeather {
    pub fn new (
        temperature: Temperature, 
        apparent_temp: Temperature, 
        humidity: u8, 
        is_day: bool, 
        precipitation: Precipitation, 
        weather_code: WeatherCode, 
        cloud_cover: u8, 
        wind: Wind
    ) -> Self {
        CurrentWeather { temperature, apparent_temp, humidity, is_day, precipitation, weather_code, cloud_cover, wind }
    }

    /// Creates an example struct with all the values filled in. 
    /// Intended for testing and developing purposes
    #[allow(dead_code)]
    pub fn new_example() -> Self {
        Self::new(
            Temperature::new(32.0, Unit::Temperature::Celsius),
            Temperature::new(35.0, Unit::Temperature::Celsius), 
            68,
            false,
            Precipitation::new(0.15, 0.12, 0.3, 0.0, Unit::Precipitation::Mm),
            WeatherCode::from_code(2).expect("Invalid WMO code provided"),
            80,
            Wind::new(40.0, 16.0, Unit::Speed::Kmh)
        )
        
    }

    /// Creates an example struct with all the values filled in, the weather code can be passed. 
    /// Intended for testing and developing purposes
    #[allow(dead_code)]
    pub fn new_example_with_code(code: usize) -> Self {
        Self::new(
            Temperature::new(32.0, Unit::Temperature::Celsius),
            Temperature::new(35.0, Unit::Temperature::Celsius), 
            68,
            false,
            Precipitation::new(0.15, 0.12, 0.3, 0.0, Unit::Precipitation::Mm),
            WeatherCode::from_code(code).expect("Invalid WMO code provided"),
            80,
            Wind::new(40.0, 16.0, Unit::Speed::Kmh)
        )
        
    }
    /// Creates an example struct with all the values filled in, the weather code can be passed.
    /// This also uses fahrenheit.
    /// Intended for testing and developing purposes
    pub fn new_example_with_code_fahrenheit(code: usize) -> Self {
        Self::new(
            Temperature::new(105.0, Unit::Temperature::Celsius),
            Temperature::new(105.0, Unit::Temperature::Fahrenheit), 
            68,
            false,
            Precipitation::new(0.15, 0.12, 0.3, 0.0, Unit::Precipitation::Mm),
            WeatherCode::from_code(code).expect("Invalid WMO code provided"),
            80,
            Wind::new(40.0, 16.0, Unit::Speed::Kmh)
        )
        
    }
}

/// Cloud cover over an area
pub enum CloudCover {
    MainlyClear,
    Partial,
    Overcast
}

/// Basic intensity of a weather event
pub enum Intensity {
    Light,
    Moderate, 
    Heavy
}

/// Intensity for weather events with 2 states
pub enum SimpleIntensity {
    Light,
    Heavy
}

pub enum WeatherCode {
    Clear,
    Cloudy(CloudCover),
    Fog{is_rime_fog: bool},
    Drizzle(Intensity),
    FreezingDrizzle(SimpleIntensity),
    Rain(Intensity),
    FreezingRain(SimpleIntensity),
    SnowFall(Intensity),
    SnowGrains,
    RainShowers(Intensity),
    SnowShowers(SimpleIntensity),
    Thunderstorm,
    ThunderstormWithHail(SimpleIntensity)
}

impl WeatherCode {
    /// Constructs a WeatherCode instance from a weather code, returns none if the weather code isnt supported
    /// List of supported weather code: 
    ///    <table>
    ///  <thead>
    ///    <tr>
    ///      <th>Code</th>
    ///      <th>Description</th>
    ///    </tr>
    ///  </thead>
    ///  <tbody>
    ///    <tr><td>0</td><td>Clear sky</td></tr>
    ///    <tr><td>1, 2, 3</td><td>Mainly clear, partly cloudy, and overcast</td></tr>
    ///    <tr><td>45, 48</td><td>Fog and depositing rime fog</td></tr>
    ///    <tr><td>51, 53, 55</td><td>Drizzle: Light, moderate, and dense intensity</td></tr>
    ///    <tr><td>56, 57</td><td>Freezing drizzle: Light and dense intensity</td></tr>
    ///    <tr><td>61, 63, 65</td><td>Rain: Slight, moderate, and heavy intensity</td></tr>
    ///    <tr><td>66, 67</td><td>Freezing rain: Light and heavy intensity</td></tr>
    ///    <tr><td>71, 73, 75</td><td>Snow fall: Slight, moderate, and heavy intensity</td></tr>
    ///    <tr><td>77</td><td>Snow grains</td></tr>
    ///    <tr><td>80, 81, 82</td><td>Rain showers: Slight, moderate, and violent</td></tr>
    ///    <tr><td>85, 86</td><td>Snow showers: Slight and heavy</td></tr>
    ///    <tr><td>95 </td><td>Thunderstorm: Slight or moderate</td></tr>
    ///    <tr><td>96, 99</td><td>Thunderstorm with slight and heavy hail</td></tr>
    ///  </tbody>
    /// </table>
    /// source: https://open-meteo.com/en/docs?hourly=&current=weather_code#weather_variable_documentation
    fn from_code(code: usize) -> Option<Self> {
        match code {
            0 => Some(Self::Clear),
            1 => Some(Self::Cloudy(CloudCover::MainlyClear)),
            2 => Some(Self::Cloudy(CloudCover::Partial)),
            3 => Some(Self::Cloudy(CloudCover::Overcast)),
            45 => Some(Self::Fog { is_rime_fog: false }),
            48 => Some(Self::Fog { is_rime_fog: true }),
            51 => Some(Self::Drizzle(Intensity::Light)),
            53 => Some(Self::Drizzle(Intensity::Moderate)),
            55 => Some(Self::Drizzle(Intensity::Heavy)),
            56 => Some(Self::FreezingDrizzle(SimpleIntensity::Light)),
            57 => Some(Self::FreezingDrizzle(SimpleIntensity::Heavy)),
            61 => Some(Self::Rain(Intensity::Light)),
            63 => Some(Self::Rain(Intensity::Moderate)),
            65 => Some(Self::Rain(Intensity::Heavy)),
            66 => Some(Self::FreezingRain(SimpleIntensity::Light)),
            67 => Some(Self::FreezingRain(SimpleIntensity::Heavy)),
            71 => Some(Self::SnowFall(Intensity::Light)),
            73 => Some(Self::SnowFall(Intensity::Moderate)),
            75 => Some(Self::SnowFall(Intensity::Heavy)),
            77 => Some(Self::SnowGrains),
            80 => Some(Self::RainShowers(Intensity::Light)),
            81 => Some(Self::RainShowers(Intensity::Moderate)),
            82 => Some(Self::RainShowers(Intensity::Heavy)),
            85 => Some(Self::SnowShowers(SimpleIntensity::Light)),
            86 => Some(Self::SnowShowers(SimpleIntensity::Heavy)),
            95 => Some(Self::Thunderstorm),
            96 => Some(Self::ThunderstormWithHail(SimpleIntensity::Light)),
            99 => Some(Self::ThunderstormWithHail(SimpleIntensity::Heavy)),
            _ => None
        }
    }

    /// Converts a weather code back to a human readable string
    pub fn to_string(&self) -> String {
        match self {
            Self::Clear => "Clear sky".to_string(),

            Self::Cloudy(cloud_cover) => match cloud_cover {
                CloudCover::MainlyClear => "Mainly clear".to_string(),
                CloudCover::Partial => "Partly cloudy".to_string(),
                CloudCover::Overcast => "Overcast".to_string(),
            },

            Self::Fog { is_rime_fog } => {
                if *is_rime_fog {
                    "Rime fog".to_string()
                } else {
                    "Fog".to_string()
                }
            }

            Self::Drizzle(intensity) => match intensity {
                Intensity::Light => "Light drizzle".to_string(),
                Intensity::Moderate => "Moderate drizzle".to_string(),
                Intensity::Heavy => "Dense drizzle".to_string(),
            },

            Self::FreezingDrizzle(intensity) => match intensity {
                SimpleIntensity::Light => "Light freezing drizzle".to_string(),
                SimpleIntensity::Heavy => "Dense freezing drizzle".to_string(),
            },

            Self::Rain(intensity) => match intensity {
                Intensity::Light => "Light rain".to_string(),
                Intensity::Moderate => "Moderate rain".to_string(),
                Intensity::Heavy => "Heavy rain".to_string(),
            },

            Self::FreezingRain(intensity) => match intensity {
                SimpleIntensity::Light => "Light freezing rain".to_string(),
                SimpleIntensity::Heavy => "Heavy freezing rain".to_string(),
            },

            Self::SnowFall(intensity) => match intensity {
                Intensity::Light => "Light snowfall".to_string(),
                Intensity::Moderate => "Moderate snowfall".to_string(),
                Intensity::Heavy => "Heavy snowfall".to_string(),
            },

            Self::SnowGrains => "Snow grains".to_string(),

            Self::RainShowers(intensity) => match intensity {
                Intensity::Light => "Light rain showers".to_string(),
                Intensity::Moderate => "Moderate rain showers".to_string(),
                Intensity::Heavy => "Violent rain showers".to_string(),
            },

            Self::SnowShowers(intensity) => match intensity {
                SimpleIntensity::Light => "Light snow showers".to_string(),
                SimpleIntensity::Heavy => "Heavy snow showers".to_string(),
            },

            Self::Thunderstorm => "Thunderstorm".to_string(),

            Self::ThunderstormWithHail(intensity) => match intensity {
                SimpleIntensity::Light => "Thunderstorm with slight hail".to_string(),
                SimpleIntensity::Heavy => "Thunderstorm with heavy hail".to_string(),
            },
        }
    }
    /// Converts a weather code back to a string containing a utf emoji representing the weather condition
    pub fn to_emoji(&self, is_night: bool) -> String {
        match self {
            Self::Clear => {
                if is_night {
                    "☀️".to_string()
                } else {
                    "🌙".to_string()
                }
            },

            Self::Cloudy(cloud_cover) => match cloud_cover {
                CloudCover::MainlyClear => "🌤️".to_string(),
                _ => "🌥️".to_string()
            },

            Self::Fog {is_rime_fog: _} => "🌫️".to_string(),

            Self::Drizzle(_) => "🌦️".to_string(),

            Self::Rain(_) | Self::FreezingRain(_) | Self::RainShowers(_) => "🌧️".to_string(),

            Self::SnowFall(_) | Self::SnowShowers(_) | Self::FreezingDrizzle(_) => "️🌨️".to_string(),

            Self::SnowGrains => "❄️".to_string(),

            Self::Thunderstorm | Self::ThunderstormWithHail(_) => "⛈️".to_string(),
        }
    }
}

// =========================================
#[derive(Debug)]
pub struct Cordinates {
    pub lat: f64,
    pub lng: f64
}

impl Cordinates {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
}

pub async fn get_cordinates() -> Result<Cordinates, public_ip_address::error::Error> {
    let res= match public_ip_address::perform_lookup(None).await {
        Ok(r) => r,
        Err(e) => return Err(e)
    };

    let err_msg = "Location api returned, but no cordinates were present";
    
    let lng = res.longitude.expect(err_msg);
    let lat = res.latitude.expect(err_msg);

    Ok(Cordinates::new(lat, lng))
}

#[allow(dead_code)]
pub async fn get_city() -> Option<String> {
    let res= match public_ip_address::perform_lookup(None).await {
        Ok(r) => r,
        Err(_) => return None
    };
    let city = res.city?;
    Some(city)
}

pub async fn get_timezone() -> Option<String> {
    let res= match public_ip_address::perform_lookup(None).await {
        Ok(r) => r,
        Err(_) => return None
    };
    let tz = res.time_zone?;
    println!("Got timzone: {}", tz);
    Some(tz)
}

pub async fn get_current_weather(units: &Units) -> Result<CurrentWeather, Box<dyn Error>> {
    let (client, mut opts) = match weather_setup(&units).await {
        Ok((client, opts)) => (client, opts),
        Err(e) => return Err(e)
    };

    let mut current_parameters: Vec<String> = vec![
        "temperature_2m", 
        "relative_humidity_2m", 
        "apparent_temperature", 
        "is_day", 
        "precipitation", 
        "rain", 
        "showers", 
        "snowfall", 
        "weather_code", 
        "cloud_cover", 
        "wind_speed_10m", 
        "wind_direction_10m"
    ].iter().map(|d| {d.to_string()}).collect();

    opts.current.append(&mut current_parameters);
    
    
    let res = client
    .forecast(opts)
    .await?
    .current
    .expect("Weather API returned current weather forecast, but it's empty (None)");
    
    // println!("{:#?}", res);

    let res = res.values;

    let temp = Temperature::new(
        res.get("temperature_2m")
                .expect("Missing temperature_2m from api response")
                .value
                .as_f64()
                .expect("Failed to parse json value"), 
                units.temperature.clone()
    );

    let app_temp = Temperature::new(
        res.get("apparent_temperature")
                .expect("Missing apparent_temperature from api response")
                .value
                .as_f64()
                .expect("Failed to parse json value"), 
                units.temperature.clone()
    );

    let humidity = {
        res.get("relative_humidity_2m")
        .expect("Missing relative_humidity_2m from api response")
        .value
        .as_u64()
        .expect("Failed to parse json value")
    };
    
    let is_day = {
        res.get("is_day")
        .expect("Missing is_day from api response")
        .value
        .as_i64()
        .expect("Failed to parse json value") 
        == 0
    };

    let prec = {
        Precipitation::new(
            res.get("precipitation")
                .expect("Missing precipitation from api response")
                .value
                .as_f64()
                .expect("Failed to parse json value") as f32, 

                res.get("rain")
                .expect("Missing showers from api response")
                .value
                .as_f64()
                .expect("Failed to parse json value") as f32,

                res.get("showers")
                .expect("Missing showers from api response")
                .value
                .as_f64()
                .expect("Failed to parse json value") as f32,

                res.get("snowfall")
                .expect("Missing snowfall from api response")
                .value
                .as_f64()
                .expect("Failed to parse json value") as f32,

                units.precipitation.clone()
        )
    };
    
    let weather_code = {
        WeatherCode::from_code(res.get("weather_code")
            .expect("Missing weather_code from api response")
            .value
            .as_u64()
            .expect("Failed to parse json value") as usize
        ).expect("Invalid weather code recived from weather API")
    };

    let cloud_cover = {
        res.get("cloud_cover")
        .expect("Missing cloud_cover from api response")
        .value
        .as_u64()
        .expect("Failed to parse json value") as u8
    };

    let wind = {
        Wind::new(
            res.get("wind_speed_10m")
                .expect("Missing wind_speed_10m from api response")
                .value
                .as_f64()
                .expect("Failed to parse json value") as f32, 

                res.get("rain")
                .expect("Missing showers from api response")
                .value
                .as_f64()
                .expect("Failed to parse json value") as f32,

                units.speed.clone()
        )
    };

    let current_weather = CurrentWeather::new(
        temp, 
        app_temp, 
        humidity as u8, 
        is_day, 
        prec, 
        weather_code, 
        cloud_cover, 
        wind
    );

    Ok(current_weather)
}
/// Saves some boilerplate by setting up units, location and timezone
async fn weather_setup(units: &Units) -> Result<(Client, Options), Box<dyn Error>> {
    let client = open_meteo_rs::Client::new();
    let mut opts = open_meteo_rs::forecast::Options::default();

    let loc = match get_cordinates().await {
        Ok(d) => d,
        Err(e) => return Err(Box::new(e))
    };

    opts.location = open_meteo_rs::Location { lat: loc.lat, lng: loc.lng };

    opts.temperature_unit = Some(match units.temperature {
        Unit::Temperature::Celsius => open_meteo_rs::forecast::TemperatureUnit::Celsius,
        Unit::Temperature::Fahrenheit => open_meteo_rs::forecast::TemperatureUnit::Fahrenheit
    });

    opts.wind_speed_unit = Some(match units.speed {
        Unit::Speed::Kmh => open_meteo_rs::forecast::WindSpeedUnit::Kmh,
        Unit::Speed::Knots => open_meteo_rs::forecast::WindSpeedUnit::Kn,
        Unit::Speed::Ms => open_meteo_rs::forecast::WindSpeedUnit::Ms,
        Unit::Speed::Mph => open_meteo_rs::forecast::WindSpeedUnit::Mph
    });

    opts.precipitation_unit = Some(match units.precipitation {
        Unit::Precipitation::Inch => open_meteo_rs::forecast::PrecipitationUnit::Inches,
        Unit::Precipitation::Mm => open_meteo_rs::forecast::PrecipitationUnit::Millimeters
    });

    opts.time_zone = get_timezone().await;

    opts.cell_selection = Some(open_meteo_rs::forecast::CellSelection::Nearest);

    Ok((client, opts))
}