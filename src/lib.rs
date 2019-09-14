pub mod qs;

#[cfg(test)]
mod tests {
    use crate::qs;

    #[test]
    fn json_to_qs_array() {
        let data = r#"
            [1,2,3,4]
        "#;
        let result  = qs::json_to_qs(data);
        match result {
            Ok(_s)=>{
                assert_eq!("0=1&1=2&2=3&3=4", _s);
            },
            Err(_e)=>{
            }
        }
    }
    #[test]
    fn json_to_qs_sort() {
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
    }

    #[test]
    fn json_to_qs_nest() {
        let data = r#"
            {"a a+b":1, "b":{"c":1,"d":{"e":2,"f":"f_ string"}},"c":"c_st[ring]","g":{"list": [1,2,3,4]}}
        "#;
        let result  = qs::json_to_qs(data);
        match result {
            Ok(_s)=>{
                assert_eq!("a%20a%2Bb=1&b%5Bc%5D=1&b%5Bd%5D%5Be%5D=2&b%5Bd%5D%5Bf%5D=f_%20string&c=c_st%5Bring%5D&g%5Blist%5D%5B0%5D=1&g%5Blist%5D%5B1%5D=2&g%5Blist%5D%5B2%5D=3&g%5Blist%5D%5B3%5D=4", _s);
            },
            Err(_e)=>{
            }
        }
    }
}

