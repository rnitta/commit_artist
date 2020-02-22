# Commit Artist

This is a CLI tool to beautify latest commit of your git repository.

![DEMO](./doc/demo.png)

## Usage

### Install Commit Artist

```shell
$ cargo install git@github.com:rnitta/commit_artist.git
$ cd your_working_dir
$ commit_artist -p ccccccc
Yay! Now your new hash of the latest commit is cccccc89f1b198c120e43f5d939d9b5412236573.
```

### Command Line Options

- [--path] path to working directory. default: current directory
- [--pattern, -p] pattern to match. default: 0000000
- [--block, -b] log\[2\](how many hashes should calculated in each thread). default: 20
- [--jobs, -j] how many thread should spawn to bruteforce. default: your max - 1

## How it works
![architecture](./doc/architecture.png)

A commit hash of git is generated from commit object.  
Commit object consists of "tree hash", "parent hash", "author info", "committer info" and "commit message".  
Author info and committer info each have "name", "email address", "timestamp".  

One of the easiest thing to configure (even after the commit is done) among these attributes above is committer's name.  
Changing it may affect almost nothing but commit hash.  

So, after a commit is done, by running Commit Artist, through changing committer's name and calculating commit hash and loop back unless it is beautiful, finally you can get a commit which have sophisticated hash. 

## Disclaimer
Use this tool on your own responsibility.  
This tool is absolutely helpless with [signed commit](https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work).   