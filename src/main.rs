#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;

use std::ops::Add;

use clap::{Arg, App};

use std::io::{Write, Cursor};
use futures::{Future, Stream};

use hyper::Client;
use tokio_core::reactor::Core;

use hyper_tls::HttpsConnector;

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    name: String, 
    address: String, 
    age: String, 
    armed: String, 
    cause: String, 
    city: String, 
    state: String, 
    day: String, 
    month: String, 
    year: String, 
    race: String, 
    sex: String
}

fn main() {
    let matches = App::new("The Counted")
                    .version(crate_version!())
                    .about("Query The Guardian's API for police killings in the United States.")
                    .arg(Arg::with_name("age")
                         .short("a")
                         .long("age")
                         .value_name("AGE")
                         .help("Search for victims of the given age."))
                    .arg(Arg::with_name("sex")
                         .short("s")
                         .long("sex")
                         .value_name("SEX")
                         .help("Search for victims of the given sex (Male or Female).")
                         .possible_values(&["Male", "Female", "Other", "Unknown"]))
                    .arg(Arg::with_name("city")
                         .short("c")
                         .long("city")
                         .value_name("CITY")
                         .help("Search for incidents in the given city, like `-c \"San Diego\"`."))
                    .arg(Arg::with_name("state")
                         .short("t")
                         .long("state")
                         .value_name("STATE CODE")
                         .help("Search for incidents in the given state, like `-s CA` for California."))
                    .arg(Arg::with_name("armed")
                         .short("m")
                         .long("armed")
                         .value_name("TYPE")
                         .help("Search for incidents in which the victim was armed, based on a description like \"Unarmed\", \"Disputed\", \"Firearm\", et cetera."))
                    .arg(Arg::with_name("race")
                         .short("r")
                         .long("race")
                         .value_name("RACE")
                         .help("Search for incidents in which the victim was of the given race, using US Census races. For instance, `-r Black`.")
                         .possible_values(&["White", "Black", "Arab-American", "Hispanic/Latino", "Asian/Pacific Islander", "Native American", "Unknown", "Other"]))
                    .arg(Arg::with_name("csv")
                         .short("v")
                         .long("csv")
                         .help("Format the output as CSV, for use in spreadsheet and statistical software. Defaults to off."))
                    .arg(Arg::with_name("all")
                         .long("all")
                         .help("Fetch all incidents.")
                         .conflicts_with_all(&["Race", "Armed", "State", "City", "Sex", "Age"]))
                  .get_matches();
    
    let mut uri_string: String = "https://thecountedapi.com/api/counted".into();
    let uri: hyper::Uri;

    let all = matches.is_present("all");
    
    if all {
        uri = uri_string.parse().unwrap_or_else(|e| {panic!("failed to parse builtin uri string for all matches: {}", e)});
    } else {
        // Not fetching all matches.
        uri_string = uri_string.add("?");
        // Go through all possible qualifiers, adding them into the query string
        if let Ok(age) = value_t!(matches, "age", u32) {
            uri_string = uri_string.add(&format!("age={}&", age));
        }

        if let Some(sex) = matches.value_of("sex") {
            uri_string = uri_string.add(&format!("sex={}&", sex));
        }

        if let Some(city) = matches.value_of("city") {
            uri_string = uri_string.add(&format!("city={}&", city));
        }

        if let Some(state) = matches.value_of("state") {
            uri_string = uri_string.add(&format!("state={}&", state));
        }

        if let Some(armed) = matches.value_of("armed") {
            uri_string = uri_string.add(&format!("armed={}&", armed));
        }

        if let Some(race) = matches.value_of("race") {
            uri_string = uri_string.add(&format!("race={}&", race));
        }
        // HACK
        let mut encoded_uri_string = uri_string.replace(" ", "%20");
        let last_position = encoded_uri_string.len() - 1;
        if encoded_uri_string.chars().nth(last_position).unwrap() == '&' {
            encoded_uri_string.remove(last_position); // Remove trailing &
        }
        uri = encoded_uri_string.parse().unwrap_or_else(|e| {panic!("failed to parse builtin uri string for limited matches: {}", e)});
    }

    // Spin up the Tokio event loop
    let mut core = Core::new().unwrap_or_else(|e| {panic!("failed to start Tokio event loop: {}", e)});

    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap_or_else(|e| panic!(e)))
        .build(&core.handle());



    let mut buffer = Vec::<u8>::new();

    // Scope for buffer -> Tokio loop borrow
    {

        // Set up the Get request
        let work = client.get(uri).and_then(|response| {        
            response.body().for_each(|chunk| {
                buffer.write_all(&chunk)
                    .map(|_| {})
                    .map_err(From::from)
            })
        });

        core.run(work).unwrap();
    }


    let records: Vec<Record> = serde_json::from_reader(Cursor::new(&buffer)).unwrap();

    for record in records {
        println!("{}, {}yo {} {} in {}, {}", record.name, record.age, record.race, record.sex, record.city, record.state)
    }

}
