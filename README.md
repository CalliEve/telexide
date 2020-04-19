# Telexide

This is a WIP easy to use library for making a telegram bot, built on tokio and hyper.

## Features

- [X] easy to use and customisable client
- [X] long-polling based update handling
    - [X] set your own timeout
    - [X] set your own limit for updates gotten at once
- [X] easy to use, macro-based command framework
- [X] easy to use and heavily customisable api client
    - [X] use your own hyper client
    - [X] use your own api struct so you control the get and post methods
    - [X] includes all telegram api endpoints
- [ ] webhook based update handling
- [ ] clear documentation
- [ ] subscribe to non-message events using command (or similar) framework
    - [ ] run command on receiving an inline query or answer to one
    - [ ] run command on receiving a poll that matches your requirements

## Examples

For examples, please see the examples dir.

More documentation will follow...
