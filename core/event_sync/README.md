# Event sync
This is to sync between a js instance and other programming language. It's a relatively simple protocol, mainly to prevent
events from being lost from a failed notification.

It's based on a polling approach, except it will notify the JS instance when there's a new event, which then the JS instance requests
all notifications since `X` id. The JS instance will then verify which it received. That way the non-js instance can safely delete
the events in the queue.