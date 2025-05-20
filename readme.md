# Fake Progress Bar

A silly little rust project based heavily on [Indicatif](https://docs.rs/indicatif/latest/indicatif/) and [CLAP](https://docs.rs/clap/latest/clap/), inspired by a now-deleted Tumblr post.

## Usage

```
fake-progress.exe [OPTIONS] -s <STEP_LENGTH>

Options:
  -s <STEP_LENGTH>         How long each message (step) is in milliseconds
  -m <MESSAGES>...         A list of messages to display next to the bar
  -i <INITAL_MESSAGE>      An optional initial message to be printed before the bar
  -f <FINAL_MESSAGE>       An optional final message to render after the bar is finished
  -a <AFTER_MESSAGE>       An optional message to be printed one line after the bar
  -h, --help               Print help
  -V, --version            Print version
```

## Example

```
fake-progress.exe -s 100 \
    -i "Transing your gender... Do not power off." \
    -f "Complete." \
    -a "Gender transed. Have a nice day :)" \
    -m  "Changing name..." \
        "Erasing Deadname..." \
        "Assigning pronouns..." \
        "Submitting forms..." \
        "Blocking TERFs..." \
        "Performing top surgery..." \
        "Performing bottom surgery..."
```

![Example output](example.gif "Example output")
