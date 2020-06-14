<p align="center">
<img src="https://raw.githubusercontent.com/Eoin-McMahon/Blindfold/master/assets/banner.png" alt="banner" style="width:100%;height:20%;">
<br>
Logo courtesy of <a href="https://www.instagram.com/do.graphics/">Dominic Houston-Watt</a>
</p>
<!-- <h1 align="center"> Blindfold - a lightweight and simple .gitignore generator</h1> -->

[![Crates.io](https://img.shields.io/crates/v/blindfold?color=orange)](https://crates.io/crates/blindfold)
[![Crates.io](https://img.shields.io/crates/d/blindfold?color=d)](https://crates.io/crates/blindfold)
[![GitHub license](https://img.shields.io/github/license/Eoin-McMahon/Blindfold)](https://github.com/Eoin-McMahon/Blindfold/blob/master/license.txt)
[![GitHub stars](https://img.shields.io/github/stars/Eoin-McMahon/Blindfold)](https://github.com/Eoin-McMahon/Blindfold/stargazers)

## ✨ Features
* Pulls .gitignore templates from gitignore.io.
* Clean and simple CLI
* Suggestion system to help correct potential typos
* Allows for the combination of any number of different templates all into one gitignore

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
## 🔨 Demo:

![demo_video](https://raw.githubusercontent.com/Eoin-McMahon/Blindfold/master/assets/demo.gif)

## 🔧 Examples of use:
```bash
# generates a single gitignore file for both dart and flutter in ./src/.gitignore
blindfold --lang dart flutter
```

```bash
# you can specify a specific destination to store the gitignore file using the dest argument
blindfold --lang rust --dest ./src/
```

```bash
# arguments can also be written in shorthand
blindfold -l rust -d ./src/
```

```bash
# shows full list of available templates
blindfold list
```

```bash
# There is a help screen that can be shown which details the subcommands and arguments to supply to the program
blindfold -h
```
