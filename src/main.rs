use crate::email::send_countdown;

mod constants;

mod email;

fn main() {
    dotenvy::dotenv().ok();

    send_countdown();
}
