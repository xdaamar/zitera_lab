# Core Concepts of Supply Chain Attacks

## Attack Vectors in the Supply Chain

1. **Dependency Confusion**: Attackers register a package on a public repository (e.g. npm or PyPI) with the same name as an internal private corporate package, exploiting package manager search order.
2. **Typosquatting**: Creating packages with slightly misspelled names (e.g. `lodsh` instead of `lodash`) hoping developers mistakenly install them.
3. **Account Hijacking & Maintainer Coercion**: Gaining unauthorized access to legitimate maintainer accounts to push backdoored updates.
4. **Poisoned Build Scripts**: Using install hooks (such as `postinstall` in npm or `setup.py` scripts in Python) to execute arbitrary shell code during `npm install` or `pip install`.
5. **Unpinned Versions & Floating Dependencies**: Using version ranges like `^1.0.0` or `latest` which pull in newly poisoned versions automatically during CI/CD builds.
