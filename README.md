# Bondsbot

A Rust CLI tool that checks your UK Premium Bonds against the latest high-value winning numbers from NS&I (National Savings & Investments).

## Overview

Bondsbot scrapes the [NS&I Prize Checker](https://www.nsandi.com/prize-checker/winners) website and compares the winning bond numbers against your own bonds to see if you've won a high-value prize. High-value prizes are published on the Prize Checker on the first working day of the month, the day before other prizes, so you can see if you've won before you get the official notification on the app!

## Features

-   Scrapes latest winning numbers from NS&I website
-   Command-line interface for easy bond specification
-   Validates your bond data format
-   Checks your bonds against all winning numbers
-   Reports any matches with prize amounts
-   Verbose mode for detailed checking output
-   Comprehensive test coverage

## Installation

### Option 1: Install from Source

1. Clone this repository:

```bash
git clone https://github.com/jasonleefrench/bondsbot
cd bondsbot
```

2. Install the CLI tool globally:

```bash
cargo install --path .
```

### Option 2: Development Build

```bash
cargo build --release
```

## Usage

### Command Line Interface

Run the checker by specifying your bonds directly on the command line:

```bash
bondsbot "420AB123456-420AB123500,300XY987654,591CD789000-591CD789050"
```

Or if using the development build:

```bash
cargo run -- "420AB123456-420AB123500,300XY987654,591CD789000-591CD789050"
```

### Bond Format

You can specify bonds in two ways:

**Single bonds:** Just provide the bond number

-   Example: `300XY987654`

**Bond ranges:** Use hyphen to separate start and end bonds

-   Format: `PREFIX-STARTNUMBER-PREFIX-ENDNUMBER`
-   Example: `420AB123456-420AB123500`

**Mixed format:** Combine single bonds and ranges, separated by commas

-   Example: `"300XY987654,420AB123456-420AB123500,591CD789000"`

### Verbose Mode

Use the `--verbose` flag to see detailed checking output:

```bash
bondsbot "420AB123456-420AB123500" --verbose
```

### Command Line Options

-   `bonds`: Bond ranges to check (positional argument, required)
-   `--verbose` or `-v`: Enable verbose output showing detailed checking process

The tool will:

1. Parse and validate your bond ranges
2. Fetch the latest winning numbers from NS&I
3. Check each of your bonds against the winners
4. Display any matches with prize amounts

## Testing

Run the test suite:

```bash
cargo test
```

## Disclaimer

Please respect NS&I's terms of service when using this tool. This tool is unofficial, for personal use only, and is not affiliated with NS&I. Always verify any potential winnings through official NS&I channels.
