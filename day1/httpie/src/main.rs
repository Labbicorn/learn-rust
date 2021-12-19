use anyhow::{anyhow, Result};
use colored::Colorize;
use mime::Mime;
use reqwest::Url;
use reqwest::{header, Client, Response};
use std::collections::HashMap;
use std::str::FromStr;
use structopt::StructOpt;

// 定义HTTPied的CLI的主入口，它包含若干个字命令
// 下面 /// 的注释是文档，clap会将其作为CLI的帮助

#[derive(StructOpt, Debug)]
#[structopt(name = "httpie")]
struct Opts {
    #[structopt(subcommand)]
    subcmd: Subcommand,
}

// 子命令分别对应不同的HTTP方法，目前只支持get/post
#[derive(StructOpt, Debug)]
enum Subcommand {
    Get(Get),
    Post(Post),
    // 我们暂时不支持其它的http方法
}

// get子命令

/// feed get with an url and will retrieve the response for you
#[derive(StructOpt, Debug)]
struct Get {
    /// HTTP请求的URL
    #[structopt(parse(try_from_str = parse_url))]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;

    Ok(s.into())
}

// post 子命令，需要输入一个URL，和若干可选的key=value,用于提供json body

/// feed post with an url and optional key=value pairs. we will post the data
/// as JSON, and retrieve the response for you
#[derive(StructOpt, Debug)]
struct Post {
    /// HTTP  请求的URL
    #[structopt(parse(try_from_str = parse_url))]
    url: String,
    /// HTTP 请求的body
    #[structopt(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

/// 命令行中的key=value 可以通过parse_kv_pair 解析KvPair结构
#[derive(StructOpt, Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行split, 这会得到一个迭代器
        let mut split = s.split('=');
        let err = || anyhow!(format!("Failed to parse{}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为key,迭代器返回Some(T)/None,
            // 我们将其转换成Ok(T)/Err(E), 然后用?处理错误。
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

// cargo run -- post httpbin.org/post a=1 b=2
#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::from_args();
    println!("Opts: {:?}", opts);

    let client = Client::new();

    let result = match opts.subcmd {
        Subcommand::Get(ref args) => get(client, args).await?,
        Subcommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    // println!("{:?}", resp.text().await?);
    // Ok(())
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    // println!("{:?}", resp.text().await?);
    // Ok(())
    Ok(print_resp(resp).await?)
}

// 打印服务器的版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

// 打印服务器返回的HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }

    println!("\n");
}

fn print_body(m: Option<Mime>, body: &str) {
    match m {
        // 对于 "application/json" 我们pretty print
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        // 其它 mime type 直接输出
        _ => println!("{}", body),
    }
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);

    let mime = get_content_type(&resp);

    let body = resp.text().await?;
    print_body(mime, &body);

    Ok(())
}

// 将服务器返回的content-type 解析成Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("http://httpbin.org/post").is_ok())
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into(),
            }
        );

        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into(),
            }
        )
    }
}
