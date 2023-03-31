# VCD Frequency Analyzer

This is a simple program written in Rust that takes a Value Change Dump (VCD) file as input and generates a CSV file with the switching activity frequency over time.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Example](#example)
- [License](#license)
- [Support](#support)

## Prerequisites

To run this program, you need to have Rust installed on your system. If you don't have Rust installed, you can follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

## Installation

To install the VCD Frequency Analyzer, follow these steps:

1. Clone the repository:
`git clone https://github.com/marlls1989/vcd_frequency.git`

2. Change directory to the cloned repository:
`cd vcd_frequency`

3. Build the program:
`cargo build --release`

4. The binary will be located in the `target/release` directory. You can either use it directly from there or copy it to a directory included in your system's `PATH`.

## Usage

To use the VCD Frequency Analyzer, run the following command:

`vcd_frequency <INPUT_FILE> <OUTPUT_FILE>`

Arguments:

- `<INPUT_FILE>`: The path to the input VCD file.
- `<OUTPUT_FILE>`: The path to the output CSV file.

Options:

- `-h`, `--help`: Print help information.

## Example

Suppose you have a VCD file named `input.vcd` and you want to generate a CSV file named `output.csv`. You would run the following command:

vcd_frequency input.vcd output.csv

This will create the `output.csv` file with the switching activity frequency over time.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for more information.

## Support

For support, please open an issue on the [GitHub repository](https://github.com/marlls1989/vcd_frequency/issues) or contact the maintainers.
