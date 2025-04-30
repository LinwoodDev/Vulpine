---
title: Introduction
---

> Welcome to Vulpine, a user friendly gui for command line tools

## Syntax

We use jinja syntax for templating the process running and json for the results. The syntax is similar to the following:

```jinja
[
  {
    "executable": "echo",
    "args": [
      "Hello {{ name }}!"
    ],
  }
]
```
