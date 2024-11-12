# random_redirector
## Introduction

This simple webapp picks a random line from the `./config/config.txt` and
- if it's a number: returns that as status code
- if it's an URL: redirects to it
- if it's a filepath: assumes it's an image returns the file
- if it's none of the above: returns status code 500

The list of URL and status codes ist only loaded once during startup and not reloaded during runtime.