# Astro monitor
A small monitoring process that checks if kstars is active, if not it sends a request to notify a user via telegram that it crashed


# Install
you trust me? run the following command

```shell
wget -O - https://raw.githubusercontent.com/MattBlack85/astro_monitor/main/install.sh | sh
```

sudo will be needed as last step to move `astromonitor` to `/usr/local/bin`


# How to use astromonitor
Given that your token is (just a random value here to give an example) `SecReTtOkEn`

Simply open a terminal and run `astromonitor SecReTtOkEn`

## Additional functionalities:
- monitoring file descriptors used (to monitor file descriptors pass the flag `--fd-monitor`)
- monitoring system resources (CPU and RAM) usage (to monitor file descriptors pass the flag `--system-monitor`)

The main folder where you'll find the logs is `~/.local/share/astromonitor/logs`

# Obtain an API token
On telegram, look for @AstroMonitorBot and issue a `/register` command, store the token in a safe place
