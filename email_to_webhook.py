#!/usr/bin/env python3


from email import message_from_file

import json
import os
import requests
import sys
import syslog

hook_url = os.environ.get('SLACK_WEBHOOK_URL')

def main():
    msg = message_from_file(sys.stdin)
    try:
        send_to_slack(dict(msg)['From'],dict(msg)['Subject'],msg.get_payload())
    except(Exception):
        syslog.syslog(syslog.LOG_ERR,"webhook failed, message was %s" % msg.get_payload())


def send_to_slack(fr,title,body):
    title = title.strip()
    body = body.strip()
    fr = fr.strip()

    slack_data = {
        'text': "Email from " + fr,
        'attachments': [
            {
                'title': title,
                'text': body,
                'fallback': body
            }
        ]
    }

    response = requests.post(
            hook_url,
            data=json.dumps(slack_data),
            headers={'Content-Type': 'application/json'}
    )

    if response.status_code != 200:
            syslog.syslog(
                    syslog.LOG_ERR, "Couldn't send webhook to slack: resp %s %s" % (response.status_code, response.text)
            )
            raise ValueError(
                    'Request to slack returned an error %s, the response is:\n%s'
                    % (response.status_code, response.text)
            )


if __name__ == "__main__":
    main()


