# process-monitor
A process and general information viewer made with Rust

![alt text](https://github.com/alk0x1/process-monitor/blob/master/terminal.png)


### Options
```bash
$ cargo run -- <options>

options:
  -m            # Don't show information about memory
  -s            # Don't show information about system
  -p <number>   # Number of processes to show (show 25 by default)

```

### To-do
- [ ] Put the process table inside a loop that update the changes in real time without change the general system info.
- [ ] Colorize red process that are consuming a lot of cpu or memory.
- [ ] Put a option to user prioritize wich process will appear on top, like the one who are consuming more cpu or more memory.
