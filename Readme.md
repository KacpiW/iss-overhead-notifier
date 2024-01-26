# ISS Tracker

This is a simple command-line application written in Rust that tracks the International Space Station (ISS). It checks whether the ISS is in close distance to a given latitude and longitude and whether it's dark at that location so you can see it.

## Prerequisites

You need to have Rust installed on your machine. If you don't have it installed, you can download it from [here](https://www.rust-lang.org/tools/install).

## Installation

Clone the repository to your local machine:

```bash
git clone https://github.com/yourusername/iss-tracker.git
cd iss-tracker
```

## Build the project:

```bash
cargo build --release
```

## Usage

You can run the application with the *--latitude* and *--longitude* options to specify your location. For example:

```bash
./target/release/issfire --latitude=-40.7128 --longitude=-74.0060
```

This will check whether the ISS is in close distance to New York City and whether it's currently dark there.