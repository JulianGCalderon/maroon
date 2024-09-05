A code block runner for markdown. Allows for documentation to stay up to date.

## Examples

If you create fenced code block with `sh run` as info string, Maroon will print the output right after the first command. The command must be prepended by `>`.
````
```sh run
> echo "Hello World!"
Hello World!
```
````
