extern crate chrono;
extern crate reqwest;

use chrono::{Duration, Utc};
use clap::{App, Arg, ArgMatches};
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    status_text: Option<String>,
    status_emoji: Option<String>,
    status_expiration: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProfileRequest {
    profile: Profile,
}

const SLACK_URL: &str = "https://slack.com/api/users.profile.set";

fn main() {
    let matches = App::new("Slacktus - Change your slack status with one command.")
	.version("1.0")
	.author("David Bonet <dbmontes@gmail.com>")
	.about("It updates your slack status in one command. Perfect for pomodoro apps and similar.")
	.arg(Arg::with_name("token")
		.short("t")
		.long("token")
		.help("The Slack API token used for the API call. You can get your token here: https://api.slack.com/custom-integrations/legacy-tokens")
		.required(true)
		.takes_value(true))
	.arg(Arg::with_name("api_url")
		.long("api-url")
		.help(&format!("The Slack API url. By default is {}", SLACK_URL))
		.required(false)
		.takes_value(true))
	.arg(Arg::with_name("status")
		.short("s")
		.long("status")
		.help("The status text you want in your slack.")
		.takes_value(true))
	.arg(Arg::with_name("emoji")
		.short("e")
		.long("emoji")
		.help("The emoji you want in your slack. Use the text of one of the slack's emojis.")
		.takes_value(true))
	.arg(Arg::with_name("expiration")
		.short("x")
		.long("expiration")
		.help("When is it going to expire? Pass a date in UNIX timestamp format. 0 by default (won't expire)")
		.takes_value(true))
	.arg(Arg::with_name("verbose")
		.short("v")
		.long("verbose")
		.help("Prints the Request and the Response"))
	.arg(Arg::with_name("duration")
		.short("d")
		.long("duration")
		.help("Seconds you want your status to stay there. Overwrites expiration flag.")
		.takes_value(true))
	.get_matches();

    let mut profile_data = Profile {
        status_text: None,
        status_emoji: None,
        status_expiration: 0,
    };

    if let Some(text) = matches.value_of("status") {
        profile_data.status_text = Some(text.into());
    }

    let url_to_call = if let Some(url) = matches.value_of("api_url") {
        url
    } else {
        SLACK_URL
    };

    if let Some(text) = matches.value_of("emoji") {
        profile_data.status_emoji = Some(text.into());
    }

    profile_data.status_expiration = get_expiration(&matches, 0);

    let token = matches
        .value_of("token")
        .unwrap_or("--token=TOKEN is required");

    let profile_request = ProfileRequest {
        profile: profile_data,
    };

    let request = reqwest::Client::new()
        .post(url_to_call)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&profile_request);

    if matches.is_present("verbose") {
        println!("Request: {:?}", request);
        println!("Request body: {:?}", profile_request);
    }

    let result: Result<_, _> = request.send();
    if matches.is_present("verbose") {
        println!("Response: {:?}", result);
    }
}

fn get_expiration(matches: &ArgMatches, default: u32) -> u32 {
    let mut final_expiration: u32 = default;

    if let Some(text) = matches.value_of("expiration") {
        final_expiration = if let Ok(expiration_parsed) = text.parse::<u32>() {
            expiration_parsed
        } else {
            panic!(
                "expiration needs to be a number in UNIX timestamp, I couldn't parse this: {}",
                text
            );
        }
    }

    if let Some(text) = matches.value_of("duration") {
        final_expiration = if let Ok(duration_parsed) = text.parse::<i32>() {
            Utc::now()
                .checked_add_signed(Duration::seconds(i64::from(duration_parsed)))
                .unwrap()
                .timestamp() as u32
        } else {
            panic!(
                "duration needs to be a number, I couldn't parse this: {}",
                text
            );
        }
    };

    final_expiration
}
