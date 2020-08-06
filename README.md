# do-capture

Simple tool and library to do a screen capture of a web page. Uses firefox controlled with geckodriver and the marionette protocol to make the screen capture.

## As library
```rust
let session = Session::new().await?;
let capture: Vec<u8> = session.capture("http://duck.com".parse().unwrap(), size::PHONE).await?;
```

## As cli tool
```bash
capture http://duck.com > cap.png
```

