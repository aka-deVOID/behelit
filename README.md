<br/>
<p align="center">
  <h3 align="center">Behelit</h3>
  <p align="center">
    The lightning-fast, asynchronous, implement base Web Framework
    <br/>
    <br/>
    <a href="https://github.com/MahanBi/Behelit"><strong>Explore the docs »</strong></a>
    <br/>
    <br/>
    <a href="https://github.com/MahanBi/Behelit/issues">Report Bug</a>
    .
    <a href="https://github.com/MahanBi/Behelit/issues">Request Feature</a>
  </p>
</p>

![Contributors](https://img.shields.io/github/contributors/MahanBi/Behelit?color=dark-green) ![License](https://img.shields.io/github/license/MahanBi/Behelit)

## Table Of Contents

- [Table Of Contents](#table-of-contents)
- [About The Project](#about-the-project)
- [Getting Started](#getting-started)
  - [Installation](#installation)
- [License](#license)
- [Authors](#authors)

## About The Project

Behelit is a web framework written in Rust. Its features are implementation base and asynchronous. but currently, an async trait is not released in this version of Rust, correctly working on the nightly version, and the [async_trait](https://github.com/dtolnay/async-trait) library.

## Getting Started

This is an example of how you may give instructions on setting up your project locally.

### Installation

- install from cargo

```sh
    cargo add behelit
```

- or add this line to your Cargo.toml

```txt
    behelit = "0.0.0"
```

## License

Distributed under the MIT License. See [LICENSE](https://github.com/MahanBi/Behelit/blob/main/LICENSE.md) for more information.

## Authors

- **deVOID** - **- [deVOID](https://github.com/MahanBi) -**

# TODO

- [ ] Core
  - [ ] write listener for TCP(http1/2) and UDP(http3)
    - [ ] with builder design pattern for building a customizable listener server on top of TokIO
 
- [ ] Proto
  - [ ] http1
  - [ ] http2
  - [ ] http3

- [ ] Request/Response
  - [ ] write request parser fast and lightweight with the lowest count of copy and clone to make it look like no std. it means to try to don't use the String type.

- [ ] Clean
  - [ ] Cargo.toml: remove additional packages and features
