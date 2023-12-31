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


Thanks!

- Ben Stahlhood II


## Running locally

This is built using `winit` and `softbuffer` and should run on all compatible machines. I am currently running macOS Sonoma public beta and Rust version `1.70`


```bash
git clone git@github.com:bstahlhood/tinyrenderer.git
cd tinyrenderer
cargo run
```

## What's Next?

The weekend of September 1st, I will implement OBJ file reading and wireframe
rendering.
