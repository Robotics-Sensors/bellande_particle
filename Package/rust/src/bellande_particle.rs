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
enum Command {
    #[structopt(name = "move", about = "Move particle")]
    Move {
        #[structopt(
            long,
            help = "Particle state as JSON-formatted list [x, y, heading, weight]"
        )]
        particle_state: String,

        #[structopt(long, help = "First rotation angle")]
        rotation1: f64,

        #[structopt(long, help = "Translation distance")]
        translation: f64,

        #[structopt(long, help = "Second rotation angle")]
        rotation2: f64,
    },

    #[structopt(name = "read-markers", about = "Read markers")]
    ReadMarkers {
        #[structopt(
            long,
            help = "Particle state as JSON-formatted list [x, y, heading, weight]"
        )]
        particle_state: String,

        #[structopt(long, help = "World information as JSON object")]
        world: String,
    },

    #[structopt(name = "create-random", about = "Create random particles")]
    CreateRandom {
        #[structopt(long, help = "Number of particles to create")]
        count: i32,

        #[structopt(long, help = "World information as JSON object")]
        world: String,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "bellande_particle", about = "Bellande Particle Tool")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

async fn make_bellande_particle_move_request(
    particle_state: Value,
    rotation1: f64,
    translation: f64,
    rotation2: f64,
) -> Result<Value, Box<dyn Error>> {
    let url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Particle/move";

    let payload = json!({
        "particle": {
            "x": particle_state[0],
            "y": particle_state[1],
            "heading": particle_state[2],
            "weight": particle_state.get(3).unwrap_or(&json!(1.0))
        },
        "rotation1": rotation1,
        "translation": translation,
        "rotation2": rotation2,
        "auth": {
            "authorization_key": "bellande_web_api_opensource"
        }
    });

    send_request(url, payload).await
}

async fn make_bellande_particle_read_markers_request(
    particle_state: Value,
    world: Value,
) -> Result<Value, Box<dyn Error>> {
    let url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Particle/read_markers";

    let payload = json!({
        "particle": {
            "x": particle_state[0],
            "y": particle_state[1],
            "heading": particle_state[2],
            "weight": particle_state.get(3).unwrap_or(&json!(1.0))
        },
        "world": world,
        "auth": {
            "authorization_key": "bellande_web_api_opensource"
        }
    });

    send_request(url, payload).await
}

async fn make_bellande_particle_create_random_request(
    count: i32,
    world: Value,
) -> Result<Value, Box<dyn Error>> {
    let url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Particle/create_random";

    let payload = json!({
        "count": count,
        "world": world,
        "auth": {
            "authorization_key": "bellande_web_api_opensource"
        }
    });

    send_request(url, payload).await
}

async fn send_request(url: &str, payload: Value) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();

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

    let result = match opt.cmd {
        Command::Move {
            particle_state,
            rotation1,
            translation,
            rotation2,
        } => {
            let particle_state: Value = serde_json::from_str(&particle_state)
                .map_err(|e| format!("Error parsing particle state: {}", e))?;

            make_bellande_particle_move_request(particle_state, rotation1, translation, rotation2)
                .await?
        }
        Command::ReadMarkers {
            particle_state,
            world,
        } => {
            let particle_state: Value = serde_json::from_str(&particle_state)
                .map_err(|e| format!("Error parsing particle state: {}", e))?;
            let world: Value = serde_json::from_str(&world)
                .map_err(|e| format!("Error parsing world info: {}", e))?;

            make_bellande_particle_read_markers_request(particle_state, world).await?
        }
        Command::CreateRandom { count, world } => {
            let world: Value = serde_json::from_str(&world)
                .map_err(|e| format!("Error parsing world info: {}", e))?;

            make_bellande_particle_create_random_request(count, world).await?
        }
    };

    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
