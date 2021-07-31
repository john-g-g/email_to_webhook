use std::env;
use std::io;
use std::io::prelude::*;

use reqwest;
use serde_json;
use serde_json::json;

use mailparse::*;

#[macro_use]
extern crate log;
use env_logger;

fn fmt_slack(message: &ParsedMail) -> serde_json::Value {
    return json!({
        "text": format!("Email from {}", message.headers.get_first_value("From").expect("Missing 'From' email header")),
        "attachments": [
            {
                "title": format!("{}", message.headers.get_first_value("Subject").expect("Missing 'Subject' email header")),
                "text": format!("{}",message.get_body().expect("Missing email body")),
                "fallback": format!("{}",message.get_body().expect("Missing email body"))
            }
        ]
    });
}

fn main() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "<{}>{}: {}",
                match record.level() {
                    log::Level::Error => 3,
                    log::Level::Warn => 4,
                    log::Level::Info => 6,
                    log::Level::Debug => 7,
                    log::Level::Trace => 7,
                },
                record.target(),
                record.args()
            )
        })
        .init();

    let hook_url = env::var("SLACK_WEBHOOK_URL").expect("Must define 'SLACK_WEBHOOK_URL'");

    let mut buffer = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle
        .read_to_end(&mut buffer)
        .expect("failed to read email massage");

    let message = parse_mail(&buffer).expect("unable to parse email message");
    info!("Parsed Message Headers {:?}", message.headers);
    info!("Parsed Message Body {:?}", message.get_body());

    let slack_message = fmt_slack(&message);
    info!("{}", slack_message);

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(hook_url)
        .json(&slack_message)
        .send()
        .expect("Failed to make webhook request");

    match response.error_for_status() {
        Ok(_response) => {
            info!("webhook response: {:?}", _response);
        }
        Err(err) => {
            error!("webhook request failed: {}", &err);
            panic!("webhook request failed: {}", &err);
        }
    }
}
