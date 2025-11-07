use fake::{Fake, faker::automotive::raw as f_automotive, locales};

pub fn license_number() -> String {
    f_automotive::LicencePlate(locales::FR_FR).fake()
}
