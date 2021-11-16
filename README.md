# okapi

Okapi is the image generation API for [IdleRPG](https://git.travitia.xyz/Kenvyra/IdleRPG). It uses [actix-web](https://github.com/actix/actix-web) and [image-rs](https://github.com/image-rs/image) under the hood.

## Running it

There is a [podman image](https://quay.io/repository/gelbpunkt/okapi) for okapi.

There are several environment variables to configure it:

- `PORT` sets the port to listen on. Defaults to 3000
- `PROXY_URL` sets the URL for a custom proxy we use internally. Can be ignored.
- `PROXY_AUTH` sets the auth key for the proxy. Can be ignored.

## Routes

### Index

`GET /`

This is a route to validate the server is up. The reply will always be `1` with a 200 status code.

### Adventures

`POST /api/genadventures`

**JSON Body format:**

`{"percentages": [[int, int], ...]}`

This route expects 30 2-element arrays with two percentages.

It will draw 30 images for each adventure in IdleRPG and return them as an array of base64 encoded PNG data.

### Chess

`POST /api/genchess`

**JSON Body format:**

`{"xml": str}`

This route expects SVG data as the XML value.

It will render it on a 390x390 canvas and return a PNG image.

### ImageOps

`POST /api/imageops/pixel`

**JSON Body format:**

`{"image": str}`

This route expects the URL to a low-resolution image.

It will resize the image to a 1024x1024 canvas and return a PNG image.

`POST /api/imageops/invert`

**JSON Body format:**

`{"image": str}`

This route expects the URL to an image.

It will invert the image and return a PNG image.

`POST /api/imageops/edges`

**JSON Body format:**

`{"image": str}`

This route expects the URL to an image.

It will grayscale the image and apply a canny algorithm over it to detect edges and return a PNG image.

`POST /api/imageops/oil`

**JSON Body format:**

`{"image": str}`

This route expects the URL to an image.

It will apply an oil-painting effect on the image and return a PNG image.

### Overlay

`POST /api/genoverlay`

**JSON Body format:**

`{"url": str}`

This route expects the URl to an image.

It will resize the image to 800x650 and overlay a transparent PNG over it to create profile images for IdleRPG. The result is returned as a PNG image.

### Profile

`POST /api/genprofile`

**JSON Body format:**

```
{
    "name": str,
    "image": str,
    "race": str,
    "color": [int, int, int, float],
    "classes": [str, str],
    "damage": str,
    "defense": str,
    "sword_name": str,
    "shield_name": str,
    "level": str,
    "money": str,
    "god": str,
    "guild": str,
    "marriage": str,
    "pvp_wins": str,
    "adventure": str,
    "icons": [str, str]
}
```

This route expects IdleRPG profile data.

It will download the image parameter if it is not "0", else uses a default background. The parameters are drawn as images and text over the background and the result is returned as a PNG image.

### Error Handling

Any error returned by the API has a HTTP status code representing the cause.

They will all use JSON as the content type and have a body like this:

`{"status": "error", "reason": "a short reason", "detail": "the exact cause"}`
