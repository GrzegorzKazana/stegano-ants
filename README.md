# Ant system based steganography 

Code in this repository is the implementation of my master's thesis on the use of ant colony optimization in image steganography. It is written in Rust.

## How to run

### Building and running locally

This steps assume you have `rust` toolchain installed. If not, head over to [official installation guide](https://www.rust-lang.org/tools/install). Or go to [Building inside docker section](#building-inside-docker).

#### Building the binary

```bash
# after cloning the repository
cargo build --release
```

#### Running the app

```bash
# using the build binary
./target/release/stegano-ants <app-arguments>

# or without specyfing full path
cargo run --release -- <app-arguments>
```

### Building inside docker

If for some reason you do not want to install `rust` toolchain, you can always build the image inside `docker` container with the provided `Dockerfile`.

#### Building the binary

```bash
docker build -t ants --target runner .
```

#### Running the app

```bash
docker run \
    --rm \
    -v `pwd`:/usr/src/stegano-ants \
    ants stegano-ants <app-arguments>
```

## App arguments

Application takes in arguments which configure the ant colony and embedding/extraction process.

Example of embedding:

_(In the following examples replace `stegano-ants` with a way of running the app of your choice from steps above)._

```bash
stegano-ants \
    --ants=1000 \
    --cycles=10 \
    --steps=100 \
    --dispatcher=basic: \
    --updater=const:1.0,0.001,0.5 \
    --mask-width=100 \
    --target-capacity=10000B \
    embed \
    --data assets/data/lorem_ipsum_large.txt \
    --image assets/images/house/house-m.bmp
```

Example of extracting:

```bash
stegano-ants \
    --ants=1000 \
    --cycles=10 \
    --steps=100 \
    --dispatcher=basic: \
    --updater=const:1.0,0.001,0.5 \
    --mask-width=100 \
    --target-capacity=10000B \
    extract \
    --steg assets/images/house/house-m_steg.bmp \
    --image assets/images/house/house-m.bmp
```

For help run:

```bash
stegano-ants --help
```

For convenience, in bash you can also store arguments inside `args.txt`, and invoke the app as follows:

```bash
stegano-ants `cat args.txt | grep -v ^#`
```

## Output

In embedding mode, two images will be generated. They will be placed in the same directory as transport image.

-   `<image_name>_steg.bmp` - steganogram containing the hidden message
-   `<image_name>_pher.bmp` - visualization of the pheromone trail created by ants, for development purposes only

In extract mode the secret message will be printed to stdout.
