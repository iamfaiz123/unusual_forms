## _Faizal@rapidinnovation.dev_
## _iamfaizalkhn@gmail.com_

# unusual froms



[![Build Status](https://travis-ci.org/joemccann/dillinger.svg?branch=master)](https://travis-ci.org/joemccann/dillinger)

unusual froms it a social site where user can post thread and comments
what makes it different form other question answers sites is that this dosent required Authentication
rather user Ip address will be stored in database and won't be visibile to other users.




## Features

- people can post multiple threads and each thread has its attributes like comments and likes
- each thread also has tags associated with it
- based on reports on a user we can blocked particular ip to access the web contents 
- threads can be seacrhed based on tags or name 

the agenda of the site is not limited to tech. question, it opens a vast variety of questions to be asked,
be tags of user choices

## Tech
this project relies on:

- [ACTIX-web] - Blazing fast backend framework build in RUST!
- [VS code] - Personal favourite
- [Rust] - Memory safe ,compile time error checker ,System level languge but i consider it as general purpose language.
- [Cargo] - Package manager for rust eco system 
- [MongoDB] - NOsql database which cut a lot of development time of schemeas and tables
- [Tokio] - Rust library for async functionality  
- [git] - for version controls


## Installation

Unusual forms requires [RUST](https://www.rust-lang.org/tools/install) v1.6+ to run.

Install the Rust compiler to start the server.

```scurl 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

For framework dependencies :
All rust program dependenices all installed with by cargo at compile time
user does not need to explicitly install it but need to tell cargo about it.
the information can be passed in Cargo.toml file



