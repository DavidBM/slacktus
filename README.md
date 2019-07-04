# Slacktus

This is an small utility for updating your slack status. I wrote it in Rust because it is multiplatform, fast and dependency free. 

I use it with my pomodoro app. I may publish it as a crate in the future after some time using it.

## Installation

After git pull and cd into the directory:

```
cargo install --path .
```

## How to use

TL;DR. As an example: 

```bash
slacktus -t xoxp-00000000000-00000000000-00000000000-00000000000000000000000000000000 -e :tomato: -s 'Focusing for 25 min, I will respond in a bit' -d 1
```


After it is installed, execute `slacktus --help` and it will show all the options.

```
Slacktus - Change your slack status with one command. 1.0
David Bonet <dbmontes@gmail.com>
It updates your slack status in one command. Perfect for pomodoro apps and similar.

USAGE:
    slacktus [FLAGS] [OPTIONS] --token <token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Prints the Request and the Response

OPTIONS:
        --api-url <api_url>          The Slack API url. By default is https://slack.com/api/users.profile.set
    -d, --duration <duration>        Seconds you want your status to stay there. Overwrites expiration flag.
    -e, --emoji <emoji>              The emoji you want in your slack. Use the text of one of the slack's emojis.
    -x, --expiration <expiration>    When is it going to expire? Pass a date in UNIX timestamp format. 0 by default (won't expire)
    -s, --status <status>            The status text you want in your slack.
    -t, --token <token>              The Slack API token used for the API call. You can get your token here:
                                     https://api.slack.com/custom-integrations/legacy-tokens
```

## Contributing

The project is veeery simple. Additions are welcome. Any change or improvement, welcome too. Just let me know the use case. If I have time I would love to updated and extend it too. 
