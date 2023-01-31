# myfind

### like linux `find`, but faster and simpler

wanted to try to make a cli with rust, so i "remade" find 

### installation

* with cargo: `cargo install myfind'

### usage

* `myfind [OPTIONS] <path>`, path is the root path you would like to search within

##### options

* `-e, --extension <EXT>`: search for files of a specific extension
* `--empty`: search for empty directories
* `-f, --file <FILE>`: file or directory to search for
* `-V, --version`: show version info

### performance vs `find`

![](img/benchmark1.png)
![](img/benchmark2.png)
