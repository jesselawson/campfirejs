# Campfire

Campfire is an engine for telling stories. 

It takes a folder of at least one Campfire file and converts it into a website that you can share or distribute, 
host online, or keep on your desktop.

Use cases include, but are not limited to:
* Interactive educational content, especially for competency-based 
  learning
* Interactive fiction (e.g., CYOA and other mutable narrative IF)
* Topic exploration and presentation

## Features

* Creates HTML+CSS static site
* Implements [CommonMark v0.30.0]()
* Auto-publish to Github Pages
* Uses `pest` for PEG parsing

# TODO

- One parser to read in the campfire file, another parser to parse the card_body
  into Markdown and campfire expressions.

# Quickstart

TODO

# Writing in Campfire

Campfire structures its content in Cards -- which can contain markdown and campfire expressions.

## Cards

A card is a collection of content the same way a webpage is a collection of content.

Cards are initialized with `$begin`, which takes the name of the cardk. 
This is how you can refer back to it. 

Every campfire project starts with at least one file called `start.campfire`, 
and the first cardk is always called "start":

```campfire
// start.campfire

$begin start

```

Think of `$begin start` as the `main` entrypoint into your program.

When you declare a new cardk with `$begin`, everything below it is considered
part of that cardk, up to either the end of the file or another `$begin` statement.

## Language design

A card is composed of one or more campfire expressions.

A campfire expression is any combination of markdown and campfire code. 

Examples:

- `# Chapter one <c did_eat_berries ? (aka) post berries c>`
- `Your favorite **campfire** code editor. <c neat! -> next_card c>`

## Anatomy of a Campfire expression

v1: (open_tag) (label) (cf_operator) (target_card) (options) (close_tag)
v1.1: (open_tag) (label) (cf_operator) (target_card) (close_tag)

cf_operator = 
{
  cf_link_operator
}


cf_link_operator = { “->” }

## Hyperlinks

Cards can link to other cards in two ways: **reveal links**, and **replace links**. 

A reveal link will reveal another card immediately below the currently active card (the one
from which the link was clicked).

A replace link will replace all the cards on the page with a new card. Starting fresh, 
in a way.

## Event hooks

All Campfire expressions will emit an event to the window that can be captured 
with `window.addEventListener(theEventName, someFunction)`. 

The most basic event is a `click` event, which occurs when a user clicks 
a link rendered by a Campfire expression. You can listen for click events or 
any derivative event from a click:

| Activity                      | Hook emitted | 
| ----------------------------- | ---------------------- | 
| Mouse click                   | `next_card1_click_event`       |
| Mouse over                    | `next_card1_mouse_hover_event` |
| Mouse down                    | `next_card1_mouse_down_event` |
| Mouse up                      | `next_card1_mouse_up_event`   | 

# Contributing 

TODO

The bones compiler works by parsing `main.bn`, then parsing each other `*.bn` file 
that it counters by walking through files as they are linked. It is organized 
by command---e.g., `build.rs` in the project's root handles the `bones build` command, 
`publish.rs` handles the `bones publish` command, etc. 

# Roadmap

TODO

* Uses Turbo (formerly turbolinks)