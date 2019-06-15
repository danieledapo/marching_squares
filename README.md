# marching-squares

Implementation of the [marching
squares](https://en.wikipedia.org/wiki/Marching_squares) algorithm to find the
boundaries of shapes given a scalar field. This algorithm can also be used to
generate heightmaps.

To understand what the library can do take a look at the examples.

```bash
$ cargo run --release --example function
$ cargo run --release --example heightmap heightmaps/italy.png
```

![function-fill.png](images/function-fill.png)
![italy.png](images/italy.png)
