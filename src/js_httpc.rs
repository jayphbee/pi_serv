use std::path::PathBuf;
use std::sync::Arc;
use std::io;

use httpc;
use httpc::SharedHttpc;
use atom::Atom;

/**
* http客户端选项
*/
pub struct HttpClientOptions(httpc::HttpClientOptions);


impl HttpClientOptions {
    /**
    * 创建默认http客户端选项
    * @returns 返回http客户端选项
    */
    pub fn default() -> Self{
        HttpClientOptions(httpc::HttpClientOptions::Default)
    }

    /**
    * 创建一般http客户端选项
    * @param https 是否使用安全的http协议
    * @param gzip 是否gzip压缩
    * @param referer 是否支持重定向
    * @param count 重定向最大次数
    * @param timeout 请求超时时长，单位毫秒
    * @returns 返回http客户端选项
    */
    pub fn normal(https: bool, gzip: bool, referer: bool, count: isize, timeout: u64) -> Self{
        HttpClientOptions(httpc::HttpClientOptions::Normal(https, gzip, referer, count, timeout))
    }

    /**
    * 创建验证主机的http客户端选项
    * @param cert_file 证书文件路径
    * @param identity_file 身份文件路径
    * @param pk 密钥
    * @param gzip 是否gzip压缩
    * @param referer 是否支持重定向
    * @param count 重定向最大次数
    * @param timeout 请求超时时长，单位毫秒
    * @returns 返回http客户端选项
    */
    pub fn vaild_host(cert_file: String, identity_file: String, pk: String, gzip: bool, referer: bool, count: isize, timeout: u64) -> Self{
        let mut cert_file_p = PathBuf::new();
        for v in cert_file.split("/"){
            cert_file_p.push(v);
        }

        let mut identity_file_p = PathBuf::new();
        for v in identity_file.split("/"){
            identity_file_p.push(v);
        }
        
        HttpClientOptions(httpc::HttpClientOptions::VaildHost(cert_file_p, identity_file_p, pk, gzip, referer, count, timeout))
    }

    /**
    * 创建代理http客户端选项
    * @param proxy_url 代理服务器url
    * @param https 是否使用安全的http协议
    * @param gzip 是否gzip压缩
    * @param referer 是否支持重定向
    * @param count 重定向最大次数
    * @param timeout 请求超时时长，单位毫秒
    * @returns 返回http客户端选项
    */
    pub fn proxy(proxy_url: String, https: bool, gzip: bool, referer: bool, count: isize, timeout: u64) -> Self{
        HttpClientOptions(httpc::HttpClientOptions::Proxy(Atom::from(proxy_url), https, gzip, referer, count, timeout))
    }


    /**
    * 创建验证主机的代理http客户端选项
    * @param cert_file 证书文件路径
    * @param identity_file 身份文件路径
    * @param pk 密钥
    * @param proxy_url 代理服务器url
    * @param gzip 是否gzip压缩
    * @param referer 是否支持重定向
    * @param count 重定向最大次数
    * @param timeout 请求超时时长，单位毫秒
    * @returns 返回http客户端选项
    */
    pub fn valid_host_proxy(cert_file: String, identity_file: String, pk: String, proxy_url: String, gzip: bool, referer: bool, count: isize, timeout: u64) -> Self{
        let mut cert_file_p = PathBuf::new();
        for v in cert_file.split("/"){
            cert_file_p.push(v);
        }

        let mut identity_file_p = PathBuf::new();
        for v in identity_file.split("/"){
            identity_file_p.push(v);
        }

        HttpClientOptions(httpc::HttpClientOptions::ValidHostProxy(cert_file_p, identity_file_p, pk, Atom::from(proxy_url),  gzip, referer, count, timeout))
    }
}

/**
* Http的Body
*/
pub struct HttpClientBody<T: httpc::GenHttpClientBody>(httpc::HttpClientBody<T>);

impl<T: httpc::GenHttpClientBody> HttpClientBody<T>{
    /**
    * 获取指定关键字的json值
    */
    pub fn get_json_val(&self, key: String) -> Option<&String> {
        self.0.get_json_val(Atom::from(key))
    }

