use crate::config::{AppConfig, EmailConfig};
use crate::middleware::GlobalCookieMiddleware;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use regex::Regex;
use std::sync::OnceLock;
use ureq::Agent;
use ureq::http::StatusCode;
use url::Url;

const BASE_URL: &str = "https://www.v2ex.com";
static REDEEM_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn auto_sign_in(config: &AppConfig) -> anyhow::Result<()> {
    let agent: Agent = Agent::config_builder()
        .max_redirects(0)
        .middleware(GlobalCookieMiddleware {
            cookie: config.cookie.clone(),
        })
        .build()
        .into();

    let sign_in_page_url = Url::parse(BASE_URL)?.join("/mission/daily")?;
    let sign_in_page_response = agent.get(sign_in_page_url.as_str()).call()?;
    if sign_in_page_response.status() == StatusCode::FOUND {
        eprintln!("Cookie 已失效，被重定向到登录页");
        if let Some(email_config) = &config.email_config {
            send_notification(
                "V2EX自动签到程序提示您：Cookie 已失效，请更换",
                email_config,
            )?;
        }
        return Ok(());
    }

    let sign_in_page_text = sign_in_page_response.into_body().read_to_string()?;

    if let Some(path) = extract_sign_in_url(&sign_in_page_text)? {
        let sign_in_url = Url::parse(BASE_URL)?.join(&path)?;
        match agent.get(sign_in_url.as_str()).call()?.status() {
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
                    )?;
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

fn send_notification(message: &str, config: &EmailConfig) -> anyhow::Result<()> {
    let creds = Credentials::new(config.smtp_user.clone(), config.smtp_pass.clone());

    let mailer = SmtpTransport::relay(&config.smtp_url)?
        .credentials(creds)
        .build();

    let email = Message::builder()
        .from(config.notify_from.parse()?)
        .to(config.notify_to.parse()?)
        .subject("V2EX 签到失败通知")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(message))?;
    mailer.send(&email)?;
    Ok(())
}
