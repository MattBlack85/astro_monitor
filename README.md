# Astro monitor
A small monitoring process that checks if kstars is active, if not it sends a request to notify a user via telegram that it crashed

# How to use this
Find the latest release [here](https://github.com/MattBlack85/astro_monitor/releases), grab the zip archive and unzip it on your filesystem.
At this point you should have already an api token, if not see [here](#obtain-an-api-token)

Run `./astromonitor FOO` where FOO is your API token


For a more user friendly list of commands try the following
```
mkdir astro_monitor
cd astro_monitor
wget https://github.com/MattBlack85/astro_monitor/releases/download/v0.1.0/astromonitor.zip
unzip astromonitor.zip
./astromonitor FOO
```

# Obtain an API token
On telegram, look for @AstroMonitorBot and issue a `/register` command, store the token in a safe place
