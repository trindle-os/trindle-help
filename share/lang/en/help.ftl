# Trindle's own words, in English, for the Help program (src/help). Keys start with help-.

# The window's name, in its title bar.
help-title = Help
# The hint in the search box, and what it is called to a screen reader.
help-search = Search or ask a question
# The hint on the button that goes back to the section read before.
help-back = Back
# The hint on the button that goes to a page or section the writing points to. $title is its name.
help-go = Go to { $title }
# Under the search box when what was typed finds nothing. $typed is the words typed.
help-found-none = Nothing in Help answers "{ $typed }". Try other words.
# Under the search box when the address given on starting is not in Help. $address is what was given.
help-address-unknown = Help has no "{ $address }", so this is the first page.
# The page and section named above a section being read. $page is the page's title.
help-in-page = In { $page }
# The name of the list of answers to a search, to a screen reader.
help-contents = Everything in Help
# The sentence at the top of the front page, over the search box and the pages.
help-lead = Everything about Trindle, in plain words. Search above, or open a page below.
# Under a page's card on the front page. $count is how many sections the page has.
help-sections =
    { $count ->
        [one] 1 section
       *[other] { $count } sections
    }
# The name of a page's first section in the list down the side, the words before its first heading.
help-overview = Overview
# What the list of a page's sections is called, down the side of the page, to a screen reader.
help-in-this-page = In this page
# The heading over the answers to a search, down the side.
help-answers = Answers
# A picture shown as its caption. $caption is the words under the picture.
help-picture = Picture: { $caption }
# The words beside the button that opens the area written for experts, shut to begin with.
help-expert = More detail for experts
# The tip of that button while the area is shut, and while it is open.
help-expert-open = Show the detail for experts
help-expert-shut = Hide the detail for experts
# The tip of the round button that brings back the whole of Help after a More opened one answer.
help-all = All of Help
# The words above the real error text, shown at the top of the section.
help-error = The error, as it was said
# The tip of the round button that copies that text.
help-error-copy = Copy the error
