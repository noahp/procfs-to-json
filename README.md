# procfs to JSON

Dump your [procfs](https://docs.kernel.org/filesystems/proc.html) to JSON!

## Usage

```bash
cargo install procfs-to-json
procfs-to-json | jq '.[0]'
{
  "pid": 1,
  "comm": "systemd",
  "exe": null,
  "cwd": null,
  "status": "S (sleeping)",
  "memory": 24260608
}
```
