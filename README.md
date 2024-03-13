## FileFox

A CLI tool written in Rust for editing large amounts of files quickly. 

It currently supports the following commands:

| Command | Action |
|----|----|
| ```filefox group <regex>``` | - Groups all files or directories located within the src directory (default './') that contain the given regex. |
| ```filefox cut <src>``` | - Rips the files out of the directory and then deletes it (ungroups the files). |