    /**
    * 增加json键值对，返回键值对数量
    */
    pub fn add_json_kv(&mut self, key: String, value: String) -> usize {
        self.0.add_json_kv(Atom::from(key), value)
    }

    /**
    * 移除指定关键字的json键值对，返回被移除的值
    */
    pub fn remove_json_kv(&mut self, key: String) -> Option<String> {
        self.0.remove_json_kv(Atom::from(key))
    }

    /**
    * 清空所有json键值对
    */
    pub fn clear_json_kvs(&mut self) {
        self.0.clear_json_kvs()
    }

    /**
    * 增加表单键值对
    */
    pub fn add_form_kv(self, key: String, value: String) -> Self {
        HttpClientBody(self.0.add_form_kv(Atom::from(key), value))
    }

    /**
    * 增加表单文件
    */
    pub fn add_form_file(self, key: String, file: String) -> Result<Self, String> {
        match self.0.add_form_file(Atom::from(key), file) {
            Ok(r) => Ok(HttpClientBody(r)),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl HttpClientBody<Vec<u8>>{
    /**
    * 创建二进制的body
    * @param body 数据
    * @returns 返回Http的Body
    */
    pub fn body(body: Vec<u8>) -> Self {
        HttpClientBody(httpc::HttpClientBody::body(body))
    }

    
}

impl HttpClientBody<String>{
    /**
    * 创建文本的body
    * @param body 文本
    * @returns 返回Http的Body
    */
    pub fn body(body: String) -> Self {
        HttpClientBody(httpc::HttpClientBody::body(body))
    }

    /**
    * 创建json的body
    * @param key 关键字
    * @param value 值
    * @returns 返回Http的Body
    */
    pub fn json(key: Atom, value: String) -> Self{
        HttpClientBody(httpc::HttpClientBody::json(key, value))
    }

    /**
    * 创建表单的body
    * @param key 关键字
    * @param value 值
    * @returns 返回Http的Body
    */
    pub fn form(key: String, value: String) -> Self{
        HttpClientBody(httpc::HttpClientBody::form(Atom::from(key), value))
    }
}

/**
* 创建一个指定http客户端选项的http客户端
* @param options http客户端选项
* @returns 返回创建结果，成功返回http客户端的引用计数
* @throws 失败抛出原因描述
*/
pub fn create_http_client(options: HttpClientOptions) -> Result<Arc<httpc::HttpClient>, String> {
    match httpc::HttpClient::create(options.0){
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}

/**
* 使用指定http客户端向指定的url发送get请求
* @param client http客户端的引用计数
* @param url http服务器的url
* @param body Http的Body
* @param callback 发送结果的异步回调，成功返回http客户端的引用计数和http响应，失败返回原因描述
*/
pub fn get<T: httpc::GenHttpClientBody>(client: &Arc<httpc::HttpClient>, url: Atom, body: HttpClientBody<T>, callback: Box<FnOnce( Result<(Arc<httpc::HttpClient>, httpc::HttpClientResponse), String>)>){
    let c = Box::new(|c: Arc<httpc::HttpClient>, r: io::Result<httpc::HttpClientResponse>|{
        match  r {
            Ok(v) => callback(Ok((c, v))),
            Err(e) => callback(Err(e.to_string())),
        }
    });
    httpc::HttpClient::get(client, url, body.0, c);
}

/**
* 使用指定http客户端向指定的url发送post请求
* @param client http客户端的引用计数
* @param url http服务器的url
* @param body Http的Body
* @param callback 发送结果的异步回调，成功返回http客户端的引用计数和http响应，失败返回原因描述
*/
pub fn post<T: httpc::GenHttpClientBody>(client: &Arc<httpc::HttpClient>, url: Atom, body: HttpClientBody<T>, callback: Box<FnOnce( Result<(Arc<httpc::HttpClient>, httpc::HttpClientResponse), String>)>){
    let c = Box::new(|c: Arc<httpc::HttpClient>, r: io::Result<httpc::HttpClientResponse>|{
        match  r {
            Ok(v) => callback(Ok((c, v))),
            Err(e) => callback(Err(e.to_string())),
        }
    });
    httpc::HttpClient::post(client, url, body.0, c);
}