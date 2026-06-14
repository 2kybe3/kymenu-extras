**kymenu-ssh** - Parses system and user SSH config files and provides a searchable host list for kymenu.

---

### Features

* Extracts and displays SSH host aliases
* Optional display of `username` and `hostname`
* Filter hosts using regular expressions
* Configurable inclusion of system/user SSH config sources

---

### Usage

```bash
# Basic usage
kymenu-ssh | kymenu --json-in

# Don't show username + hostname
kymenu-ssh --username=false --hostname=false | kymenu --json-in

# Also include system SSH config
kymenu-ssh --system-config=true | kymenu --json-in

# Filter hosts with regex
kymenu-ssh --regex "prod-.*" | kymenu --json-in

# Select and SSH into host
ssh "$(kymenu-ssh | kymenu --json-in)"
```

