use askama::Template;
use chrono::Local;
use lettre::{Message, Transport, message::header::ContentType};

use crate::constants::{
    CARGO_PKG_VERSION, COUNTDOWN_HEADER, COUNTDOWN_POST_MESSAGE, COUNTDOWN_PRE_MESSAGE,
    COUNTDOWN_SUBJECT, EMAIL_ADDRESS, MAILER, OTD_HEADER, OTD_MESSAGE, OTD_SUBJECT, SMTP_FROM,
    TARGET_DATE,
};

#[derive(Template)]
#[template(path = "email_countdown.html")]
struct CountdownEmailTemplate<'a> {
    subject: &'a str,
    header: &'a str,
    message_pre: &'a str,
    number: u64,
    message_post: &'a str,
    version: &'a str,
}

#[derive(Template)]
#[template(path = "email_completed.html")]
struct OnTheDayEmailTemplate<'a> {
    subject: &'a str,
    header: &'a str,
    message: &'a str,
    version: &'a str,
}

pub fn send_email(from: &str, to: &str, subject: &str, body: &str) -> anyhow::Result<()> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body.to_string())?;

    MAILER.send(&email)?;
    Ok(())
}

pub fn send_countdown() {
    let days_diff = (*TARGET_DATE)
        .signed_duration_since(Local::now().date_naive())
        .num_days();

    match days_diff {
        1.. => {
            let countdown_template = CountdownEmailTemplate {
                subject: &COUNTDOWN_SUBJECT,
                header: &COUNTDOWN_HEADER,
                message_pre: &COUNTDOWN_PRE_MESSAGE,
                number: days_diff as u64,
                message_post: &COUNTDOWN_POST_MESSAGE,
                version: CARGO_PKG_VERSION,
            };
            let countdown_email_content = countdown_template
                .render()
                .expect("Countdown email failed to render");

            if let Err(ref err) = send_email(
                &SMTP_FROM,
                &EMAIL_ADDRESS,
                &COUNTDOWN_SUBJECT,
                &countdown_email_content,
            ) {
                eprintln!("Failed to send countdown email: {err:?}");
            }
        }
        0 => {
            let otd_template = OnTheDayEmailTemplate {
                subject: &OTD_SUBJECT,
                header: &OTD_HEADER,
                message: &OTD_MESSAGE,
                version: CARGO_PKG_VERSION,
            };
            let otd_email_content = otd_template.render().expect("OTD email failed to render");

            if let Err(ref err) =
                send_email(&SMTP_FROM, &EMAIL_ADDRESS, &OTD_SUBJECT, &otd_email_content)
            {
                eprintln!("Failed to send on-the-day email: {err:?}");
            }
        }
        ..=-1 => (),
    };
}
