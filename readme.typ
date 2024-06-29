#import "@preview/wrap-it:0.1.0": wrap-content  // https://github.com/ntjess/wrap-it/blob/main/docs/manual.pdf
#import "./doc_templates/src/style.typ": set_style
#import "./doc_templates/src/note.typ": *


#show: doc => set_style(
    topic: "Random Redirector",
    author: "구FS",
    language: "EN",
    doc
)


#align(center, text(size: 8mm, weight: "bold")[Random Redirector])
#line(length: 100%, stroke: 0.3mm)
\
\
= Introduction

This simple webapp redirects the user to a random URL or returns an empty response with the status code from a list defined in `./config/redirect_list.txt`. The list of URL and status codes ist only loaded once during startup and not reloaded during runtime.