dumb_logger is a minimal logging module.

It allows you to use the `log` macros `error!`, `warn!`,
`info!`, `debug!` and `trace!` to print to `stdout`.

It has no dependencies other than `log`.  It doesn't print timestamps or
sequence numbers or print in color.  It just prints the message that was
logged.

How to use it:
```
use log::trace;
dumb_logger::init();
trace!("hello world");
```
