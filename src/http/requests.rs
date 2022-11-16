use super::method::Method;
use super::method::MethodError;
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::{Formatter, Result as fmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str{
        &self.path
    }
    
    pub fn method(&self) -> &Method{
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString>{
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // 1 match str::from_utf8(buf){
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvaildEncoding),
        // }
        // 2 match str::from_utf8(buf).or(Err(ParseError::InvaildEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }

        let request = str::from_utf8(buf)?;
        //值得注意的是 上面三种写法是等效的，但是在使用 ？时
        //编译器会自动将错误从最近的一个函数（这里是from_utf8）的错误类型
        //转到我们希望的错误类型（这里是ParseError)，我们需要提供对应的impl块的转换方法

        // match get_next_word(request){
        //     Some((method,request)) =>{},
        //     None =>return Err(ParseError::InvalidRequest),
        // }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // match path.find('?'){
        //     Some(i) =>{
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     }
        //     None =>{}
        // }

        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invaild Request",
            Self::InvalidEncoding => "Invaild Encoding",
            Self::InvalidProtocol => "Invail Protocol",
            Self::InvalidMethod => "Invail Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "ParseError {}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "ParseError {}", self.message())
    }
}

//提供错误转换
impl Error for ParseError {}

//这里是从utf-8错误转换成parseError
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // let mut iter = request.chars();
    // loop{
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {}
    //         None => break
    //         }
    // }
    for (index, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..index], &request[index + 1..]));
            //直接+1并不是一个好的选择 除非你知道你要跳过的确实是一byte长的东西 比如现在
        }
    }
    unimplemented!()
}
