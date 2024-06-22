# What's ?

It's a simple tool to clone directories contents from the local machine.

## Installation

```bash
cargo install cloning
```

## Usage

```bash
cloning src dest
```

## Customize

### Example

```bash
cloning src dest -t '[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}'
```

* [**Templates**](https://docs.rs/indicatif/0.17.8/indicatif/#templates)
  * `bar`: renders a progress bar. By default, 20 characters wide. The style string is used to color the elapsed part, the alternative style is used for the bar that is yet to render.
  * `wide_bar`: like bar but always fills the remaining space. It should not be used with wide_msg.
  * `spinner`: renders the spinner (current tick string). Note that spinners do not automatically tick by default. You either need to call enable_steady_tick or manually call tick.
  * `prefix`: renders the prefix set on the progress bar.
  * `msg`: renders the currently set message on the progress bar.
  * `wide_msg`: like msg but always fills the remaining space and truncates. It should not be used with wide_bar.
  * `pos`: renders the current position of the bar as integer
  * `human_pos`: renders the current position of the bar as an integer, with commas as the thousands separator.
  * `len`: renders the amount of work to be done as an integer
  * `human_len`: renders the total length of the bar as an integer, with commas as the thousands separator.
  * `percent`: renders the current position of the bar as a percentage of the total length (as an integer).
  * `percent_precise`: renders the current position of the bar as a percentage of the total length (with 3 fraction digits).
  * `bytes`: renders the current position of the bar as bytes (alias of binary_bytes).
  * `total_bytes`: renders the total length of the bar as bytes (alias of binary_total_bytes).
  * `decimal_bytes`: renders the current position of the bar as bytes using power-of-10 units, i.e. MB, kB, etc.
  * `decimal_total_bytes`: renders the total length of the bar as bytes using power-of-10 units, i.e. MB, kB, etc.
  * `binary_bytes`: renders the current position of the bar as bytes using power-of-two units, i.e. MiB, KiB, etc.
  * `binary_total_bytes`: renders the total length of the bar as bytes using power-of-two units, i.e. MiB, KiB, etc.
  * `elapsed_precise`: renders the elapsed time as HH:MM:SS.
  * `elapsed`: renders the elapsed time as 42s, 1m etc.
  * `per_sec`: renders the speed in steps per second.
  * `bytes_per_sec`: renders the speed in bytes per second (alias of binary_bytes_per_sec).
  * `decimal_bytes_per_sec`: renders the speed in bytes per second using power-of-10 units, i.e. MB, kB, etc.
  * `binary_bytes_per_sec`: renders the speed in bytes per second using power-of-two units, i.e. MiB, KiB, etc.
  * `eta_precise`: the remaining time (like elapsed_precise).
  * `eta`: the remaining time (like elapsed).
  * `duration_precise`: the extrapolated total duration (like elapsed_precise).
  * `duration`: the extrapolated total duration time (like elapsed).

### Open dest directory

```bash
cloning src dest -t '[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}' -o
```


## Demo

![demo](cloning-demo.gif)
