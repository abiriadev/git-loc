<div align="center">

# `git-loc`

$\mathcal O(n)$ source lines over time analyzer for git

</div>

## Example

The number of lines of source code in the [`pocketbase/pocketbase`](https://github.com/pocketbase/pocketbase) repository over time:

```
 180200 ┤                                       ╭─╮                                      
 175808 ┤                                      ╭╯ │                                      
 171417 ┤                                     ╭╯  ╰───╮                                  
 167025 ┤                                     │       │                                  
 162634 ┤                                     │       │                                  
 158242 ┤                                     │       │                                  
 153850 ┤                                    ╭╯       │                       ╭───────── 
 149459 ┤                           ╭╮    ╭──╯        │           ╭───────────╯          
 145067 ┤                        ╭──╯│╭───╯           │        ╭──╯                      
 140676 ┤                     ╭──╯   ╰╯               │ ╭──────╯                         
 136284 ┤                     │                       ╰─╯                                
 131892 ┤                    ╭╯                                                          
 127501 ┤                    │                                                           
 123109 ┤                    │                                                           
 118718 ┤                 ╭──╯                                                           
 114326 ┤               ╭─╯                                                              
 109934 ┤            ╭──╯                                                                
 105543 ┤            │                                                                   
 101151 ┤    ╭───────╯                                                                   
  96760 ┤   ╭╯                                                                           
  92368 ┼───╯                                                                           
           LOC over time
```

## Usage

```sh
$ git loc

Count lines of code over time through Git history

Usage: git-loc [OPTIONS] [REPOSITORY]

Arguments:
  [REPOSITORY]
          Sets the path to the repository
          
          [default: .]

Options:
  -i, --ignore <IGNORE>
          Filenames to ignore from statistics

  -I, --ignore-file <IGNORE_FILE>
          Path to a file that lists filenames to ignore

  -f, --format <FORMAT>
          Output Format
          
          [default: chart]

          Possible values:
          - chart:  Show ascii graph on terminal
          - ndjson: Output each data point as ndjson

      --width <WIDTH>
          Width of the chart

      --height <HEIGHT>
          Height of the chart

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
