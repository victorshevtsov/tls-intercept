# tls-intercept

This is a simple example showing the basic (yet) approach to intercept an HTTPS requests by capturing all the traffic flowing through a network interface.
The example uses:

- `pcap`
- `pnet`
- `tls-parser`

## Build

```rust
cargo build
```

## Run

```bash
sudo ./target/debug/tls-intercept
```

In a new terminal perform an `HTTPS` request with `curl`

```bash
curl -vi --http1.1 curl --tlsv1.2 --tls-max 1.2 https://example.com
```

## Next steps

As soon as we can capture an outgoing HTTPS requests, we need to decrypt it. The decryption process uses secrets that can be obtained thanks to SSLKEYLOGFILE feature. More info on that:
- [The SSLKEYLOGFILE Format for TLS](https://www.ietf.org/archive/id/draft-thomson-tls-keylogfile-00.html)
- [How to Decrypt SSL using Chrome or Firefox and Wireshark in Linux](https://knowledgebase.paloaltonetworks.com/KCSArticleDetail?id=kA14u000000wkvECAQ&lang=en_US%E2%80%A9)
- [Decrypt SSL traffic with the SSLKEYLOGFILE environment variable](https://my.f5.com/manage/s/article/K50557518)

As the articles above proves that it's possible to decrypt a captured HTTPS traffic, and the `tls-intercept` already captures the packets, the only thing left to do is to decrypt the captured packets the same way as Wireshark does.