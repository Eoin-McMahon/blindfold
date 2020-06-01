<h1 align="center"> Blindfold - a lightweight and simple .gitignore generator</h1>
<p align="center">
<img height="325" width="333" src="./blindfolded_github.png">
<br>
Logo courtesy of [Dominic Houston-Watt](https://www.instagram.com/do.graphics/)
</p>

This project makes use of gitignore.io and provides a cli to generate gitignore templates for any language of your chosing!

#### Examples:
```bash
# generates a single gitignore file for both dart and flutter in ./src/.gitignore
$ blindfold --lang dart flutter --dest ./src/
```

```bash
# shows full list of available templates
$ blindfold list
```

#### Installation
* Rust is a prerequisite
* clone the repository
* run cargo build
* executable stored in blindfold/target/debug/blindfold
