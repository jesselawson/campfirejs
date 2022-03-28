# Campfire

Campfire makes it possible to render blocks of Markdown as HTML and then link 
to other blocks in a system of "cards," which are like pages. 

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

TODO

## Set commands

Campfire set commands are used to configure Campfire. 

- `$set @title = <string>` Sets the window title of `index.html` (e.g., `$set @title = My Campfire`)
- `$set @card_html_tag = <string>` Sets the element used for new cards (e.g., `$set @card_html_tag = <div>`). This 
  is the element that has class `campfire-card`.


# Architecture
## Parsing

The parser reads the *.campfire file in two stages:
- First, it organizes the file into cards and $set commands.
- Second, it organizes cards into markdown and campfire expressions.

## Compiling

The compiler is responsible for compiling the markdown and campfire expressions.
They're stored in the compiled_body of each card that's part of the document's 
cards_list.

## Generating Javascript

Cards are prepared for dynamic insertion and left to the `onclick.js` plugin 
to handle when, where, and how the card content is rendered. 

## Building the project

The default behavior is to compile a Campfire project into single, valid webpage (`index.html`). 

At the root of the project, run: `campfire build`

# Headers and Footers

If Campfire detects a `header.html` or `footer.html` file in the working directory, 
it will use the contents of each for their respective areas.

Here is an example of locations in the main file, `index.html`:

```
| -------------------- |
|        header        | This can be blank if you want to embed the Campfire project
| -------------------- |
|         body         |
| -------------------- |
|      javascript      |
| -------------------- |
|        footer        | This can be blank if you want to embed the Campfire project
| -------------------- |
```

# CSS Support

If a `style.css` file is found at the root of the project, the contents of that 
file will be loaded into a `<style>` block in the head of `index.html`. 

# Plugin support

To start making plugins, create a `plugins` directory at the root of your project.

## Link click plugin {#onclick-plugin}

You can write your own Javascript to handle the `click` event that is fired off when 
a Campfire link is clicked. 

Campfire will look for a file named `plugins/onclick.js`. If found, it will 
replace its default click even behavior with whatever is found in the plugin file. 

If you leave the file blank, links just wont work; you'll need to at least 
implement some sort of way to show the linked-to card. To help, Campfire provides 
a few functions:

| Function | Description | 
| --------------------------- | ------ |
| `link_element()` | Returns the HTML element of the link that was clicked via `getElementById()` |
| `campfire_card_container()` | Returns the HTML element of the div container where cards are appended as links to them are clicked |
| `target_card_element()` | Returns the HTML element of the target card's root via `getElementById()` |
| `target_card_html_content()` | Returns a string of valid HTML that can be inserted somewhere with `insertAdjacentHTML()` |

The default behavior of every link's click event is as follows:

```javascript
link_element().classList.add('cf-clicked');
campfire_card_container().insertAdjacentHTML('beforeend', target_card_html_content());
// Fades in the card; if you don't delay this a bit, the fade effect wont be visible.
window.setTimeout(function() {
    target_card_element().classList.add('cf-fade-in');
},50);
```

When writing your own `onclick.js` plugin, be sure to account for at least two 
things:
- Change the link to indicate that it was clicked
- Show the user the contents of the next card in a way that allows any links in 
  that card to then reveal any linked-to cards

# Contributing 

# Roadmap

TODO

* Uses Turbo (formerly turbolinks)


# Ideas

- Group links together so that when one is clicked, the rest are no longer available. 
  This is a game feature; is this necessary? (could this be achieved with a plugin-esque add-on?)


Campfiresphere(?)

What if you could register a link online and anytime someone linked to it, it would 
display YOUR card? (This would never work because someone would just change their 
card to something mean or gross -- this is a moderation nightmare).

e.g.:

```campfire
This is %{a link}(^windex).
```

The `^` character means "remote" link. 

Remote links are registered on a first-come, first-serve basis. No squatting.

If I register `^jesse`, That means that someone grabbing `{^jesse}` would actually
be getting `{^jesse/home}`. Since I own `^jesse`, I can make any number of other cards 
in my account, accessible by `{^jesse/card_name}`. 

I can mark cards private. If a user nagivates to one of my private cards OR a 
card that does not exist, it pulls my 404 card. 

- [Feature] You can include a campfire file in another campfire fire. 

```
my-proj/
  start.campfire
  header.html
  footer.html


```

- [Enhancement] header/footer should just be template.html. When will you want a 
  custom header but not footer? Just have a special tag that MUST exist in the file 
  exactly once, and if that is the case, then it's a valid template. 
  It can literally be the only contents of the file if you want. 


- [marketing] Instant "message from" them. You're reading an article and see a link to 
my name. Instead of that link going to another website, it modifies the existing 
site, showing more information but from the resource itself. 

Instead of "go here to read about all this," it's "Here. Here is exactly the card you're 
looking for." 

- [Community] For people building plugins, have an arg command that emits 
  document.link_index as JSON so that they can generate whatever they need to generate.

- If I create a system where anyone can define the javascript that is generated 
  when a link is created, that would give people the opportunity to define their own 
  Campfire-esque experience. Campfire is a community. Share what you build with 
  Campfire -- and how you've built Campfire itself. 

  No reason why this can't be a future feature. Just requires a bit of overhaul.


  - [Issue] If text exists between a card, the error is not helpful:

  thread 'main' panicked at 'unsuccessful parse: Error { variant: ParsingError { positives: [EOI, card], negatives: [] }, location: Pos(771), line_col: Pos((49, 1)), path: None, line: "// Of course, this is only an example!␊", continued_line: None }', src/build/parser.rs:98:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

- [Enhancement] Refactor CampfireError handling; instead of 

```rust
pub enum CampfireError {
  ...
};
```

It should be 

```rust
pub enum CampfireError {
  ...
};

return campfire_error(CampfireError::InvalidFile, &filename);


```

- [Feature/Bug] When a link is clicked, search the link index for all other links to that card 
  and mark them as clicked.
  (Perhaps this should be configurable? I can forsee interesting reasons for wanting to be 
  able to render the same card many times (for example, when teaching a concept -- instead of 
  scrolling back up, just render the content again?))

- [Enhancement] `plugins/onclick.js` should really be `plugins/link_click.js` because
  what if we wanted to rope in a different onclick event somewhere?


- [Enhancement] `campfire build --html_pages` outputs the project in zero javascript --
  all html files linked to one another and the start card as index.html

- [Enhancement?] Being able to hook into different events

Campfire creates a set of empty functions that can be overridden to provide 
custom functionality based on event listeners attached to each link. 

Using the example link `%{Let's go!}(next_card)`, the following 
event functions can be overridden: 

| Event type    | Global function 
| ------------- | ---------------------
| `mouseenter`  | `window.Campfire.next_card_mouseenter`
| `mousedown`   | `window.Campfire.next_card_mousedown`
| `mouseup`     | `window.Campfire.next_card_mouseup`
| `mouseleave`  | `window.Campfire.next_card_mouseleave`
| `click`       | `window.Campfire.next_card_click`

Or maybe these could be "Event Hooks":

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

