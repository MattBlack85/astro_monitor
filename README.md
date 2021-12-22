# Astro monitor
A small monitoring process that checks if kstars is active, if not it sends a request to notify a user via telegram that it crashed


# Install
you trust me? run the following command

```shell
wget -O - https://raw.githubusercontent.com/MattBlack85/astro_monitor/main/install.sh | sh
```

sudo will be needed as last step to move `astromonitor` to `/usr/bin`


# How to use astromonitor
Simply open a terminal and run `astromonitor XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX` where XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX is your API token


# Obtain an API token
On telegram, look for @AstroMonitorBot and issue a `/register` command, store the token in a safe place
