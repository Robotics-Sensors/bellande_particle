curl -X 'POST' \
  'https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Particle/bellande_particle' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
    "particle": [0, 0, 0, 1.0],
    "movement": {
        "rotation1": 45.0,
        "translation": 1.0,
        "rotation2": -45.0
    },
    "world": {
        "width": 10.0,
        "height": 10.0,
        "markers": [[1.0, 1.0]]
    },
    "count": 10,
    "auth": {
        "authorization_key": "bellande_web_api_opensource"
    }
  }'
echo ""
