# About

A little cli to manage todo list stored in specified file.



# function requirements


Config init: Generate config file

    $ todors config init

Create new todolist

    $ todors new <mylist>

List all files

    $ todors ls --files

Set the context under which following commands will work

    $ todors use <mylist>

Now,you can run following commands to manage task in todo list named "mytodofilel"

    $ todors add "<id_or_partial_desc>"
    $ todors done/undone  "<id_or_partial_desc>"
    $ todors rm "<id_or_partial_desc>"

List all tasks:

    $ todors ls 


# Todo structure 

# 

todo init <list-name>
    - it should create list in cfgr dir
    - list: metadata
    

Todo list Strucutre

- 
```
Name:
Created_at:
Modified_at:
---
Task List
---
1. <added_at> || task description
2.


