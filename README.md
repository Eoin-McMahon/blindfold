<p align="center">
<img src="https://raw.githubusercontent.com/Eoin-McMahon/Blindfold/master/assets/banner.png" alt="banner" style="width:100%;height:20%;">
<br>
Logo courtesy of <a href="https://www.instagram.com/do.graphics/">Dominic Houston-Watt</a>
</p>
<!-- <h1 align="center"> Blindfold - a lightweight and simple .gitignore generator</h1> -->

[![Build](https://github.com/Eoin-McMahon/blindfold/workflows/Build/badge.svg)](https://github.com/Eoin-McMahon/blindfold/actions?query=workflow%3ABuild)
[![Crates.io](https://img.shields.io/crates/d/blindfold?color=d)](https://crates.io/crates/blindfold)
[![GitHub license](https://img.shields.io/github/license/Eoin-McMahon/Blindfold)](https://github.com/Eoin-McMahon/Blindfold/blob/master/license.txt)
[![GitHub stars](https://img.shields.io/github/stars/Eoin-McMahon/Blindfold)](https://github.com/Eoin-McMahon/Blindfold/stargazers)

## ✨ Features
* Pulls .gitignore templates from gitignore.io.
* Clean and simple CLI
* Allows for the combination of any number of different templates all into one gitignore
* Allows for appending to pre-existing gitignore templates so that custom directories are not overridden.

## 📦 Installation
NOTE: Rust must be installed on your system for this to work. (<a href="https://www.rust-lang.org/learn/get-started">Install Rust</a>)

#### 📥 Download from crates.io

```bash
cargo install blindfold
```

#### 🏗️ Build from source
```bash
git clone https://github.com/Eoin-McMahon/blindfold.git
cd blindfold
cargo install --path ./
```

This will install the binary and add it to your path. Once installed you can use the tool as shown in the examples below.

## 🔧 Examples of use:
```bash
# generates a single gitignore file for both dart and flutter in ./src/.gitignore
blindfold generate dart flutter -d ./src/
```

```bash
# use the append flag to add to the pre-existing gitignore file (can be shortened to -a)
blindfold generate macos --append
```

```bash
# you can specify a specific directory to store the gitignore file in using the dir argument
blindfold generate rust --dir ./src/
```

```bash
# arguments can also be written in shorthand
blindfold generate rust -d ./src/ -a 
```

```bash
# shows full table list of available templates
blindfold list
```

```bash
# There is a help screen that can be shown which details the subcommands and arguments to supply to the program
blindfold -h
```
