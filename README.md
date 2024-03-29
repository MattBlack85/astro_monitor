# Astro monitor
<p align="center">
  <img src="https://github.com/MattBlack85/astro_monitor/assets/4163222/5798898a-2569-49e3-b60c-f783b9134bf6" alt="Your image title" width="200"/>
</p>


A small program that can help you with your astro session. 
The main functionality is the watchdog; if astromonitor notices Kstars is dead il will send a request to notify you via telegram. 
The other cool feature is the backup, astromonitor can backup:
- your INDI configuration for your devices
- the profile containing your equipment and setup
- PHD2 profile
- generic Kstars option (theme, colors, etc.)

  
you can restore the backup on an other PC or when installing the suite again.

# Have a look at how it works
https://github.com/MattBlack85/astro_monitor/assets/4163222/587784c4-6a9b-4485-9444-9d0d61d9897e

# Install
you trust me? run the following command

```shell
wget -O - https://raw.githubusercontent.com/MattBlack85/astro_monitor/main/install.sh | sh
```

sudo will be needed as last step to move `astromonitor` to `/usr/local/bin`


# How to use astromonitor
Using astromonitor is easy, just follow these instruction

## First step - obtain a token using telegram
On telegram, look for @AstroMonitorBot (the icon is the bubble nebula from hubble) and send a `/register` command, it will answer with a token, store it in a safe place, that's your key for the backups.

## Make a backup
Open a terminal and run `astromonitor --do-backup XXXXXXXXXXXXXXXXXX` paste your key obtained in the previous step instead of XXXXXXXXXX

## Retrieve a previously made backup
Open a terminal and run `astromonitor --retrieve-backup XXXXXXXXXXXXXXXXXX` paste your key obtained in the previous step instead of XXXXXXXXXX

## Monitor Kstars during a session
Open a terminal and run `astromonitor --kstars XXXXXXXXXXXXXXXXXX` paste your key obtained in the previous step instead of XXXXXXXXXX

This will start monitoring Kstars and if it crashes, it will send you a notification via telegram

# List of all functionalities:
you can obtain the same list by running `astromonitor --help`, here what we actually support:

- `--kstars` monitor Kstars and send a telegram notification if it crashes during a session
- `--do-backup` make a backup of the Kstars database and INDI devices configuration and store it remotely
- `--retrieve-backup` restore the previously saved backup
- `--fd-monitor` monitor file descriptors used to check if there is any leak
- `--system-monitor` monitor system resources (CPU and RAM) usage and log it

The main folder where you'll find the logs is `~/.local/share/astromonitor/logs`

# Compile it and run
If you want to compile it by yourself, the project is pure `Rust`, you just need the rust toolchain (see https://rustup.rs/), then clone this repo with `git clone https://github.com/MattBlack85/astro_monitor` or `git clone git@github.com:MattBlack85/astro_monitor` cd into the folder `cd astro_monitor` and then `cargo build --release`.
You will find the compiled program then under `target/release/astromonitor` you can move the program around (for example /usr/local/bin)
