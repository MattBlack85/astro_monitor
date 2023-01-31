# Astro monitor
A small program that can help you with your astro session. 
The main functional it is the watchdog; if astromonitor notices Kstars is dead il will send a request to notify a user via telegram. 
The other cool feature is the backup, astromonitor can backup your INDI configuration for your devices and the equipment you use for your ekos profile; you can restore them on other PC or when installing the suite again. 


# Install
you trust me? run the following command

```shell
wget -O - https://raw.githubusercontent.com/MattBlack85/astro_monitor/main/install.sh | sh
```

sudo will be needed as last step to move `astromonitor` to `/usr/local/bin`


# How to use astromonitor
Given that your token is (just a random value here to give an example) `SecReTtOkEn` you should pass it to astromonitor this way `astromonitor SecReTtOkEn [COMMAND (like --kstars)]`

Open a terminal and run `astromonitor --help` to list the available commands

## Functionalities:
- `--kstars` monitor Kstars and send a telegram notification if it crashes during a session
- `--do-backup` make a backup of the Kstars database and INDI devices configuration and store it remotely
- `--retrieve-backup` restore the previously saved backup
- `--fd-monitor` monitor file descriptors used to check if there is any leak
- `--system-monitor` monitor system resources (CPU and RAM) usage and log it

The main folder where you'll find the logs is `~/.local/share/astromonitor/logs`

# Obtain an API token
On telegram, look for @AstroMonitorBot and issue a `/register` command, store the token in a safe place
