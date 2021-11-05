# 🌔 ``luau-rs``
> [Luau](https://github.com/Roblox/luau) bindings for the [Rust](https://www.rust-lang.org) programming language using [bindgen](https://github.com/rust-lang/rust-bindgen)

## ⚠️ Disclaimer
This does not provide bindings for everything as luau does not provide an adequate API for C bindings, which trips up bindgen & makes ffi exponentially more difficult (thanks to using C++'s ``std::string`` and whatnot). See [luau/121](https://github.com/Roblox/luau/issues/121). (It is also *untested* thanks to this..)

## Usage
Add this to your ``Cargo.toml``
```toml
[dependencies]
luau = { git = "https://github.com/Vurv78/luau-rs" }
```

## Requirements
See the requirements for bindgen [here](https://rust-lang.github.io/rust-bindgen/requirements.html)