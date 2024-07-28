# Tinyrenderer in Rust for fun :)

I was starting to feel a bit of burn out (boredom). Whenever I do, I tend to look
back to my roots of what made me fall in love with coding to begin with:

* Graphics programming
* Game Development
* Operating Systems

So this will be a fun distractor for me on weekends. Feel free to fork, follow
along, or provide feedback if you like.

This attempts to port the original C++ code based on the series of articles
located here [Tiny Renderer Series](https://github.com/ssloy/tinyrenderer/wiki/Lesson-0:-getting-started)

Thanks for checking in on my fun project!

## Running locally

This is built using `winit` and `softbuffer` and should run on all compatible machines. I am currently running macOS Sonoma public beta and Rust version `1.70`


```bash
git clone git@github.com:bstahlhood/tinyrenderer.git
cd tinyrenderer
cargo run
```

## Things to note

I am running htis on macOS. So my main focus is running on macOS. Even though I am using `winit` and in theory this should work on Windows and Linux, I have not tested it.

I am taking an interative approach to this, so to start most of the code will be in `main.rs`. As I progress, I will break things out into Traits and modules.

## What is completed so far?

- [x] Window and simple frame buffer
- [x] Line drawing using Bresenham's Line Drawing Algorithm
- [x] OBJ Model loading
- [x] Wireframe rendering using lines
- [ ] Triangle Rasterization

## What's Next?

The weekend of September 8th, I will implement Triangle Rasterization.
