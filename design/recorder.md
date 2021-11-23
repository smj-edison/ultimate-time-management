# Recorder
This is the part of time tracking which has to do with the recording of time spend. There are many different backends
that all interact with the core through JSON. The backend interface is language and platform agnostic, so it could be implemented
in Rust or C++ for a computer tracker, web extension on the chrome webapp store, Swift for an apple tracker, etc. It doesn't matter
if it interacts through a web socket, native channel, or other method of communication between applications.

## Activity
An activity is time spent doing something, on something, for some amount of time. Here's a (tentative) JSON example:
```json
{
    "recordId": "973c2f5b-d675-4db7-aa35-8a08bde90e62",
    "startedAt": "2021-11-23T03:55:41.441Z",
    "endedAt": "2021-11-23T03:55:47.278Z",
    "device": "bee7e9c5-01e0-4120-a3c5-b8ad4c080fa1", // will have info on device type and such
    "uri": "http://google.com/chrome"
}
```

The backend is not responsible for tagging/interpreting the results. It merely provides what was running and for what time.
For long sessions of using an application, the recorder will periodically update the "endedAt" time in order to accurately reflect
how long the person is on the device. 

The backend should keep its own internal record, at least for a time. Whenever the time-tracking application receives a new record,
it will provide the last time it received a record to make sure no events that were missed.