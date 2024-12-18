curl -X 'POST' \
  'https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Limit/bellande_particle' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
    "node0": [0, 0, 0],
    "node1": [100, 100, 100],
    "environment": [1000, 1000, 1000],
    "size": [10, 10, 10],
    "goal": [200, 200, 200],
    "obstacles": [
      {
        "position": [50, 50, 50],
        "dimensions": [20, 20, 20]
      }
    ],
    "search_radius": 50,
    "sample_points": 20,
    "auth": {
      "authorization_key": "bellande_web_api_opensource"
    }
  }'
echo ""
