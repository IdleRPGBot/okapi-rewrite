import requests

req = requests.post("http://localhost:3000/api/genoverlay", json={"url": "https://i.imgur.com/WS4xcW3.jpeg"})
open("test.png", "wb").write(req.content)
