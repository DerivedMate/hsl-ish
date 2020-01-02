# HSL-ish
A dead simple, accurate enough (see the tests) implementation of the [HSL](https://en.wikipedia.org/wiki/HSL_and_HSV) color space in rust.

## Motivation
I couldn't find the right implementation of the color space for one of my visualization project (none of the known to me implementations at the time implemented implemented the HSL-RGB conversion), so I decided to create my own, tiny and dead simple implementation.

## Caveats
The HSL-RGB conversion (`Rgb::from(hsl: Hsl)`) currently appears to give results that are one-off from the expected ones under certain conditions (see: [issue](#)).

## Contributing
Any contributions, be it bug fixes or new features, are greatly appreciated :grin:.

## Credits
The original implementation of the algorithm used to convert between the spaces was written by the author of the [mjijackson blog](http://mjijackson.com) in javascript. ([A repost of the original blog post](https://axonflux.com/handy-rgb-to-hsl-and-rgb-to-hsv-color-model-c)).