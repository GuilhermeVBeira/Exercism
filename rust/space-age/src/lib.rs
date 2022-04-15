// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const ONE_YEAR_IN_SECONDS: f64 = 31557600.0;
const EARTH_ORBIT: f64 = 1.0;
const MERCURY_ORBIT: f64 = 0.2408467;
const MARS_ORBIT: f64 = 1.8808158;
const JUPITER_ORBIT: f64 = 11.862615;
const SATURN_ORBIT: f64 = 29.447498;
const URANUS_ORBIT: f64 = 84.016846;
const VENUS_ORBIT: f64 = 0.61519726;
const NEPTUNE_ORBIT: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! create_years_during {
    ($planet:ty ,$orbit:expr) => {
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                let earth_years = d.seconds / ONE_YEAR_IN_SECONDS;
                earth_years / $orbit
            }
        }
    };
}

create_years_during!(Mercury, MERCURY_ORBIT);
create_years_during!(Venus, VENUS_ORBIT);
create_years_during!(Earth, EARTH_ORBIT);
create_years_during!(Mars, MARS_ORBIT);
create_years_during!(Jupiter, JUPITER_ORBIT);
create_years_during!(Saturn, SATURN_ORBIT);
create_years_during!(Uranus, URANUS_ORBIT);
create_years_during!(Neptune, NEPTUNE_ORBIT);
