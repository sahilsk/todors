# About

A little cli to manage todo list stored in specified file.



# function requirements

Create new todolist

    $ todors list new "mytodofile"

Set the context under which following commands will work

    $ todors list set mytodofile

Now,you can run following commands to manage task in todo list named "mytodofilel"

    $ todors task add "finish task today"
    $ todors task done "finish task today"
    $ todors task rm "finish task today"
    $ todors task undon "finish task today"


