# Campfire

This project is a proof-of-concept compiler for the Campfire language. It 
takes a `.campfire` file and compiles it to a static website (`index.html`, 
`campfire.js`, and an optional `style.css`).

The Campfire language is basically GitHub-flavored Markdown organized 
into cards that can be linked to and from one another. Every time you click 
a link to a specific card, a new instance of that card is revealed on the screen.

Campfire was originally designed to build competency-based digital textbooks, 
but you're not limited to only educational content. It naturally encourages 
exploration and is useful for both mutable and immutable narratives. 

Use cases include, but are not limited to:
* Interactive educational content, especially for competency-based learning 
* Interactive fiction (e.g., CYOA and other mutable narrative IF)
* Topic exploration and presentation

**Currently in working alpha.** Things are subject to change, and that's okay. 
## Current features (and limitations)

**Marketing fluff**

* Define your cards in one file (`start.campfire`)
* Write in Markdown (uses a custom Markdown-compatable syntax)
* Lightning-fast compilation to a static site

**Reality**

* Single `start.campfire` source file only (future goal: parses all `.campfire` files present in directory)
* Compiles to a static site (index.html, campfire.js, optional style.css) (future goal: compile to single html file)
* Markdown compatible (future goal: fix inline code snippets not rendering properly, and fix command tags executing in code fences)
* Uses `pest` to interpret Campfire
## Next Milestone 

The next milestone is the `publish` command, to auto-publish to Github Pages. 

I understand this presumes that people will be hosting these on GitHub Pages, but 
think of this as a sensible default rather than a restriction.

# Quickstart

**As a user**

1. Download the latest release, and extract the `campfire` executable to a directory 
in your path. 

2. Create a project directory and then, inside, a file named `start.campfire`. 

```bash
$ mkdir my-project && cd my-project
my-project/$ touch start.campfire
```

3. Copy and paste the following into `start.campfire`:

```campfire
$set @title = Hello, World!

$begin start

Hello, world! It's nice to %{meet you}(last).

$end
$begin last

How fun!

$end
```

4. Compile your project, which creates a `project` folder.

```bash
my-project/$ campfire build

```

5. Open the `index.html` file that is inside the `project` folder. 

# Writing in Campfire

## Cards

Campfire content is organized in **cards**, which always start with the **begin tag** (`$begin <cardname>`)
on a new line and always end with **end tag** (`$end`) on a new line.

When you declare a new card with `$begin`, everything below it is considered
part of that card, up to either the end of the file or an end tag.

## Campfire expressions

Content between the begin and end tags can be any combination of GitHub-flavored 
markdown and **campfire expressions**. 

Currently, only one Campfire expression has been implemented: the **card link**. 
A card link creates a link to another card and has the following syntax: 

`%{link text}(target_card)`

For example, if I wanted to have the word "button" link to the card named 
`next_card`, I would write the following: 

`%{button}(next_card)`

A link behaves like an `<a>` tag, but instead of going to a new page, it reveals 
the target card.

```campfire
You can go to the %{cabin}(go_to_cabin) or %{the car}(go_to_car!).
```

In the above example, the word `cabin` is rendered as a Campfire link that will 
reveal the `go_to_cabin` card when clicked. 

```
%{cabin}(go_to_cabin)
    |         |
  label  target card
```

## Campfire files

Every campfire project starts with at least one file called `start.campfire`, 
and the first card is always named "start":

```campfire
// start.campfire

$begin start

```

Think of `$begin start` as the `main` entrypoint into your Campfire experience.

Currently, Campfire only supports having one file named `start.campfire`.
## Set commands

Campfire set commands are used to configure Campfire. 

- `$set @title = <string>` Sets the window title of `index.html` (e.g., `$set @title = My Campfire`)
- `$set @card_html_tag = <string>` Sets the element used for new cards (e.g., `$set @card_html_tag = <div>`). This 
  is the element that has class `campfire-card`.


# Architecture

Campfire's `do_build()` function kicks off the main compilation cycle, which has 
four steps: parsing, compiling, generating, and building. 

In the Parsing step, the parser reads the `start.campfire` file in two stages:
- First, it organizes the file into cards and $set commands.
- Second, it organizes cards into markdown and campfire expressions.

In the Compiling step, the compiler is responsible for compiling the markdown and campfire expressions.
They're stored in the `compiled_body` of each card that's part of the document's 
`cards_list`.

In the Generating step, cards are prepared for dynamic insertion by creating 
HTML elements for them, and left to the `onclick.js` plugin to handle when, 
where, and how the card content is rendered. 

The Building step is where the files are written. The goal default behavior is to 
compile a Campfire project into single, valid webpage (`index.html`). For now, 
there is an index.html, campfire.js, and optional style.css.

To build a project, go to the root of your project and run: `campfire build`

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

Feel free to clone, tinker, then open a pull request. 

# Ideas

- Group links together so that when one is clicked, the rest are no longer available. 
  This is a game feature; is this necessary? (could this be achieved with a plugin-esque add-on?)

- [Feature] You can include a campfire file in another campfire fire. 

- [Enhancement] header/footer should just be template.html. When will you want a 
  custom header but not footer? Just have a special tag that MUST exist in the file 
  exactly once, and if that is the case, then it's a valid template. 
  It can literally be the only contents of the file if you want. 

- [Idea] Instant "message from" them. You're reading an article and see a link to 
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

// Errors should return something helpful
return campfire_error(CampfireError::InvalidFile, &filename);
```

- [Feature/Bug] When a link is clicked, search the link index for all other links to that card 
  and mark them as clicked.
  (Perhaps this should be configurable? I can see interesting reasons for wanting to be 
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

- [Feature; don't be so hard on yourself] (cloned_elements_inserts) What 
happens when you link to a card that was already linked to? Shouldn't that card come again?
Perhaps we clone the element so that we can create as many of the cards as there are links to it.

- [Expression plugins]

Exploring **expression plugins** for the beta version.

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