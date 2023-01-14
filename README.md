<h1 style="text-align: center; font-size:72px">Rusty Checkers</h1>

> THIS IS IN PRODUCTION: currently not stable

<img style="text-align: center;" src="./.img/topreadmepicture.png" alt="image">


This is a simple checkers game board written in rust for the Unix terminal, but
may be ported to other terminals.

Name Pending


1. [Instillation](#install)
        1. [Build from sorce](#source)
                1. [Installing Rust](#getrust)
                2. [Building Project](#building)
2. [How to use](#use)


# How to install rusty checkers.<a name="install"></a>

> Currently since this is no whare near production this is not in any
> repository, of my knowledge, and you will have to build this from scratch.


## How to install rusty checkers.<a name="install"></a>

<h5>installing rust</h5><a name="getrust"></a>

To install this program you will have to have [Rust](https://www.rust-lang.org/) 
installed on your system.
I suggest that you install via [rustup](https://rustup.rs/)'s convenient one
liner:

        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

But you could just as easily download it from your distributions package manager
and or by installing cargo, I bet there are other ways.


<h5>installing rust</h5><a name="building"></a>

After the installation of rust, you can now compile this project.
I will be demonstrating with the cargo rust compiler, now to compile you will
need to run:

        cargo build --release

After which it it will save to target/release/rustycheckers.

Now you could stop here or you could go further by moving it to your user binary
file, so you do not have to navigate to this directory every time you want to
use it in the terminal unaware. To accomplish this you will need to run a
singular command, with the right privileges, to move or copy the binary to the
correct directory.

To move it:
        mv target/release/rustycheckers /usr/local/bin/rustycheckers

To copy it:
        cp target/release/rustycheckers /usr/local/bin/rustycheckers

And now you have it, now you can play checkers in your terminal with or without
you very real friends.


# How to use rusty checkers.<a name="use"></a>

To use rusty checkers you will first have to [install it](#install), and after
that you can learn the nuances of using this terminal application.

I am not going to teach you to play the game but if you do not know you can look
it up, or go [here](https://duckduckgo.com/) to find out.

So there is an indicator by the prompt to tell you whose turn it is, either
black or reds. On this board red is yellow so that it wont blend into the board
and you can actually play.

You can move you pieces by giving the continents of the piece you want to move
and where you want it to move to.

And if you want to quit just type

        quit


> Warning[3]:
>       (1): The white digits are the only pair of numbers that acutally work as
>       numbers
>       (2): You will have to translate a - h to 1 - 8 because alphanumerics are
>       currently no supported
>       {3}: Currently no way to kill another


# What To Implement.<a name="future">

1. Numbers Translate to Users
2. Accept alphanumerics as input
3. Ability to kill another piece



