<p align="center">
<img height="380" width="1920" src="https://github.com/Eoin-McMahon/Blindfold/blob/master/assets/banner.png">
<br>
Logo courtesy of <a href="https://www.instagram.com/do.graphics/">Dominic Houston-Watt</a>
</p>
<!-- <h1 align="center"> Blindfold - a lightweight and simple .gitignore generator</h1> -->

## ✨ Features
* Pulls .gitignore templates from gitignore.io.
* Clean and simple CLI
* Suggestion system to help correct potential typos
* Allows for the combination of any number of different templates all into one gitignore

## 📦 Installation
NOTE: Rust must be installed on your system for this to work.
It can be installed easily by running the following command:
```console
foo@bar:~$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Download from crates.io

```bash
cargo install bliss
```

### Build from source
* Clone the repository and cd into it
* Once in the top level directory of the repo run the command:
```console
foo@bar:~$ cargo install --path ./
```

This will install the binary and add it to your path. Once installed you can use the tool as shown in the examples below.

## ⚙️ Demo:

![demo_video](https://github.com/Eoin-McMahon/Blindfold/blob/master/assets/demo.gif)

## 🔧 Examples of use:
```console
// generates a single gitignore file for both dart and flutter in ./src/.gitignore
foo@bar:~$ blindfold --lang dart flutter
```

```console
// you can specify a speciic destination to store the gitignore file using the dest argument
foo@bar:~$ blindfold --lang rust --dest ./src/
```

```console
// arguments can also be written in shorthand
foo@bar:~$ blindfold -l rust -d ./src/
```

```console
// shows full list of available templates
foo@bar:~$ blindfold list
```

```console
// There is a help screen that can be shown which details the subcommands and arguments to supply to the program
foo@bar:~$ blindfold -h
```
