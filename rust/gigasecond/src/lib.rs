use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
const GIGA_SECOND: time::Duration = Duration::seconds(1_000_000_000);

pub fn after(start: DateTime) -> DateTime {
    start + GIGA_SECOND
}
