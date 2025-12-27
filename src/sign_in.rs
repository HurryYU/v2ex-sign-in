use crate::config::{AppConfig, EmailConfig};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::redirect::Policy;
use reqwest::{StatusCode, Url};
use std::sync::OnceLock;

const BASE_URL: &str = "https://www.v2ex.com";
static REDEEM_REGEX: OnceLock<Regex> = OnceLock::new();

pub async fn auto_sign_in(config: &AppConfig) -> anyhow::Result<()> {
    let mut headers = HeaderMap::new();
    if let Ok(value) = HeaderValue::from_str(&config.cookie) {
        headers.insert("Cookie", value);
    }
    let client = reqwest::Client::builder()
        .redirect(Policy::none())
        .default_headers(headers)
        .build()?;

    let sign_in_page_url = Url::parse(BASE_URL)?.join("/mission/daily")?;
    let sign_in_page_response = client.get(sign_in_page_url).send().await?;
    if sign_in_page_response.status() == StatusCode::FOUND {
        eprintln!("Cookie 已失效，被重定向到登录页");
        if let Some(email_config) = &config.email_config {
            send_notification(
                "V2EX自动签到程序提示您：Cookie 已失效，请更换",
                email_config,
            )
            .await?;
        }
        return Ok(());
    }

    let sign_in_page_text = sign_in_page_response.text().await?;

    if let Some(path) = extract_sign_in_url(&sign_in_page_text)? {
        let sign_in_url = Url::parse(BASE_URL)?.join(&path)?;
        match client.get(sign_in_url).send().await?.status() {
            StatusCode::FOUND => println!("签到成功"),
            status => {
                eprintln!("签到失败，请稍后重试.状态码: {}", status);
                if let Some(email_config) = &config.email_config {
                    send_notification(
                        &format!(
                            "V2EX自动签到程序提示您：签到失败，请稍后重试。HTTP状态码：{}",
                            status.as_u16()
                        ),
                        email_config,
                    )
                    .await?;
                }
            }
        }
    } else {
        eprintln!("今日可能已经签到")
    }
    Ok(())
}

fn extract_sign_in_url(html: &str) -> anyhow::Result<Option<String>> {
    let re = REDEEM_REGEX.get_or_init(|| {
        Regex::new(r"location\.href\s*=\s*'(/mission/daily/redeem[^']+)'").expect("正则编译失败")
    });

    let result = re.captures(html).map(|caps| caps[1].to_string());

    Ok(result)
}

async fn send_notification(message: &str, config: &EmailConfig) -> anyhow::Result<()> {
    let creds = Credentials::new(config.smtp_user.clone(), config.smtp_pass.clone());

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_url)?
            .credentials(creds)
            .build();

    let email = Message::builder()
        .from(config.notify_from.parse()?)
        .to(config.notify_to.parse()?)
        .subject("V2EX 签到失败通知")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(message))?;
    mailer.send(email).await?;
    Ok(())
}
