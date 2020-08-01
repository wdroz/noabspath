# Say no to absolute paths in your codebases!

**noabspath** search and highlight all absolute paths detected.

<p align="center"><img src="https://raw.githubusercontent.com/wdroz/noabspath/master/img/demo.gif"/></p>

## Usage

<pre>
noabspath 0.1.6
William Droz <william.droz.ch@gmail.com>
check that there aren't obvious absolute paths in codebases

USAGE:
    noabspath <PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --ignore_file <IGNORE_FILE>    File that contains patterns to ignore (default .gitignore)

ARGS:
    <PATH>    path of codebase to check

</pre>

## Use case

The most common use case is to use **noabspath** in your CI pipeline.

