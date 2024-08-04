# Emmapack

**E**asily<br>
**m**anagable<br>
**m**ajestically<br>
**a**bstracted<br>
**pack**ets

## Goal

I wanted to make a rust library that was easy to use in any project, to simply communicate between rust apps, be it through tcp sockets, maybe encrypted with ssl. Basically in whatever way I would ever want to send data, I wanted to be able to.

## Overview

Although this is a very simple project, it went through about 5 iterations of it's implementation, but I soon realized that using different wrapper objects is extremely cumbersome and not really viable without constant maintenance, I just wanted to be able to connect and send data without having to serialize packets manually.

So I ended up just making traits for sending and reading packets, as well as serializing and deserializing packets. If you want to manually do the serialization you can. If not, you can derive serde `Serializable` and `Deserializable` and an implementation will be provided. Similarly, as long as your type implements `Read`/`Write`, or `AsyncRead`/`AsyncWrite`, you will be able to use `.send_packet`&`.read_packet` or `.send_packet_sync`&`.read_packet_sync` respectively.

Remember that you can use a giant enum with all of your packet types, this will make your life very easy, since you don't have to deal with manually specifying what to deserialize into, and is also much safer.

## Features

- tokio

Note: The tokio feature is enabled by default, exclude it if you do not want to ship tokio beacuse of it's runtime, not because you want to use a different async runtime, `AsyncRead` and `AsyncWrite` traits come from tokio, it is currently impossible to use a different runtime.

## Examples

Right now there's only one example in `/examples/std_tcp_server.rs`, but it outlines pretty well how to use the library.

## Breaking changes

I do not intend to support any version mismatch between protocol versions, however, if you want to check it manually, you can use `emmapack::CURENT_PROTOCOL_VERSION`, any non-breaking changes won't increment this value, so checking it is a good idea.
