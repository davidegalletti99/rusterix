# Rusterix
This project is a Rust revisitation of the C++ library 
[AsterCXX](https://github.com/dgrafe/astericxx) found on GitHub.
AsterCXX is a library for encoding and decoding aeronautical
telemetry data according to the [Asterix](https://en.wikipedia.org/wiki/ASTERIX)
standard maintained by Eurocontrol.

This library, like the astericxx library, is intended to be an implementation of 
the ASTERTIX data types based on a set user-defined configuration files. This 
configuration-driven approach allows users to define and customize the data 
types they need to work with, making the library flexible and adaptable to
various use cases.

## Features
- Easy-to-use Rust API for encoding and decoding ASTERIX data.
- Configuration-driven design for defining custom data types.

## TODO List
- Implement the full ASTERIX standard.
- Implement encoding functionality.
- Write unit tests for all modules.
- Add comprehensive error handling.
- Improve documentation and examples.