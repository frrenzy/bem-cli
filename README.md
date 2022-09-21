# bem-cli

Generates basic nested BEM file structure and some boilerplate code

### Pre-requisites

```sh
curl
wget
zsh
```

### Installation

```sh
zsh -c "$(curl -fsSL https://raw.github.com/frrenzy/bem-cli/master/install.sh)"
```

### How to use

- `bem create` - scaffolds BEM project inside current directory
- `bem create [-i, --install]` - scaffolds BEM project inside current directory AND installs dependencies automatically
- `bem version` - shows current version
- `bem update` - checks for updates
- `bem component <name>` - generates React component with specified name in current/src/components directory. CSS-module included
- `bem component <name> [--sass, --scss]` - generates React component with specified name in current/src/components directory. CSS-module will use specified preprocessor
