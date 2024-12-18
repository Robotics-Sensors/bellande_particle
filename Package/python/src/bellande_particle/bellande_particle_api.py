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

def make_bellande_particle_move_request(particle_state, rotation1, translation, rotation2):
    url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Particle/move"
    
    payload = {
        "particle": {
            "x": particle_state[0],
            "y": particle_state[1],
            "heading": particle_state[2],
            "weight": particle_state[3] if len(particle_state) > 3 else 1.0
        },
        "rotation1": rotation1,
        "translation": translation,
        "rotation2": rotation2,
        "auth": {
            "authorization_key": "bellande_web_api_opensource"
        }
    }
    
    return send_request(url, payload)

def make_bellande_particle_read_markers_request(particle_state, world_info):
    url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Particle/read_markers"
    
    payload = {
        "particle": {
            "x": particle_state[0],
            "y": particle_state[1],
            "heading": particle_state[2],
            "weight": particle_state[3] if len(particle_state) > 3 else 1.0
        },
        "world": world_info,
        "auth": {
            "authorization_key": "bellande_web_api_opensource"
        }
    }
    
    return send_request(url, payload)

def make_bellande_particle_create_random_request(count, world_info):
    url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Particle/create_random"
    
    payload = {
        "count": count,
        "world": world_info,
        "auth": {
            "authorization_key": "bellande_web_api_opensource"
        }
    }
    
    return send_request(url, payload)

def send_request(url, payload):
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
    parser = argparse.ArgumentParser(description="Run Bellande Particle API")
    subparsers = parser.add_subparsers(dest='command', required=True)
    
    # Move command
    move_parser = subparsers.add_parser('move', help='Move particle')
    move_parser.add_argument("--particle-state", required=True,
                            help="Particle state as JSON-formatted list [x, y, heading, weight]")
    move_parser.add_argument("--rotation1", type=float, required=True,
                            help="First rotation angle")
    move_parser.add_argument("--translation", type=float, required=True,
                            help="Translation distance")
    move_parser.add_argument("--rotation2", type=float, required=True,
                            help="Second rotation angle")
    
    # Read markers command
    read_parser = subparsers.add_parser('read-markers', help='Read markers')
    read_parser.add_argument("--particle-state", required=True,
                            help="Particle state as JSON-formatted list [x, y, heading, weight]")
    read_parser.add_argument("--world", required=True,
                            help="World information as JSON object")
    
    # Create random command
    random_parser = subparsers.add_parser('create-random', help='Create random particles')
    random_parser.add_argument("--count", type=int, required=True,
                            help="Number of particles to create")
    random_parser.add_argument("--world", required=True,
                            help="World information as JSON object")
    
    args = parser.parse_args()
    
    try:
        if args.command == 'move':
            particle_state = json.loads(args.particle_state)
            result = make_bellande_particle_move_request(
                particle_state,
                args.rotation1,
                args.translation,
                args.rotation2
            )
        elif args.command == 'read-markers':
            particle_state = json.loads(args.particle_state)
            world_info = json.loads(args.world)
            result = make_bellande_particle_read_markers_request(
                particle_state,
                world_info
            )
        elif args.command == 'create-random':
            world_info = json.loads(args.world)
            result = make_bellande_particle_create_random_request(
                args.count,
                world_info
            )
        
        print(json.dumps(result, indent=2))
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON format in input parameters - {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
