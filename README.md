# git-credential-pass
A Simple [git credential helper](https://git-scm.com/docs/gitcredentials) for [gnu pass](https://www.passwordstore.org/)
If the templating language is not powerful enough and you only want to read passwords from pass look at (pass-git-helper)[https://github.com/languitar/pass-git-helper]

## Features
* Can store, get and erase passwords from pass
* Mostly uses .gitconfig for configuration
* Very simple template language

## Installation

### Archlinux

Install git-credential-pass from aur

### Debian/Ubuntu

Download latest release and use ```dpkg -i <FILE>``` to install it

## Configuring

~/.config/git-credential-pass/git.ldev.eu.org.template

```
{password}
login: {username}
```

.gitconfig

```toml
[credential]
  helper = pass --pass-name "git/{protocol}/{host}" --template "~/.config/git-credential-pass/{host}.template"
```
> **NOTE**
> Text between {} gets replaced by the value returned by git. See [custom_helpers](https://git-scm.com/docs/gitcredentials#_custom_helpers).
> **NOTE**
> Use \\ to escape characters \\{ will be treated as a literal


### More examples

#### Store passwords as git/{host}/{username}
.gitconfig
```toml
[credentials]
    helper = pass -p "git/{host}/{username}" --template "~/.config/git-credential-pass/{host}.template"
```

#### Use only for a specific host
.gitconfig

```toml
[credentials "https://git.ldev.eu.org"] # only use git-credential-pass for git.ldev.eu.org
	useHttpPath = true
    helper = pass -p "git/ldev" --template "~/.config/git-credential-pass/git.ldev.eu.org.template"
[credentials] # use cache for everything else
    helper=cache
```

#### Store credentials using url
.gitconfig

```toml
[credentials]
	useHttpPath = true
    # Store the credentials using the url path
    helper = pass -p "git/{path}" --template "~/.config/git-credential-pass/template.template"
```
