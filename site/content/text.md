---
title: Text
summary: Trindle's own plain text editor, simple on purpose, and able to handle Markdown.
group: Using Trindle
order: 60
---

Text opens, edits and saves plain text. It is the default for plain text, Markdown, logs and source files.

![The Text window with a toolbar of round buttons and the words Save and Save as, one tab called Untitled, an empty page with line number 1, and a status line reading Line 1, column 1, 0 words, 0 letters, UTF-8.](img/text.png "What you will see: Text with one empty document. The status line at the foot counts lines, words and letters.")

## The basics

- **Several files at once.** They open as a slim row of tabs, each with its own small close button. The text keeps the room.
- **Work is never lost.** Text saves as it goes. Closing anything not yet saved to a file asks first.
- **Saving never loses the old words.** Text writes the new words to a new file beside the document. It swaps that in for the old one in a single step. The name always leads to the whole old file or the whole new one, even if the power goes in the middle. If a save is cut short, Text puts it right the next time it starts.
- **Other encodings.** An encoding is the way letters are stored in a file. Text reads and writes a file in the encoding it came in, as well as UTF-8.
- **Recent files.** Text keeps a list of them. A setting chooses what happens at launch. Either reopen the files from last time, or start with one blank document.

## Editing

Text has line numbers, word wrap on or off, find and replace, a word and letter count, and go to line.

Undo and Redo are round buttons in the top bar after the saving ones. Each is greyed when there is nothing to undo or redo.

To find a word:

1. Press Ctrl+H, or the find and replace button in the top bar.
2. Type the word in the bar that opens.
3. Text shows the words it finds in the page. It counts them, such as 1 of 2, and the arrows move between them.

![Text with a short shopping list open and the word milk found in two places.](img/text-found.png "What you will see: the find bar with the word milk, and the word found shown in the list.")

Spell checking uses the dictionaries already on the computer. Unless Settings names one, it uses British English if it is there. Next comes the computer's own language, then American English.

You can change the font and its size. The look follows Trindle's own, dark or light.

## Markdown

Markdown is a way of writing plain text with simple marks for headings and lists. In Markdown mode, one pane switches between the raw text and the finished look. They are not shown side by side. You can save the finished look as PDF or HTML.

## Closing

- The title bar's close button and Ctrl+Q close Text. Each document with work not yet saved is asked about in turn.
- Ctrl+W, or a tab's own close button, closes that one document and never Text itself. Closing the last one leaves an empty document in its place.

## Related

- Folders opens a file in Text whenever it is plain text.

- [Read about Folders](folders.html)
