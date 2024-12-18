// Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use reqwest;
use serde_json::{json, Value};
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bellande_particle", about = "Bellande Particles Tool")]
struct Opt {
    #[structopt(
        long,
        help = "Particle state as JSON-formatted list [x, y, heading, weight]"
    )]
    particle: String,

    #[structopt(
        long,
        help = "Movement parameters as JSON-formatted list [rotation1, translation, rotation2]"
    )]
    movement: Option<String>,

    #[structopt(
        long,
        help = "World information as JSON object with width, height, and markers"
    )]
    world: Option<String>,

    #[structopt(long, help = "Particle count for random generation")]
    count: Option<i32>,
}

async fn make_bellande_particles_request(
    particle: Value,
    movement: Option<Value>,
    world: Option<Value>,
    count: Option<i32>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Particle/bellande_particle";

    let payload = json!({
        "particle": particle,
        "movement": movement,
        "world": world,
        "count": count,
        "auth": {
            "authorization_key": "bellande_web_api_opensource"
        }
    });

    let response = client
        .post(url)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    // Parse JSON strings to Values for validation
    let particle: Value = serde_json::from_str(&opt.particle)
        .map_err(|e| format!("Error parsing particle: {}", e))?;

    // Parse optional movement
    let movement: Option<Value> = match opt.movement {
        Some(ref m) => {
            Some(serde_json::from_str(m).map_err(|e| format!("Error parsing movement: {}", e))?)
        }
        None => None,
    };

    // Parse optional world
    let world: Option<Value> = match opt.world {
        Some(ref w) => {
            Some(serde_json::from_str(w).map_err(|e| format!("Error parsing world: {}", e))?)
        }
        None => None,
    };

    // Run using API
    match make_bellande_particles_request(particle, movement, world, opt.count).await {
        Ok(result) => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
