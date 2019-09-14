extern crate url;
use url::form_urlencoded;
use serde_json::{Value};

fn seperator<'a>(len: usize) -> &'a str{
    if len == 0{
        ""
    }else {
        "&"
    }
}

fn transform_key(pkey: &str, key: &str) -> String{
    if pkey.len() == 0{
       key.to_string() 
    } else {
        format!("{}[{}]", pkey, key)
    }
}

fn parse_object(pkey:&str, v: &serde_json::Map<String, Value>)  -> String{
    let mut s = String::with_capacity(50); 
    for (key, val) in v {
        let _s = parse_value(&transform_key(pkey,key), val); 
        s.push_str(seperator(s.len()));
		s.push_str(&_s.to_owned());
    }
    s
}

fn parse_as_string(pkey: &str, v: &str) -> String{
    let s: String = form_urlencoded::Serializer::new(String::with_capacity(pkey.len()+1+v.len()))
        .append_pair(pkey, &v)
        .finish();
    s.replace("+","%20")
}


fn parse_array(pkey: &str, v: &Vec<Value>) -> String {
    let mut s = String::with_capacity(v.len()*(10+pkey.len()+3)); 
    for (idx, value) in v.iter().enumerate() {
        let _s = parse_value(&transform_key(pkey,&idx.to_string()), value);
        s.push_str(seperator(s.len()));
		s.push_str(&_s.to_owned());
    }
    s
}

fn parse_value(pkey:&str, v: &Value) -> String{
	if v.is_object() == true{
		let _s = parse_object(pkey, &v.as_object().unwrap());
        String::from(_s)
	} else if v.is_array() {
		let _s = parse_array(pkey,&v.as_array().unwrap());
        String::from(_s)
	} else if v.is_null() == true {
        String::with_capacity(0)
	} else if v.is_string() == true {
        let _s = parse_as_string(pkey, &v.as_str().unwrap());
        String::from(_s)
	} else if (v.is_boolean() || v.is_number()) == true{
        let _s = parse_as_string(pkey, &v.to_string());
        String::from(_s)
    } else{
        String::with_capacity(0)
    }
}

pub fn json_to_qs(data:&str) -> Result<String, String> {
    let result = serde_json::from_str::<Value>(data);
    match result {
        Ok(v) => {
	        Ok(parse_value("", &v))
        },
        Err(e) => Err(e.to_string()),
    }
}

