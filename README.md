# Menzir

Menzir is a command-line cryptocurrency exchange rate application that allows users to check real-time exchange rates for various crypto coins and convert prices between fiat and digital currencies.

## Features

- Get exchange rates for multiple cryptocurrencies and fiat currencies.
- Convert cryptocurrency prices to fiat or other crypto coins.
- Retrieve historical exchange rate data for any supported coin.

## Installation

Clone the repository and install the dependencies:
## commands
- get exchange for different currencies : cargo run -- -f usd -t link  //you can use fiat to crypto or cryto to fiat
- get exchange for different currencies with amount: cargo run -- -f usd -t link -a 12
- get details of cryptocurrency exchanges : cargo run -- -e Binance // the api will make ur life harder
- get exchange rate of any coin the api supports in a given date: cargo run -- -f bitcoin -d dd-mm-yy    //dont forget to make the names similiar
