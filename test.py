import requests

req = requests.post("http://localhost:3000/api/imageops/oil", json={"image": "https://cdn.discordapp.com/avatars/356091260429402122/cb01d86552cc783532cabdb7ad1c70bd.png?size=1024"})
open("test.png", "wb").write(req.content)
