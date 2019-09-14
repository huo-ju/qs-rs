# qs-rust


Inspired by javascript [qs](https://github.com/ljharb/qs) module



## Usage

```rust

extern crate qs_rs;

use crate::qs_rs::qs;

let data = r#"
    {"b":1, "a":{"c":1,"b":2},"c":"c_st[ring]"}
"#;

let result  = qs::json_to_qs(data);

match result {
    Ok(_s)=>{
        assert_eq!("a%5Bb%5D=2&a%5Bc%5D=1&b=1&c=c_st%5Bring%5D", _s);
    },
    Err(_e)=>{
    }
}
```
