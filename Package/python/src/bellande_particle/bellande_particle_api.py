# Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

#!/usr/bin/env python3
import requests
import argparse
import json
import sys

def make_bellande_particles_request(particle, movement=None, world=None, count=None):
    url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Particles/bellande_particles"
    
    # Convert string inputs to lists/dicts if they're strings
    if isinstance(particle, str):
        particle = json.loads(particle)
    if isinstance(movement, str):
        movement = json.loads(movement)
    if isinstance(world, str):
        world = json.loads(world)
        
    payload = {
        "particle": particle,
        "movement": movement,
        "world": world,
        "count": count,
        "auth": {
            "authorization_key": "bellande_web_api_opensource"
        }
    }
    
    headers = {
        'accept': 'application/json',
        'Content-Type': 'application/json'
    }
    
    try:
        response = requests.post(url, json=payload, headers=headers)
        response.raise_for_status()
        return response.json()
    except requests.RequestException as e:
        print(f"Error making request: {e}", file=sys.stderr)
        sys.exit(1)

def main():
    parser = argparse.ArgumentParser(description="Run Bellande Particles API")
    parser.add_argument("--particle", required=True,
                       help="Particle state as JSON-formatted list [x, y, heading, weight]")
    parser.add_argument("--movement",
                       help="Movement parameters as JSON-formatted list [rotation1, translation, rotation2]")
    parser.add_argument("--world",
                       help="world information as JSON object with width, height, and markers")
    parser.add_argument("--count", type=int,
                       help="Particle count for random generation")
    
    args = parser.parse_args()
    
    try:
        result = make_bellande_particles_request(
            args.particle,
            args.movement,
            args.world,
            args.count
        )
        
        print(json.dumps(result, indent=2))
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON format in input parameters - {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
