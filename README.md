# email_to_webhook

put it in `/root/.forward`:

```
|"/usr/bin/envdir /etc/environment.d /usr/local/bin/email_to_webhook"
```

Then:

```
mkdir -p /etc/environment.d
echo "https://hooks.slack.com/services/blah/blah/secretblah" > /etc/environment.d/SLACK_WEBHOOK_URL
```

## Adapted from https://git.eeqj.de/sneak/hacks/src/branch/master/email-to-webhook to use Rust instead of Python for easier dependency management because I'm too dumb to install Python on Nixos
