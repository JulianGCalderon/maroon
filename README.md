A code block runner for markdown. Allows for documentation to stay up to date.

## Syntax

If you create fenced code block with `sh run` as info string, Maroon will print the output right after the first command. The command must be prepended by `>`.
````
```sh run
> echo "Hello World!"
Hello World!
```
````

## Usage

To learn about the Maroon, run
```sh run
> maroon --help
```

To execute a markdown file, run:
```sh
maroon README.md
```
It will output the updated file to stdout.

To update a markdown file inplace, run:
```sh
maroon -i README.md 
```

