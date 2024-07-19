#import "@preview/wrap-it:0.1.0": wrap-content  // https://github.com/ntjess/wrap-it/blob/main/docs/manual.pdf
#import "./doc_templates/src/note.typ": *
#import "./doc_templates/src/style.typ": set_style


#show: doc => set_style(
    topic: "random_redirector",
    author: "구FS",
    language: "EN",
    doc
)
#set text(size: 3.5mm)


#align(center, text(size: 2em, weight: "bold")[random_redirector])
#line(length: 100%, stroke: 0.3mm)
\
\
= Introduction

This simple webapp redirects the user to a random URL or returns an empty response with the status code from a list defined in `./config/redirect_list.txt`. The list of URL and status codes ist only loaded once during startup and not reloaded during runtime.