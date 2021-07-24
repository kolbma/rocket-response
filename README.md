[![Rust](https://github.com/kolbma/rocket-response/actions/workflows/rust.yml/badge.svg)](https://github.com/kolbma/rocket-response/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/rocket-response)](https://crates.io/crates/rocket-response)
[![docs](https://docs.rs/rocket-response/badge.svg)](https://docs.rs/rocket-response)

# rocket-response - Provides enum for variable Rocket Responses

This [crate] provides 3 enums to be flexible in returning [Responses].

1. [RocketResponse] provides all non-generic [Response] types.
2. [RocketResponseGeneric] provides [Response]-types non-generic
   and generic using a single generic type.
3. [RocketResponseGeneric2] allows a different [Flash] type.

If you miss any [Response], you are welcome to open an [issue]
or even better provide a pull-request!

Because it is difficult to decide on the generics what might be useful, your usecases are really
welcome in an [issue].

## Usage

For usage add the crate to your dependencies

```toml
[dependencies]
rocket-response = { version = "0.0.1-rc.1" }
```

## Features

You can depend on a couple of features, which provide additional types.

* json
* msgpack
* templates-handlebars or templates-tera

```toml
[dependencies]
rocket-response = { version = "0.0.1-rc.1", features = ["json", "templates-tera"] }
```

## Example

```rust
use rocket::{get, response::{self, status, Redirect}};
use rocket_response::RocketResponseGeneric2 as RocketResponse;

#[get("/<id>")]
pub(crate) fn rocket_response_generic2(
    id: usize,
) -> RocketResponse<&'static str, Redirect> {
    match id {
        0 => RocketResponse::Flash(response::Flash::error(
          Redirect::to("/"),
            format!("Invalid id {}", id),
        )),
        1 => RocketResponse::Unauthorized(status::Unauthorized(Some(
            "admin need authentication",
        ))),
        _ => RocketResponse::Html(response::content::Html(
            "<html><body>Hello world</body></html",
        )),
    }
}
```

[crate]: https://crates.io/crates/rocket-response/
[RocketResponse]: https://docs.rs/rocket-response/latest/rocket_response/enum.RocketResponse.html
[RocketResponseGeneric]: https://docs.rs/rocket-response/latest/rocket_response/enum.RocketResponseGeneric.html
[RocketResponseGeneric2]: https://docs.rs/rocket-response/latest/rocket_response/enum.RocketResponseGeneric2.html
[Response]: https://docs.rs/rocket/0.5.0-rc.1/rocket/response/struct.Response.html
[Responses]: https://docs.rs/rocket/0.5.0-rc.1/rocket/response/struct.Response.html
[Flash]: https://docs.rs/rocket/0.5.0-rc.1/rocket/response/struct.Flash.html
[issue]: https://github.com/kolbma/rocket-response/issues

## Licenses

You can choose between __[MIT License](https://opensource.org/licenses/MIT)__ or __[Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0)__.

### MIT License

Copyright (c) 2021 Markus Kolb

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice (including the next paragraph) shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

### Apache License 2.0

Copyright 2021 Markus Kolb

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
