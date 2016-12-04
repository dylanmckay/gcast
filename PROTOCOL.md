# CASTV2 protocol

The CASTV2 protocol is a TCP-based client/server message passing protocol.

## Prerequsites

Before being able to communicate with the CASTV2 protocol, you will need:

* A TCP networking library
* A [TLS](https://en.wikipedia.org/wiki/Transport_Layer_Security) library (i.e. [openssl](https://www.openssl.org))
* A [protocol buffers](https://developers.google.com/protocol-buffers/) compiler
* A JSON library

## *TODO*

* Talk about broadcasing with `*` as the `destination_id`

## Low-level

### TCP/IP

To communicate with a Cast device using the CASTV2 protocol, you first need
to open up a network connection on port `8009` to the Cast device.

### TLS

At the lowest level, just above TCP, the CASTV2 protocol is transmitted over
[TLS](https://en.wikipedia.org/wiki/Transport_Layer_Security).

The Cast device itself uses a self-signed certificate for verification.

You may run into problems if your TLS library automatically rejects self-signed
certificates. You must explicitly allow them in order to complete a successful
TLS handshake.

### Raw protocol

Just above SSL, data is transmitted in discrete packets named "messages".

A single message is sent with a 32-bit big endian unsigned integer representing
the size of the message, and then the raw message itself.

| Field       | Type           |
|-------------|----------------|
| Size        | 32-bit integer |
| Raw message | Raw bytes      |

## Mid level

Cast messages are described using the [protobuf](https://developers.google.com/protocol-buffers) library.

The full protobuf definition can be found at `extensions/common/api/cast_channel` in the Chromium
[source tree](https://chromium.googlesource.com/chromium/src.git/+/master/extensions/common/api/cast_channel).

The message definition looks like this

```protobuf
message CastMessage {
  enum ProtocolVersion {
    CASTV2_1_0 = 0;
  }

  required ProtocolVersion protocol_version = 1;

  required string source_id = 2;
  required string destination_id = 3;

  required string namespace = 4;

  enum PayloadType {
    STRING = 0;
    BINARY = 1;
  }

  required PayloadType payload_type = 5;

  optional string payload_utf8 = 6;
  optional bytes payload_binary = 7;
}
```

### Protocol version

Every message is required to specify the version of the CAST protocol it uses.

### Source/destination IDs

These are textual IDs used to identify the endpoints that the messages are being transmitted to/from.

Google Chrome uses `sender-0` to identify itself and it uses `receiver-0` to identify the Cast device.

These are not the only possible values.

When `PING` messages are sent from the Cast device to the client, the messages have `Tr@n$p0rt` as
the source and destination.

### Namespaces

A namespace can be thought of as a "channel name" through which a message can be sent.

For example, the initial `CONNECT` message is sent on the `urn:x-cast:com.google.cast.tp.connection` namespace.

* `urn:x-cast:com.google.cast.tp.connection`
* `urn:x-cast:com.google.cast.tp.heartbeat`
* `urn:x-cast:com.google.cast.receiver`
* `urn:x-cast:com.google.cast.tp.deviceauth`

#### `urn:x-cast:com.google.cast.tp.connection`

This is used to send `CONNECT` and `CLOSE` message to manage the virtual connection between the
client and the Cast device.

#### `urn:x-cast:com.google.cast.tp.heartbeat`

This is used to send `PING` and `PONG` heartbeat events.

#### `urn:x-cast:com.google.cast.receiver`

This is used to manage and query the Cast receiver itself. Most of the interesting messages
go through this namespace.

#### `urn:x-cast:com.google.cast.tp.deviceauth`

This is used for authentication of the Cast device. Authentication only occurs if the
client initiates it.

### Payloads

There are two types of messages - `STRING` and `BINARY`.

#### `STRING` payloads

`STRING` payloads contain JSON data. For all textual messages, it looks like the following
template.

```json
{
    "type": "<MESSAGE_TYPE>",
    "<option 1>": "<value 1>",
    "<option 2>": "<value 2>"
    ...
}
```

#### `BINARY` payloads

*TODO*

## High level

This describes the highest level of the protocol - the format of the different message types.

### Messages

#### Summary


| Message                   | Example payload                                                              |
|---------------------------|------------------------------------------------------------------------------|
| `CONNECT`                 | `{ "type": "CONNECT" }`                                                      |
| `CLOSE`                   | `{ "type": "CLOSE" }`                                                        |
| `PING`                    | `{ "type": "PING" }`                                                         |
| `PONG`                    | `{ "type": "PONG" }`                                                         |
| `GET_STATUS`              | `{ "type": "GET_STATUS" }`                                                   |
| `RECEIVER_STATUS`         | `{ "type": "RECEIVER_STATUS", "requestId: 31432", "status": { ... } }`       |
| `LAUNCH`                  | `{ "type": "LAUNCH", "appId": "YouTube" }`                                   |
| `STOP`                    | `{ "type": "STOP", "sessionId": "f2f6a2c3-2c92-4c43-9fb2-ca0b2872a75d" }`    |


#### `CONNECT` (Client -> Cast device)

This is a textual message with no extra data fields. It always looks like this:

```json
{ "type": "CONNECT" }
```

It is always transmitted on the `urn:x-cast:com.google.cast.tp.connection` namespace.

#### `CLOSE` (Bidirectional)

This is a textual message with no extra data fields. It always looks like this:

```json
{ "type": "CLOSE" }
```

This can be used by either the client or the Cast device to close the virtual connection.

It is always transmitted on the `urn:x-cast:com.google.cast.tp.connection` namespace.

#### `PING` (Bidirectional)

This is a textual message with no extra data fields.

A ping can be send from either the client to the server and vice versa. If the client does not
send pings to the Cast device automatically, the server will send pings to the client.

If the client doesn't send a `PING` or `PONG` to the server for more than a few seconds, the
Cast device will immediately drop the connection (without even sending a `CLOSE` message).

It always looks like this:

```json
{ "type": "PING" }
```

It is always transmitted on the `urn:x-cast:com.google.cast.tp.heartbeat` namespace.

#### `PONG` (Bidirectional)

This is a textual message with no extra data fields.

A pong message is used to respond to a `PING` to let the other end know that the connection
is still alive (hence the namespace `heartbeat`).

It always looks like this:

```json
{ "type": "PONG" }
```

It is always transmitted on the `urn:x-cast:com.google.cast.tp.heartbeat` namespace.

#### `GET_STATUS` (Client -> Cast device)

This is a textual message with no extra data fields.

The `GET_STATUS` message is sent from the client to the Cast device to query the
current status (volume, running apps, etc).

The Cast device always responds with a `RECEIVER_STATUS` message.

It is always transmitted on the `urn:x-cast:com.google.cast.receiver` namespace.

```json
{ "type": "GET_STATUS" }
```

#### `RECEIVER_STATUS` (Cast device -> Client)

This is a textual message.

The `RECEIVER_STATUS` message is sent from the Cast device to the client to notify
it about a status change (or when it is explicitly requested via the `GET_STATUS` message).

The data inside the payload varies depending on the state of the Cast device. The volume status
is always present.

It is always transmitted on the `urn:x-cast:com.google.cast.receiver` namespace.

##### Volume

`RECEIVER_STATUS` always contains the current status of the volume.

Here is an example message when there are no running applications:

```json
{
    "requestId": 0,
    "status": {
        "volume": {
            "controlType": "attenuation",
            "level": 1.0,
            "muted": false,
            "stepInterval": 0.05000000074505806
        }
    },
    "type": "RECEIVER_STATUS"
}
```

###### Control types

Here is a list of known `controlType` values for the volume.

* `attenuation`

###### Level

The level is a floating-point value between `0.0` and `1.0` representing the volume.

A value of `1.0` represents the maximum volume, whereas `0.5` represents half-volume and
`0.0` represents no volume.

###### Applications

When there is at least one running application, an extra array-valued entry is present - `applications`:

```json
{
    "requestId": 0,
    "status": {
        "applications": [{
            "appId": "YouTube",
            "displayName": "YouTube",
            "isIdleScreen": false,
            "sessionId": "164454a7-bc83-4013-9d6d-fb2e9d1c7a7a",
            "statusText": "YouTube TV"
        }],
        "volume": {
            "controlType": "attenuation",
            "level": 1.0,
            "muted": false,
            "stepInterval": 0.05000000074505806
        }
    },
    "type": "RECEIVER_STATUS"
}
```

#### `LAUNCH`

This is a textual message with one data field: `appId`.

```json
{ "type": "LAUNCH", "appId": "YouTube" }
```

Upon sending a `LAUNCH` message, the Cast device will send `RECEIVER_STATUS` updates which
will have details about the application session once it has launched.

It is always transmitted on the `urn:x-cast:com.google.cast.receiver` namespace.

#### `STOP`

This is a textual message with one data field: `sessionId`.

This message will stop an application running with the given session identifier.

A list of running applications and their session ids can be obtained from reading
a `RECEIVER_STATUS` message from the Cast device.

```json
{
    "type": "STOP",
    "sessionId": "f2f6a2c3-2c92-4c43-9fb2-ca0b2872a75d"
}
```

It is always transmitted on the `urn:x-cast:com.google.cast.receiver` namespace.

# Discovery

There are two different methods to find Cast devices on a network - `mDNS` and `DIAL`.

## mDNS

This uses the standard [multicast DNS](https://en.wikipedia.org/wiki/Multicast_DNS) to send messages
out to all devices on the network and search for a specific service.

In order to find Cast devices, it is sufficient to multicast a DNS query for `_googlecast._tcp.local`.

All Cast devices will respond with `A` records indicating their IP addresses.

## DIAL

Not documented.
