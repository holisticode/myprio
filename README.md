# myprio

`myprio` is a very simple task management system for the command line.

I know, not a very inspiring name, but I doubt it will ever make it into the arch linux repos :).
It is my personal project to learn rust.

Collaborations are very welcomed, but for the time being I assume this to be a personal only project.

## Basics
I find myself often postponing things with dates, and shuffling around. So `myprio` doesn't use due dates and such.
Instead, it uses a much more flexible, intuitive prioritization based on the [ Eisenhower method ](https://en.wikipedia.org/w/index.php?title=Eisenhower_method&redirect=no) priority matrix.

Just assign priorities for
* Urgent AND Important
* Urgent NOT Important
* Important Not Urgent

It still allows for lower ranked priorities too though:
* To Do :       Still has to be done, but lowest priority
* Watch :       Something which maybe hasn't been decided yet
* Nice To Do:   Yeah. 
* Some Day:     A loose reminder

These of course come from my very personal preferences and experiences.  I am using this tool myself now. 
Doesn't need to match anyone else's ideas...

## Backend
`myprio` uses a [Sqlite](https://www.sqlite.org/) backend, but could be extended to use anything really.
The default path for the db file is at `$HOME/mytasks.sql`. 
This should be made configurable via command line and it's half way there.

## Installation
Just use `cargo build` after cloning the repo to build the binary, and run the `myprio` binary.

