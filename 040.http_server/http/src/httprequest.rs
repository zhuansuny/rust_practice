use std::{collections::HashMap};

#[derive(Debug,PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

// 实现 From trait 则可以用 .into() 从&str转换成Method
impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST"=> Method::Post,
            _ => Method::Uninitialized,
            
        }
    }
}
//http 协议版本
#[derive(Debug,PartialEq, Eq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
    
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _=> Version::Uninitialized,
        }
        
    }
    
}

#[derive(Debug,PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method, //请求方法
    pub version: Version, //http协议版本
    pub resource: Resource,//请求资源
    pub headers: HashMap<String,String>,//请求头
    pub msg_body: String,//请求体
}

impl From<String> for HttpRequest {
    fn from(req:String) -> Self {
        let mut  parsed_method = Method::Uninitialized;
        let mut  parsed_version = Version::V1_1;
        let mut  parsed_resource = Resource::Path("".to_string());
        let mut  parsed_headers = HashMap::new();
        let mut  parsed_msg_body = "";

        for line in req.lines()  {
            if line.contains("HTTP"){
                let(method,resourse,version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resourse;
            }else if line.contains(":") {
                let(key,value) = process_header_line(line);
                parsed_headers.insert(key, value);
                
            }else if line.len() == 0 {
            // 空行
            }else {
            // 消息体
                parsed_msg_body = line;
            }
            
        }

        HttpRequest{
            method:parsed_method,
            version:parsed_version,
            resource:parsed_resource,
            headers:parsed_headers,
            msg_body:parsed_msg_body.to_string(),
        }
        
    }
    
}
// 解析请求行
fn process_req_line(s: &str) -> (Method,Resource,Version){
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into()
    )

   
}
//解析请求头
fn process_header_line(s: &str) -> (String,String){
    let mut items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = items.next(){
        key = k.to_string();
    }
    if let Some(k) = items.next(){
        value = k.to_string();
    }

    ( key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m,Method::Get);
    }
    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v,Version::V1_1);

    }

    #[test]
    fn test_read_http(){
        let s = String::from("GET /greeting HTTP/1.1\r\nHost: localhost\r\nAccept: */*\r\nUser-Agent: curl/7.71.1");
        let mut headers_expected:HashMap<String,String> = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.71.1".into());
        let req: HttpRequest = s.into();

        assert_eq!(Method::Get,req.method);
        assert_eq!(Version::V1_1,req.version);
        assert_eq!(Resource::Path("/greeting".to_string()),req.resource);
        assert_eq!(headers_expected,req.headers);
        

    }

}



