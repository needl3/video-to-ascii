# Video to ASCII Converter | Convert images or live cam videos to ASCII format

## How to use

1. Install the dependencies with `cargo`
2. Print help with `cargo run`
```bash
> cargo run
Usage: ./asciiman <options>
Options:
	--video /dev/video0
	--image /path/to/image
		--color <options>
Color options:
No options: Only use foreground colors
bgcolor : Use background color
Note: Omit --color to not use colored rendering
```
3. Run with a video card as `cargo run -- --video /dev/video0`
4. Run with an image as `cargo run -- --image /path/to/image`
5. Run with more color support as `cargo run -- --video /dev/video0 --color`
6. Run with background color support as `cargo run -- --video /dev/video0 --color bgcolor`

Watch Demo: https://youtu.be/dUp_vIxBgLc
