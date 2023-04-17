extern crate iata_bcbp;

use iata_bcbp::Bcbp;
use wasm_bindgen::prelude::*;
use std::str::FromStr;
use arrayvec::ArrayVec;
use js_sys::Array;

const MAX_LEGS: usize = 10;
    
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
pub fn parse(barcode: &str) -> Array {

    let rsp = Bcbp::from_str(barcode).unwrap();

    let mut legs = ArrayVec::<[Leg; MAX_LEGS]>::new();


    let mut i: usize = 0;
    
    for leg in rsp.legs().iter() {

    	let l: Leg = Leg{
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

    legs.into_iter().map(JsValue::from).collect()
}
