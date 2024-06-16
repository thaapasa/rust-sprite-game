# 2D Sprite-based Game in Rust

A simple 2D game developed using Rust and ggez. Control a
character, make it walk, run, and interact within a 2D
environment.

## Features

- **Character Movement**: Use arrow keys to move the character left
  or right.
- **Running Mechanic**: Hold the shift key while moving to make the
  character run.
- **Sprite Animation**: Smooth animations for walking and running using
  sprite sheets.
- **Easy Exit**: Press `Q` or `ESC` to exit the game anytime.

## Installation

1. **Clone the Repository**:

```bash
git clone https://github.com/thaapasa/rust-sprite-game
```

2. **Navigate to the Directory**:

```bash
cd rust-sprite-game
```

3. **Install Required Packages**:

Make sure you have Rust installed:
[https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)

## Usage

Run the game using:

```bash
cargo run
```

## Development

In-game coordinates are as follows:

- `+y` is up
- `+x` is right
- `0,0` is lower left corner of game screen
- Actor position is the bottom left corner of the actor's bounding box
- Hence, full actor bbox position is:
  `(actor x, actor y) to (actor x + actor width, actor y + actor height)`
- Note: Character sprites are far larger than the character bounding box.
  The sprite tiles are 128x128 pixels, but the bounding box is only
  42x74 pixels (centered on the x-axis, aligned to bottom on the y-axis).

## License

[MIT](./LICENSE)
