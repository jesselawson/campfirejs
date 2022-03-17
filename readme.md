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

A card is composed of one or more Campfire expressions.

A campfire expression is any combination of Markdown and Campfire snippets.

For the **Alpha** version of Campfire, there is only one snippet available: the **link**.

A link behaves like an `a` tag, but instead of going to a new page, it reveals 
the target card.

```campfire
You can go to the %{cabin}(go_to_cabin) or %{the car}(go_to_car!).
```

In the above example, the word `cabin` is rendered as a Campfire link that will 
reveal the `go_to_cabin` card when clicked. 

```
    label        target card
%{cabin}(go_to_cabin)
```

The roadmap is exploring **plugins** for the beta version.

`%{cabin}(go_to_cabin)[some_plugin]`

Each plugin is a function that is automatically injected with a data structure 
to help you customize Campfire: 

```campfire
Take me to your %{leader}(go_to_leader)[toggleOnClick;randomBackground]
```

```typescript
function toggleOnClick() {
  window.campfire.getDocument(); // returns an array of all cards
  window.campfire.getCurrentCard(); // returns the most recently called card as DIV Element
  window.campfire.getCard(name); // Returns card by name as DIV Element
  window.campfire.thisLink(); // Returns this link's id as SPAN Element
}
```

label: The HTML that gets rendered as the link
target card: The card the link should action us to
plugins: semicolon-separated list of plugins.

Extendable via plugins, with many built-in plugins.

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


# Ideas

- Group links together so that when one is clicked, the rest are no longer available. 
  This is a game feature; is this necessary? (could this be achieved with a plugin-esque add-on?)