curl -X 'POST' \
  'https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Particle/move' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
    "particle": {
        "x": 0,
        "y": 0,
        "heading": 0,
        "weight": 1.0
    },
    "rotation1": 45.0,
    "translation": 1.0,
    "rotation2": -45.0,
    "auth": {
        "authorization_key": "bellande_web_api_opensource"
    }
  }'

curl -X 'POST' \
  'https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Particle/read_markers' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
    "particle": {
        "x": 0,
        "y": 0,
        "heading": 0,
        "weight": 1.0
    },
    "world": {
        "width": 10.0,
        "height": 10.0,
        "markers": [[1.0, 1.0]]
    },
    "auth": {
        "authorization_key": "bellande_web_api_opensource"
    }
  }'

curl -X 'POST' \
  'https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Particle/create_random' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
    "count": 10,
    "world": {
        "width": 10.0,
        "height": 10.0,
        "markers": [[1.0, 1.0]]
    },
    "auth": {
        "authorization_key": "bellande_web_api_opensource"
    }
  }'

echo ""
