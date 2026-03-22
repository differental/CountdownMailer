use chrono::NaiveDate;
use lettre::{
    SmtpTransport,
    transport::smtp::authentication::{Credentials, Mechanism},
};
use std::{env, sync::LazyLock};

pub const DATE_FORMAT: &str = "%Y-%m-%d";

pub static TARGET_DATE: LazyLock<NaiveDate> = LazyLock::new(|| {
    let raw = env::var("TARGET_DATE").expect("TARGET_DATE must be set");

    NaiveDate::parse_from_str(&raw, DATE_FORMAT)
        .unwrap_or_else(|_| panic!("TARGET_DATE must be in the specified format {DATE_FORMAT}"))
});

// Messages in emails
pub static COUNTDOWN_SUBJECT: LazyLock<String> =
    LazyLock::new(|| env::var("COUNTDOWN_SUBJECT").expect("COUNTDOWN_SUBJECT must be set"));

pub static COUNTDOWN_HEADER: LazyLock<String> =
    LazyLock::new(|| env::var("COUNTDOWN_HEADER").expect("COUNTDOWN_HEADER must be set"));

pub static COUNTDOWN_PRE_MESSAGE: LazyLock<String> =
    LazyLock::new(|| env::var("COUNTDOWN_PRE_MSG").expect("COUNTDOWN_PRE_MSG must be set"));

pub static COUNTDOWN_POST_MESSAGE: LazyLock<String> =
    LazyLock::new(|| env::var("COUNTDOWN_POST_MSG").expect("COUNTDOWN_POST_MSG must be set"));

pub static OTD_SUBJECT: LazyLock<String> =
    LazyLock::new(|| env::var("OTD_SUBJECT").expect("OTD_SUBJECT must be set"));

pub static OTD_HEADER: LazyLock<String> =
    LazyLock::new(|| env::var("OTD_HEADER").expect("OTD_HEADER must be set"));

pub static OTD_MESSAGE: LazyLock<String> =
    LazyLock::new(|| env::var("OTD_MSG").expect("OTD_MSG must be set"));

// --- SMTP Email Configurations ---
// Recipient address of all notification emails
pub static EMAIL_ADDRESS: LazyLock<String> =
    LazyLock::new(|| env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS must be set"));

// SMTP Server
pub static SMTP_SERVER: LazyLock<String> =
    LazyLock::new(|| env::var("SMTP_SERVER").expect("SMTP_SERVER must be set"));

// SMTP Credentials
pub static SMTP_CREDS: LazyLock<Credentials> = LazyLock::new(|| {
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
    Credentials::new(smtp_username, smtp_password)
});

// SMTP Port
pub static SMTP_PORT: LazyLock<u16> = LazyLock::new(|| {
    env::var("SMTP_PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(587)
});

// SMTP From
pub static SMTP_FROM: LazyLock<String> =
    LazyLock::new(|| env::var("SMTP_FROM").expect("SMTP_FROM must be set"));

pub static MAILER: LazyLock<SmtpTransport> = LazyLock::new(|| {
    SmtpTransport::starttls_relay(SMTP_SERVER.as_str())
        .expect("Invalid SMTP server")
        .port(*SMTP_PORT)
        .credentials(SMTP_CREDS.clone())
        .authentication(vec![Mechanism::Plain])
        .build()
});

pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
