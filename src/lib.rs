extern crate iata_bcbp;

use arrayvec::ArrayVec;
use iata_bcbp::Bcbp;
use js_sys::Array;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

const MAX_LEGS: usize = 10;

// https://github.com/martinmroz/iata_bcbp/blob/master/src/bcbp/mod.rs

#[wasm_bindgen]
pub struct Leg {
    carrier: String,
    flightnumber: String,
    origin: String,
    destination: String,
    date: String,
}

#[wasm_bindgen]
impl Leg {
    pub fn string(&self) -> String {
        return format!(
            "{} #{} {} -> {} {}",
            self.carrier(),
            self.flightnumber(),
            self.origin(),
            self.destination(),
            self.date()
        );
    }

    pub fn carrier(&self) -> String {
        self.carrier.clone()
    }

    pub fn flightnumber(&self) -> String {
        self.flightnumber.clone()
    }

    pub fn origin(&self) -> String {
        self.origin.clone()
    }

    pub fn destination(&self) -> String {
        self.destination.clone()
    }

    pub fn date(&self) -> String {
        self.date.clone()
    }
}

#[wasm_bindgen]
pub fn parse(barcode: &str) -> Result<Array, JsValue> {
    let rsp = Bcbp::from_str(barcode);

    let _ = match rsp {
        Ok(bcbp_data) => {
            let mut legs = ArrayVec::<[Leg; MAX_LEGS]>::new();
            let mut i: usize = 0;

            for leg in bcbp_data.legs().iter() {
                let l: Leg = Leg {
                    carrier: leg.operating_carrier_designator().trim().to_string(),
                    flightnumber: leg.flight_number().trim().to_string(),
                    origin: leg.from_city_airport_code().trim().to_string(),
                    destination: leg.to_city_airport_code().trim().to_string(),
                    date: leg.date_of_flight().trim().to_string(),
                };

                legs.push(l);

                i = i + 1;

                if i >= MAX_LEGS {
                    break;
                }
            }

            return Ok(legs.into_iter().map(JsValue::from).collect());
        }
        Err(err) => {
            // https://github.com/martinmroz/iata_bcbp/blob/master/src/error.rs
            return Err(format!("Failed to parse barcode, {}", err).into());
        }
    };
}
