link_element().classList.add('cf-clicked');
campfire_card_container().insertAdjacentHTML('beforeend', target_card_html_content());
// Fades in the card; if you don't delay this a bit, the fade effect wont be visible.
window.setTimeout(function() {
    target_card_element().classList.add('cf-fade-in');
},50);
