# Recorder
This is the part of time tracking which has to do with the recording of time spend. There are many different backends
that all interact with the core through JSON. The backend interface is language and platform agnostic, so it could be implemented
in Rust or C++ for a computer tracker, web extension on the chrome webapp store, Swift for an apple tracker, etc. It doesn't matter
if it interacts through a web socket, native channel, or other method of communication between applications.

A recorder can be "underneath" another recorder. For example, a web extension would be taking detailed measurements on what a user
was doing on google chrome, while the recorder running on the desktop would just not that the web browser was open. The detailed
measurements supplement the general observation, and they can be blended together as well.

## Event
An event is something that happened. Here's a (tentative) JSON example:
```json
{
    "event": "applicationOpened",
    "recordId": "973c2f5b-d675-4db7-aa35-8a08bde90e62",
    "occuredAt": "2021-11-23T03:55:41.441Z",
    "device": "bee7e9c5-01e0-4120-a3c5-b8ad4c080fa1",
    "instance": "4255a06a-a035-4f42-bc03-462ee3bdf27f", // instance is separate from device, multiple instances allowed per device
    "uri": "http://google.com/chrome",
    "status": "focused", // a different measurement should be used for different status (music-playing, background, etc)
    "other": {
        "windowSize": [1000, 1000]
    }
}
```

## Device
Information on a device. When the tracker is first installed it will request device information. All fields are nullable
```json
{
    "type": "laptop",
    "os": "linux/ubuntu 20.04",
    "desktopServer": "X11",
    "architecture": "x86_64",
    "supportedFeatures": ["recording/windowSize", "response/popup", "response/lockScreen"]
}
```

The backend is not responsible for tagging/interpreting the results. It merely provides what was running and for what time.
For long sessions of using an application, the recorder will periodically update the "endedAt" time in order to accurately reflect
how long the person is on the device. 

The backend should keep its own internal record, at least for a time. Whenever the time-tracking application receives a new record,
it will provide the last time it received a record to make sure no events that were missed.
