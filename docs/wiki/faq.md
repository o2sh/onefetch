# FAQ

## Why isn’t my language detected?

Onefetch relies on language detection rules from the Linguist ecosystem and the language definitions supported by onefetch. If a language is not detected, it is usually because the repository contents do not match those rules.

Common reasons include:

- The file is very small and does not contain enough information to identify the language.
- The file uses an uncommon or unsupported extension.
- The file is generated, vendored, binary, ignored, or otherwise excluded from analysis.
- The language is not currently supported by Onefetch.

If your language is not shown, first check whether it is supported by Onefetch. If it is not supported yet, open a new language request so support can be added.
