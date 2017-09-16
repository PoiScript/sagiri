Sagiri
===

[WIP] Embedding kitsu.io services into Telegram.

<div style="text-align:center">
  <img alt="sagiri" src="assets/sagiri.png" width="512">
</div>

Start
---

Firstly, make sure that you have installed rustc and cargo.
If not, I recommend you using [rustup](https://rustup.rs).

Secondly, clone the full repositry and build:

```
$ git clone https://github.com/PoiScript/sagiri.git
$ cd sagiri
$ env TOKEN=BOT_TOKEN cargo build --release
```

Now, you can run sagiri using `env TOKEN=BOT_TOKEN cargo run --release`.
To run it automatically, use a simple systemd service:

```yml
# /etc/systemd/system/sagiri.service

[Unit]
Description=Embedding kitsu.io services into Telegram.
ConditionFileNotEmpty=/path/to/sagiri/Cargo.toml

[Service]
Environment=TOKEN=BOT_TOKEN
WorkingDirectory=/path/to/sagiri
# if you're using rustup, cargo should be in ~/.cargo/bin.
ExecStart=/path/to/cargo run --release

[Install]
WantedBy=multi-user.target # runlevel 3
```

```
$ systemctl daemon-reload
$ systemctl enable sagiri
$ systemctl start sagiri
$ systemctl status sagiri
```
