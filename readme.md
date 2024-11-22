# random_asset_displayer
## Introduction

This simple webapp picks a random asset from `./assets/`, loads the filecontent, and displays it. Display mode depends on file type. Currently supported are:
- raw HTML
- images: *.gif, *.jpg, *.jpeg, *.png, *.webp
- videos: *.mp4, *.webm

The list of URL and status codes ist only loaded once during startup and not reloaded during runtime.