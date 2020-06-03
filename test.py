import requests

req = requests.post("http://localhost:3000/api/genadventures", json={"percentages": [[1, 2]]*30})
print(req.json())
