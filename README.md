# rgb2hex

A simple script for converting rgba or rgb to hexidecimal from the cli because I got tired of looking it up.

If you install the binary from release, put in a place where you can use it like `~/.local/bin` or `/usr/local/bin/` (assuming you're using Linux).

Make it executable:
`chmod +x rgba2hex`

Example usage:
`rgba2hex 115 239 247 0.5`

Example output:
`#73EFF780`

Alpha can be omitted, the default alpha (full opacity) will be returned:

```
> rgba2hex 115 239 247
#73EFF7FF
```

Alpha can also be passed as a unsigned 8-bit integer:

```
> rgba2hex 115 239 247 32
#73EFF720
```
