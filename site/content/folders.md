---
title: Folders
summary: Trindle's own app for looking after your files, with one pane or two.
group: Using Trindle
order: 30
---

Folders is where you browse, open, copy, move and tidy your files. Open it from the ring.

![The Folders window in Home. A strip at the top switches between Single and Dual panes. The list shows Documents, Downloads, Music and Pictures with their sizes and dates.](img/folders-home.png "What you will see: Folders with one pane. The side bar on the left has Pinned and Places.")

## One pane or two

A pane is one list of files. Folders opens with one pane.

1. Find the Single and Dual switch at the left of the top bar.
2. Press it, or press F3. The change is at once.
3. Your choice is kept for next time.

![A drawn picture of Folders with one pane, and with two panes side by side with a strip of buttons between them.](img/two-panes.svg "What you will see: a diagram of one pane and two panes. It is drawn to explain, not a screenshot.")

With one pane, Folders starts on Home. With two, the left pane starts on Home and the right on Recent. In Folders' own settings you can choose where each pane starts, or have it carry on where you left it.

With two panes, each is a card of its own. A strip of buttons sits between them. The strip can show this folder in the other pane, swap the panes, pick what differs from the other pane, link the two panes and compare them.

The pane you are using has a slightly different edge on its card. That tells you which one a click in the side bar will change. Both panes are always the same width.

![Folders with two panes side by side, each showing a list of made-up files, with a strip of buttons between them.](img/folders-dual.png "What you will see: Folders with two panes and made-up folders.")

## The side bar

The side bar has a Pinned section for the folders you pin. Below it is Places: Storage, Recent and Trash.

Right-click a folder to pin it. If you have no pins yet, Downloads and Documents are pinned for you. If you unpin one, it stays gone.

## Seeing your files

There are three views: Icons, List and Compact. In List you choose which columns show, such as size, type and the date changed.

Folders makes small pictures of your pictures, videos and PDFs. A separate program reads videos and PDFs inside a sealed box with no network. A booby-trapped file cannot reach anything else.

![Folders in Icons view: four folders, Documents, Downloads, Music and Pictures, drawn as large icons.](img/folders-icons.png "What you will see: the same folder in Icons view. List and Compact are the other two views.")

Press Space on a video or a song and it plays straight away. You get pause, a slider and the volume, without opening another app.

## The right-click menu

The first level of the menu shows no more than twelve lines. The least used lines are in four sub-menus: Select, Pack, send and share, Picture, and Tools. Each line has one home, so you find it in the same place whatever you right-clicked.

![The right-click menu on a folder in Folders, with lines such as New folder, Open, Cut, Copy, Rename, Pack, send and share, Tools, Move to Trash and Properties.](img/folders-menu.png "What you will see: the right-click menu on a folder. Pack, send and share and Tools open sub-menus.")

A delete is never the first line of a sub-menu. Wipe securely and Delete permanently sit as far from Open as the menu goes.

Every file opens. If a file's name says nothing about its kind, Folders reads the start of it. An empty file or a plain text one opens in Text. Where no app opens a kind, the Open with window comes up for you to choose one.

## Other computers
Questions: why won't my NAS open; how do I find a shared folder on my network; how do I see other computers in Folders

Network finds the computers on your network that share folders, such as a NAS. A NAS is a small computer that holds files for the whole house. Folders looks in two ways at once and lists each computer once. Open one to see its shares.

If no computers are found, Folders says so, and says what it asked.

A login you type for a shared folder can be remembered for 90 days. The switch says so. A remembered login is sent only to the address it was used at before. If a computer answers at a new address, Folders asks again and says why.

An nfs:// address opens only on a server that offers NFS version 4. Version 3 is found only on very old NAS boxes, and Trindle does not use it, so it keeps nothing extra running. If a server does not offer version 4, a label under the files says so. Kerberos-secured NFS mounts do not work.

Saving a file back to another computer writes a fresh copy first. It swaps it in only when it is complete, so a dropped connection never leaves you with half a file.

## Copying and moving

Paste in Folders moves only what you cut in Folders. Files cut in another program are copied, and the original stays.

![Folders with a card at the foot reading Copying 3 items to Documents, a percentage, a ring that fills up, and pause and stop buttons.](img/folders-copy.png "What you will see: a copy under way. The card at the foot has pause and stop buttons.")

While a big copy runs, a quiet line in its row says that copying uses the disk heavily. Other programs may feel slower until it ends.

## Folders' own settings

Folders keeps its settings in Folders.

1. Press the round cog at the far end of the top bar, or press Ctrl and comma.
2. A card opens over the window.
3. Change what you want. It is saved as you go. There is no Save button.

The card covers when Folders opens, the file endings and tick boxes, opening drives, and the logins Folders has kept.

![The Folders settings card over the file list. Tabs read When Folders opens, The list, Drives, Kept logins and Shares. Three choices say where Folders starts, and two boxes set the left and right pane.](img/folders-settings.png "What you will see: the settings card, opened from the round cog at the top bar.")

## On its own

Folders is a separate app. It does not need the ring or the rest of Trindle to run, and it may be offered on its own one day. What reaches outside it, such as pinning to the ring or opening Settings, only shows up when that part is there.

## When something goes wrong

When Folders cannot do what you asked, it says so in red words at the foot of the window, and a small round **More** button sits beside them. Press it and Help opens at the page for that message. Each page below says what happened, why, and what to do.

![The Folders window with its file list. At the foot, red words say one item could not be moved, invoice 0042.pdf, because it changed after it was copied, with a round More button at the right.](img/folders-failure-foot.png "What you will see: a failure at the foot of the window, with the round More button at the right.")

- [An archive wants a password](folders.html#archive-password): a zip or 7z that asks for one, or refuses the one you typed.
- [An archive will not open](folders.html#an-archive-will-not-open): not an archive Folders reads, damaged, a part missing.
- [An archive will not unpack here](folders.html#an-archive-will-not-unpack-here): too big, no room, or a folder that cannot be written in.
- [Work that did not finish](folders.html#work-that-did-not-finish): a copy, move or delete that left some items behind.
- [Choices that were not kept](folders.html#choices-that-were-not-kept): a setting that came back after you closed Folders.
- [A program would not start](folders.html#a-program-would-not-start): a line that hands your file to another program.
- [A part is not on this computer](folders.html#a-part-is-not-on-this-computer): a line that needs something Folders does not carry itself.
- [A drive would not open or be put away](folders.html#a-drive-would-not-open-or-be-put-away): the Storage page and the side bar.
![The Folders window with its file list. At the foot, red words say HOLIDAY could not be put away because the target is busy, with a round More button at the right.](img/folders-failure-drive.png "What you will see: the foot of the window when a drive could not be put away.")

- [A server would not answer](folders.html#a-server-would-not-answer): a name not found, a connection turned away, a link that dropped, an address that cannot be read.
- [A server would not let you in](folders.html#a-server-would-not-let-you-in): a login, a key, a new or changed identity, a certificate.
- [A server would not do that](folders.html#a-server-would-not-do-that): a server that answered and then said no, or a secure link that could not be made.
![The Folders window with its file list. At the foot, red words say the other program did not take all of what was dragged, so it may not have all of it, with a round More button at the right.](img/folders-drag-short-copy.png "What you will see: a drag that was a copy and fell short.")

- [A phone would not show its files](folders.html#a-phone-would-not-show-its-files): a phone or camera plugged in by USB.
- [A name was refused](folders.html#a-name-was-refused): a name that cannot be used, or an unsafe one from a server.
- [A share asks for a login](folders.html#a-share-asks-for-a-login): a Windows share that wants a user name and password, or a kept login that was held back.
- [A share would not open](folders.html#a-share-would-not-open): a Windows share or NFS export that did not answer, was not found, or was refused.
- [A folder could not be shared](folders.html#a-folder-could-not-be-shared): the Share part of a folder's Properties.
- [A file could not be read](folders.html#a-file-could-not-be-read): the preview, the viewer, a picture, a video, a PDF or the checksums.
- [Details could not be changed](folders.html#details-could-not-be-changed): the dates, read-only flag, photo notes and song tags in Properties.
- [A folder would not open](folders.html#a-folder-would-not-open): a pane that cannot show a folder, or a place that is not answering.
- [Colours and emblems did not follow](folders.html#colours-and-emblems-did-not-follow): something moved and its colour or emblem stayed behind.
- [A drag from or to another program did not finish](folders.html#a-drag-from-or-to-another-program-did-not-finish): a drop that brought nothing, or a drag the other program took only in part.
- [Something could not be made here](folders.html#something-could-not-be-made-here): a new folder, document, link, font or wallpaper.
- [A comparison or search stopped](folders.html#a-comparison-or-search-stopped): Compare and Duplicates.

## Archive password
Questions: why does my zip ask for a password; why does Folders say that password does not open it; how do I open a locked RAR; why can I not open a password protected 7z; what does it is locked with a password mean

Some zip and 7z files are locked with a password. When you open or unpack one, Folders asks for it in a small card called **Password for** and the file's name. Type the password and press **Open** or **Extract**.

![Folders with a small card over the file list. The card is headed Password for holiday.zip, says the file is locked with a password, and has an empty password box and an Extract button.](img/folders-archive-password-card.png "What you will see: the Password for card over a locked archive. The box is empty.")

If the password is not the one, the card stays and says so in red words under the box: "That password does not open it." Type another and try again. The password is kept only while the archive is open and is never saved to the disk.

![The same Password for holiday.zip card after a wrong password. Red words under the box say That password does not open it, and a round More button sits at the foot of the card.](img/folders-archive-password-wrong.png "What you will see: the card after a wrong password. The red words are under the box.")

![The Password for holiday.zip card after a password that fits neither way. Red words under the box say that password does not open it, or the archive is damaged, with a round More button.](img/folders-archive-password-or-damaged.png "What you will see: the card when Folders cannot tell a wrong password from a damaged archive.")

Some messages are different:

- "It is locked with a password and packed a way Folders cannot unlock." None of the readers Folders has can unlock this kind.
- "It is a RAR locked with a password, which Folders cannot unlock; nothing is wrong with the file." Use a program that can unlock a RAR.
- "That password does not open it, or the archive is damaged." With some archives, 7z in particular, Folders cannot tell the two apart, so it says both. Check the password first. If you are sure of it, the file may be damaged: download or copy it again.
- "holiday.zip is locked with a password. Open it again to type it." Another window was open over Folders, so the card could not come up. Close that window, then open the archive again.

:::expert
A password is held in memory only while a pane shows the archive, and is wiped from memory when the card goes. Zip is read with the traditional zip lock and with AES; 7z is read with its own AES lock. A 7z whose file names are locked too needs the password just to list what is inside it.
:::

## An archive will not open
Questions: why will Folders not open my archive; what does it is not an archive Folders can open mean; why does a split archive not open; why is the part of Trindle that reads this kind missing; what does damaged or cut short mean

Folders opens an archive as if it were a folder, so you can look inside it. When it cannot, the red words say why in a few words. These are the reasons.

![A pane opened on broken.zip. The pane is empty except for red words saying broken.zip could not be read because it is damaged or cut short, repeated at the foot beside a round More button.](img/folders-archive-not-opened.png "What you will see: a zip that could not be read. The red words are at the foot, with More beside them.")

- **It is not an archive Folders can open.** Folders goes by the ending of the name and by the first bytes of the file. The file may not be an archive at all, or it may be a kind Folders does not read. Check that the file is the one you meant.
- **It is damaged or cut short.** The file ends too soon or its inside makes no sense. This often happens to a download that stopped part way. Get the file again.
- **It is packed a way Folders cannot read.** The archive uses a method Folders does not know.
![A pane opened on notes.zip. Red words say notes.zip could not be read because it is not an archive Folders can open, repeated at the foot beside a round More button.](img/folders-archive-not-an-archive.png "What you will see: a file that is not an archive Folders can open.")

- **A part of it is missing, and every part has to be in the same folder.** A split archive is several files. Put all of them in one folder, then open the first.
- **The part of Trindle that reads this kind is missing, and installing updates puts it back.** Kinds such as RAR, disc images and .deb files are read by a reader that comes with Trindle. Install the waiting updates, then try again.
- **It is not there any more.** The file was moved or deleted after Folders showed it.
- **It could not be read.** Folders was given no better reason.

![A pane opened on old.7z. Red words say old.7z could not be read because it is a damaged 7z whose part ends too soon, repeated at the foot beside a round More button.](img/folders-archive-damaged-7z.png "What you will see: a 7z that is damaged inside.")

When Folders can tell what is wrong inside a zip or 7z, it says so after "damaged zip" or "damaged 7z": for example that a check does not match, a part ends too soon, a header is far too big, or a count is far too big. All of these mean the file is damaged or has been made to look wrong. Get it again.

If you were taking something out of the archive, the words begin "It could not be taken out of the archive" and end with one of the reasons above.

:::expert
Zip, tar (plain, gzip, bzip2 and xz) and 7z are read by Folders' own code. Every other kind is read by the system's libarchive, shut in a box with no network and no access to the rest of your files. A booby-trapped archive cannot reach anything else. If that box or reader is missing, the reason is the "part of Trindle" line above.
:::

## An archive will not unpack here
Questions: why will my archive not unpack; what does there is not room here for what it holds mean; why does it say it holds more than a million things; why does it need more memory or time than Folders allows; why could names not be taken out

Unpacking is careful. Folders will not fill a drive to the brim, and it limits what a stranger's archive may ask of your computer. These are the reasons it stops.

![The Folders window with its file list. At the foot, red words say one item could not be extracted, big.7z, because there is not room here for what it holds, with a round More button at the right.](img/folders-archive-no-room.png "What you will see: the foot of the window after an unpack stopped. More is at the right.")

- **There is not room here for what it holds.** The drive you are unpacking onto has less free space than the archive needs. Folders always leaves some free, so it stops a little before the drive is full. Free some space or unpack onto another drive.
- **Nothing can be written in that folder.** The folder is read-only, or you do not have the right to put files in it. Unpack into a folder of your own.
![The Folders window with its file list. At the foot, red words say one item could not be extracted, big.7z, because nothing can be written in that folder, with a round More button at the right.](img/folders-archive-no-write.png "What you will see: the foot of the window when the folder cannot be written in.")

- **It holds more than a million things.** Folders will not unpack more than a million things from one archive.
- **It needs more memory or time to read than Folders allows.** Some archives are made to be very costly to open. Folders stops rather than slow the whole computer.
- **This 7z needs more memory to unpack than Folders will give it.** The same rule, for a 7z whose settings ask for a very large amount.
- **N could not be taken out, as their names reach outside or something is in the way.** An archive can hold names that try to climb out of your folder. Folders refuses those and unpacks the rest.

:::expert
The room kept free on the drive is 256 MB. The box that reads the other kinds may use 2 GB of memory. Listing the names of an archive may take it up to 120 seconds of processor time, and unpacking up to two hours. A 7z whose dictionary is larger than 2 GB is refused. Nothing is written outside the folder you chose, links are made last and may not point out, and nothing that comes out is ever run.
:::

## Work that did not finish
Questions: why could my files not be copied; why did a move leave files behind; what does it was left mean; why did a delete not work; what do the words in brackets after a file name mean

When a copy, move or delete does not finish, the red words say how many items failed. They name the first three, then "and N more". Each name may have a reason in brackets after it. If there are no brackets, Folders was not given a reason for that item.

![The Folders window with its file list. At the foot, red words say two items could not be copied and name holiday plans.txt and notes.txt with a reason for each, with a round More button at the right.](img/folders-work-failed.png "What you will see: the foot of the window after a copy where two items failed.")

What the reasons mean:

- **It was not there when this was looked at, so it was left.** A hidden item or a link was gone by the time Folders reached it.
- **Too big to check fully, so it was left.** A hidden item or a link sat in a folder too big to look through to the end.
- **It has changed since it was dropped, so nothing was brought.** You dragged it in from another program and it changed before Folders took it.
- **Something else was given that name meanwhile.** Another program made a file with that name after Folders looked at the folder.
- **The one already there could not go to the Trash.** You chose to replace something. Folders could not put the old one in the Trash first, so it left both.
- **It changed after it was copied, so it is still where it was.** In a move, Folders copies first. It removes the original only when the copy is proved whole. This one changed, so the original stays.
![The Folders window with its file list. At the foot, red words say the drive HOLIDAY would not say it kept what was written, and that report.pdf could not be copied, with a round More button at the right.](img/folders-work-not-kept.png "What you will see: a drive that would not say it kept what was written.")

- **The drive would not keep what was written, so some of it may not be there.** The drive did not confirm that everything reached it. Check the copy before you rely on it.
- **Not all of it could be read.** A folder's list stopped part way.
- **It is inside the one it would replace.** You cannot replace a folder with something that is inside it.
- **This copy made it.** A folder was being copied into itself, so Folders did not copy the new folder the copy had just made.
- **The copy is not what was read.** What was written did not match what was read, so Folders refused it.

![The Folders window with its file list. At the foot, red words say one item could not be copied, Letters, because not all of it could be read, with a round More button at the right.](img/folders-work-not-all.png "What you will see: a folder whose list stopped part way.")

More reasons you may see:

- **It could not be looked through to its end, so it is still where it was.** A folder was too big or too slow to check before it was moved.
- **It could not be taken away.** The copy is whole, but the original could not be removed, so both exist.
- **It could not go to the Trash.** This drive has no safe place for a Trash, there was nothing to move, or the Trash already holds too many items with that name. Delete permanently, if you mean it, or free some room.
- **Not moved, with how many unreadable files, system files or other things stayed.** A folder is taken away only when everything in it is proved to have arrived, so one that holds such things stays where it was.
- **Only JPEG, PNG, WebP and BMP pictures can be turned, the picture could not be read, or it is not a kind of picture Folders writes.** This is for Turn and Resize.
- **Renaming stopped part way.** A batch rename met something it could not do and stopped. The reason follows.

Folders takes an original away only once its copy is proved whole, so an item named here is still where it was. Try again with fewer items, or one at a time.

![The Folders window with its file list. At the foot, red words say one item could not be put in the Trash, budget.ods, because it could not go to the Trash, with a round More button at the right.](img/folders-work-no-trash.png "What you will see: the foot of the window when something could not go to the Trash.")

:::expert
A move across drives copies, proves each copy whole, and only then removes the original, so a failed move leaves the original in place. A file counts as moved only once its copy is known to be complete.
:::

## Choices that were not kept
Questions: why did Folders forget my view; why did my columns come back; what does could not be kept for next time mean; why could Folders not save my settings

Folders keeps your choices in small files of its own in the `trindle` folder of your settings, such as `folders.conf` and `files-views`. They cover panes, views, grouping, columns and icon size. When it says a choice "could not be kept" or "could not be saved", the choice works until you close Folders, then it goes back to what it was.

![The Folders window with its file list. At the foot, red words say The columns could not be saved, with a round More button at the right.](img/folders-choice-not-kept.png "What you will see: the foot of the window when a choice could not be kept.")

A file cannot be written when the disk is full or when the folder cannot be written in. There is one more reason. If a settings file is there but cannot be read, Folders leaves it as it is and writes nothing, so nothing in it is lost.

![The Folders window with its file list. At the foot, red words say the choice of one pane or two could not be kept, with a round More button at the right.](img/folders-choice-panes-not-kept.png "What you will see: another choice that could not be kept.")

The ring's favourites and the pinned folders are kept the same way, and say so in the same words when they cannot be saved. Two more lines belong here: the middle of the ring holds 12 favourites and says it is full until you take one off; and a name whose letters the list cannot hold cannot be kept on the ring.

What to do: make some room on the disk, or check that the settings folder is yours to write in. If the file cannot be read, [report a problem](report-a-problem.html).

The same goes for the lines in Folders' own settings window, such as "It could not be saved." The words after say why. If the settings file is there but cannot be read, the window shows the usual answers and says nothing can be changed until it can be.

:::expert
Folders reads and writes these itself, so it works on its own without Trindle's settings file. `folders.conf` holds a line for each setting as `name=value`; the files for colours, emblems and views hold a line for each folder. Each is written as a new file that takes the old one's place whole. The ring's favourites are kept in the ring's own file. A name that cannot be kept in a settings file, or a value with a line break in it, is refused before anything is touched.
:::

## A program would not start
Questions: why will Folders not open my file in the app I chose; why did a new window not open; why is there no email program to share with; why could the wallpaper not be set; why could Folders not open as administrator

Some lines in Folders hand your file to another program. When that program cannot be started, the red words name it, or say what you were trying to do.

![The Folders window with its file list. At the foot, red words say Folders could not be opened as administrator, with a round More button at the right.](img/folders-program-not-started.png "What you will see: the foot of the window when a program would not start.")

- **An app could not be started.** The app you chose in Open with is not there any more, or would not run. Choose another, or install it again from Apps.
- **There is no email program to share with.** Share by email needs an email program. Install one from Apps. This line also says so when you have picked only folders, because a folder cannot be attached.
- **A new window could not be opened.** Folders starts a second copy of itself for this, and that failed.
- **Folders could not be opened as administrator.** The administrator window did not start.
- **The wallpaper could not be set.** Trindle's wallpaper program did not run.

If the same line fails every time, [report a problem](report-a-problem.html) and say what you pressed.

:::expert
Share by email asks `xdg-email` to attach the files, which hands them to your email program. A new window is the same Folders program started again on the chosen folder.
:::

## A part is not on this computer
Questions: why does Folders say a part is not on this computer; why can I not share a folder; why can I not open a Windows share; why is there no terminal to open this folder in; why can I not share with AI programs

Folders is built to work on its own, without the rest of Trindle. A few lines need a part that is not part of Folders, and on a computer that lacks it, Folders says so rather than doing nothing.

![The Folders window with its file list. At the foot, red words say No terminal is installed to open this folder in, with a round More button at the right.](img/folders-part-missing.png "What you will see: the foot of the window when a part is missing from the computer.")

- **Sharing a folder** is set up by Trindle's own helper.
- **Opening a Windows share, or an NFS export,** needs Trindle's own helper.
- **Setting the wallpaper** needs Trindle's wallpaper program.
- **Opening a folder as administrator** needs pkexec.
- **Sharing with AI programs** needs Trindle's program for them.
- **Open in a terminal** needs a terminal. Install any terminal from Apps and Folders will use it.

On a Trindle desktop the Trindle parts are there. If one is missing, install the waiting updates.

## A drive would not open or be put away
Questions: why will my USB drive not open; what does this drive is changing mean; what does drives cannot be reached just now mean; why could a drive not be put away; why will my encrypted drive not unlock

Folders opens a drive for you from the side bar or from its card on the Storage page. If the drive is locked with a password, Folders unlocks it first, then opens it as you. Putting a drive away is the reverse. Folders closes it, locks it again if it was encrypted, and switches it off once nothing else on it is open. Only drives that are plugged in can be put away. A disk inside the computer never can.

![The Storage page of Folders with two cards, System disk and a USB drive called HOLIDAY that is not mounted. At the foot, red words say HOLIDAY could not be opened, with the reason and a round More button.](img/folders-drive-open-failed.png "What you will see: the Storage page after a drive would not open. The red words are at the foot.")

- **This drive is changing. Open it again when it finishes.** The drive is already being opened or put away. Wait for that to end, then try again.
- **Drives cannot be reached just now.** Folders talks to the computer's drive service (UDisks) and it did not answer. Try again in a moment. If it keeps happening, restart the computer.
- **HOLIDAY could not be opened**, with a reason after it. The reason comes from the drive service, so it is in its words. If the drive is locked, check that you typed the right password.
- **HOLIDAY could not be put away**, with a reason after it. If the reason says something is using the drive, close the folders and files you have open on it, then try again.

Before a drive is put away, any pane that shows it moves off it, so nothing in Folders holds it open.

:::expert
Drives are opened and put away through UDisks. A drive is mounted as you, and opened. A plugged-in drive is unmounted, locked again if encrypted, and powered off once nothing else on it is open.
:::

## A server would not answer
Questions: why can Folders not find my server; what does no server called that could be found mean; why does it say the server did not answer; what does turned the connection away mean; why was the connection to the server lost; why does it say that address could not be read; what does there is no way through to it mean

Folders reaches another computer by its address, such as `sftp://jo@example.com`. Before it asks for a login, it has to get through to the computer at all. When it cannot, the red words say how far it got.

![The Connect to a server card with the address sftp://jo@nas.example.com typed in. Red words under the box say no server called nas.example.com could be found, with a round More button.](img/folders-server-no-answer.png "What you will see: the Connect to a server card when no server of that name could be found.")

- **No server called that could be found.** The name was not known to your network. Check the spelling. If it is a computer in your home, try its number instead, such as `192.168.1.20`.
- **It turned the connection away.** The computer is there, but nothing on it is waiting for this kind of connection on that port. The server program may be switched off, or the port may be a different one. Add the port to the address, such as `sftp://jo@nas.example.com:2222`.
- **It did not answer.** Folders waited and heard nothing. The computer may be asleep or switched off, or on another network. Folders waits about 20 seconds before it gives up.
![The Connect to a server card with the address ftp://jo@nas.example.com typed in. Red words say nas.example.com did not answer, with a round More button.](img/folders-server-did-not-answer.png "What you will see: the card when a server did not answer.")

- **There is no way through to it.** Something between you and the computer, such as a firewall or a network that is down, stopped the connection.
![The Connect to a server card with the address sftp://jo@nas.example.com. Red words say the connection to the server was lost, with a round More button.](img/folders-server-lost.png "What you will see: the card when the link to a server dropped.")

- **The connection to the server was lost, the server stopped answering, or the server closed the connection.** The link worked and then dropped. Open the place again. Anything half-sent is not kept as a whole file: a copy to a server is checked before the old one is replaced.
- **The server sent a message that could not be read.** The server and Folders no longer agree. Open it again. If it keeps happening, [report a problem](report-a-problem.html).
- **That address could not be read.** The words after it say why: it has no `sftp://`, `ftp://` or `mtp://` at the start; only `sftp://`, `ftp://`, `ftps://`, `ftpes://`, `dav://`, `davs://`, `smb://`, `nfs://` and `mtp://` can be opened; a `[` has no `]`; it names no server; the server's name is not a name; the path holds a line break; there is something after the server's name; or what comes after the colon is not a port.
- **The server's address is not known.** A copy of a file from a server was saved, but Folders can no longer tell which server it came from.

:::expert
SFTP rides inside your own `ssh` program with a 20 second connect time, so what Folders says here is what `ssh` found, put into plain words. FTP and WebDAV connect straight from Folders, trying each address the name gives. A secure FTP server whose name cannot be carried in a certificate is refused.
:::

## A server would not let you in
Questions: why does Folders ask me to trust a server; what does no longer has the identity it had before mean; why does it say the server wants a password or a key; what does the certificate could not be checked mean; why was my password not sent; why did the server not accept that login; what is a fingerprint

Folders is careful about who it talks to, because what you type goes to whoever answers. These are the times it stops and tells you why.

![The Connect to a server card for nas.example.com, a server not visited before, with a warning about it and the button Trust it and connect.](img/folders-server-trust.png "What you will see: the Connect to a server card for a server seen for the first time.")

- **It has not been visited before.** Folders cannot tell it is the right computer. If you expected to reach it, press **Trust it and connect**. Folders remembers it, as ssh does.
- **It no longer has the identity it had before.** This is red because it is the one to take seriously. The computer may have been rebuilt, or someone may be in the way. Folders does not open it. If its owner changed it, remove the old one from your known hosts first, then try again.
![The Connect to a server card for nas.example.com. Red words say it no longer has the identity it had before, that someone may be in the way, and that it is not opened, with a round More button.](img/folders-server-identity-changed.png "What you will see: the card when a server's identity has changed.")

- **This server wants a password.** Type it in the password box on the card and press **Connect**.
- **This server wants a key or password.** Name your key file on the card, or add the key to your ssh agent, and try again. For ssh, a password or a key's passphrase is asked for in a small window of Trindle's own.
- **The server did not accept that login.** The user name, password or key was not right.
- **The key is not there.** The key file named on the card does not exist, or is somewhere Trindle never looks.
![The Connect to a server card for nas.example.com with a box for a key file. Red words ask for a key or a password, with a round More button.](img/folders-server-needs-key.png "What you will see: the card when a server wants a key or password.")

- **The certificate could not be checked.** A secure FTP server proves who it is with a certificate. It failed the usual checks for one of these reasons: this computer has no trusted certificates to check it against; it is not signed by anyone this computer trusts, as happens with a certificate a server made for itself; it has expired; it is not valid yet; it was made for a different name; it has been withdrawn; or it did not pass the usual checks. If you can, compare the fingerprint on the card with the one the server's owner gives you, and trust it only if they match.
- **The certificate is not the one you trusted before.** Its owner may have renewed it, or someone may be in the way. Trust the new one only if its owner confirms this fingerprint.
- **The server answers at an address that was only heard on the network.** Any device on a network can answer to a name, so the password has not been sent. If that address is right, press **Connect**. Folders says this on the card, and again in red if work with that server stopped because its link had dropped.

![The Connect to a server card for a secure FTP server, nas.example.com, with the fingerprint AB:CD:12:34:56:78 shown and a warning that its certificate is not the one trusted before.](img/folders-server-cert-changed.png "What you will see: the card when a server's certificate is not the one trusted before.")

A fingerprint is a short code that stands for one certificate or one computer's identity. Two that match are the same.

:::expert
A trusted SFTP server goes into your ordinary ssh known hosts, and a changed one is refused with strict host checking, so Folders never overrides ssh's own refusal. A trusted secure FTP certificate is remembered by its fingerprint for that server, and only that very certificate is let through. A password for FTP or WebDAV is sent to the address it was typed for and no other.
:::

## A server would not do that
Questions: why did the server refuse my copy; what does the server could not do that mean; why does it say permission was refused; what does it is already there mean; why does the secure connection fail; why is ssh not installed; why does a file say it came cut short

The computer answered and let you in, and then would not do what was asked, or the secure link could not be made. The red words give the server's reason in plain words. When it is a list of items from a copy, a move or a delete, each has its reason in brackets, as on [Work that did not finish](folders.html#work-that-did-not-finish).

![The Folders window with its file list. At the foot, red words say one item could not be copied, report.odt, because permission was refused for report.odt, with a round More button at the right.](img/folders-server-refused.png "What you will see: the foot of the window after a copy to a server was refused.")

What a server may say:

- **It is not there / it is already there.** The thing was moved or made by someone else while you worked.
- **Permission was refused.** Your account on the server may not do that in that folder.
- **It is in use by another program.** Try again when the other program has finished.
- **It is not supported yet / the server could not do that with it.** The server does not offer that action, or said no without a better reason. Where the server gave one, it follows.
- **The server is full.** There is no room left for the file.
- **The folder is not empty.** A folder with things in it is not deleted by this action. Delete what is in it first.
- **It is a file, not a folder.** The address ends in a file.
- **The file came cut short, ended before it was all sent, or the copy sent was not the length it should be.** The transfer did not finish whole, so the old file is kept.
![The Connect to a server card with the address dav://jo@nas.example.com. Red words say the server is full, with a round More button.](img/folders-server-full.png "What you will see: the card when a server says it is full.")

- **The folder listing could not be read.** The server answered in a way Folders cannot read.
- **The server does not list folders over WebDAV here, did not answer as a web server, or wants a kind of login Folders does not know.** The address may not be the WebDAV address. Check it with the server's owner.
- **A secure connection could not be set up, started, or finished.** The server stopped answering or closed the link while it was being made. Try again. If it keeps failing, try the same address with `ftp://` in front, but only if you accept that nothing is protected.
![The Connect to a server card with the address ftps://jo@nas.example.com. Red words say secure connections could not be set up because the server closed the link, with a round More button.](img/folders-server-secure-failed.png "What you will see: the card when a secure connection could not be made.")

- **This server does not offer secure FTP.** Try the address with `ftp://` in front. Nothing sent that way is protected.
- **The server will not protect file transfers, so nothing was sent.** Folders refuses to send the file in the clear when you asked for it to be protected.
- **The server did not welcome us.** It answered, but did not give the welcome an FTP server gives.
- **The ssh program is not installed, or ssh could not be started.** SFTP needs ssh. Install it, then try again.
- **SFTP is not on this server, or it is an old kind.** The server may offer another way in, such as FTP.
![The Connect to a server card with the address ftp://jo@nas.example.com. Red words say the server did not welcome us and give what it said, with a round More button.](img/folders-server-no-welcome.png "What you will see: the card when an FTP server did not give its welcome.")

- **Left alone: it changed just before removal.** WebDAV removes a file only if it is still the one Folders looked at.
- **Two files may be given one name.** A plain FTP server cannot refuse a name that is taken, so Folders looks first and says this is all it can do.
- **Not all of it arrived, so it is still where it was.** A move takes the original away only when the copy is whole.
- **A link, a pipe, a device or a socket was not sent.** Folders sends files and folders only.
- **The old one could not be removed, and is there whole as another name.** When a replacement is swapped in, the old file is kept safe if it cannot be removed.

:::expert
A server's file is written under a temporary name and swapped in only when its length is right. When a move across drives is cut short, the original stays. WebDAV removes with a condition on the file's size and time, so a file that changed meanwhile is left. Plain FTP cannot be told to refuse a taken name, so the name is looked at first, which leaves a moment in which someone else could take it.
:::

## A phone would not show its files
Questions: why does my phone not show in Folders; why does it say the phone shows no storage; how do I make my phone show its files; what does permission was refused for the phone's USB connection mean; why did the phone stop answering

Phones, tablets and cameras show in the side bar by themselves once they are plugged in and unlocked. You do not type an address for one. Folders talks to the phone over the USB cable.

![A Folders pane for a phone called Pixel. Red words say the phone shows no storage and tell you to unlock it and choose File transfer, with a Try again button and a round More button beside them.](img/folders-phone-no-storage.png "What you will see: a pane for a phone that shows no storage.")

- **The phone shows no storage.** Unlock the phone and choose **File transfer** for the USB connection. Then try again.
- **That phone is not plugged in / the phone was unplugged.** The cable came out, or the phone went to sleep. Plug it in and unlock it.
![A Folders pane for a phone called Pixel. Red words say permission was refused for the phone's USB connection, with a Try again button and a round More button.](img/folders-phone-refused.png "What you will see: a phone whose USB connection was refused.")

- **Permission was refused for the phone's USB connection.** Trindle was not allowed to use that USB device. Unplug the phone, plug it back in and try again. If it stays refused, [report a problem](report-a-problem.html).
- **The phone is in use by another program.** Another program has the phone open. Close it and try again.
- **The phone stopped answering, refused that, or took only part of what was sent.** The link dropped part way, or the phone said no. Try again, and check the file on the phone.
- **The phone sent something that made no sense.** Unplug it, plug it in and open it again.
- **The phone is full, or the file was shorter than it said.** Free some room on the phone, and copy the file again.
![A Folders pane for a phone called Pixel. Red words say the phone is full, with a Try again button and a round More button.](img/folders-phone-full.png "What you will see: a phone that has no room left.")

- **Files and folders can only go inside one of the phone's storages.** Open a storage, such as the internal one, and put things in that.
- **It is not a folder, it is a folder, or it is not empty.** The action does not fit what is there. A folder is deleted only when empty.
- **A whole storage cannot be removed or renamed.** Folders will not do either.
- **The phone refused that (code 2xxx).** The phone gave no reason in words, only a number. The number helps [a report](report-a-problem.html).

:::expert
Folders speaks MTP and plain PTP itself over USB, with no help from the system's file service.
:::

## A name was refused
Questions: why will Folders not take that name; what does a name cannot hold a slash mean; why does it say the server sent an unsafe name; why was a name skipped; what does that name is kept for the system mean

Folders checks every name before it uses it, whether you typed it or a server sent it. A name it will not use is said in red, and nothing is renamed or made with it.

![A Folders pane with a rename box open on notes.txt. Red words at the foot say A name cannot hold a slash, with a round More button beside them.](img/folders-name-refused.png "What you will see: a rename that was refused. The red words are at the foot.")

Names you type:

- **A name cannot be empty.**
- **A name cannot hold a slash.** A slash separates folders, so it cannot be part of a name.
- **That name is kept for the system.** A single dot or two dots mean the folder itself and the one above it.
- **That name is too long.** No name can be longer than 255 bytes. Letters outside English may take more than one byte each.
- **A file or folder called that is already here.** Choose another name, or rename or move the one that is there.
- **It could not be renamed, with a reason.** The name was fine, but the disk or the folder would not allow the change. You may not write in that folder, or the disk is read-only.

![A Folders pane with a rename box open on notes.txt, now empty. Red words at the foot say A name cannot be empty, with a round More button beside them.](img/folders-name-empty.png "What you will see: a rename box left empty.")

Names a server sends:

- **Skipped: the server sent an unsafe name.** A server can send a name that is not one plain name, such as one with a slash in it that would climb out of the folder. Folders will not make a row for it, because anything you did with it could reach somewhere you did not intend. The rest of the folder is shown. If a few were skipped, the words say how many and name them.
- **A thing with no name** is left out in the same way.

:::expert
A name from a server must be one plain name before it is used on this computer, where it could become part of a path. The same check is made for the name of a server's file that is fetched to be opened, and for the server's own name.
:::

## A share asks for a login
Questions: why does Folders ask for a user name and password for a shared folder; why does it say the user name or password was not right; what does the login kept for it has not been sent mean; why does it ask again for a share I remembered; what does the keyring could not be reached mean; why was my login not kept

A shared folder on a Windows computer or a NAS (an SMB share) often wants a user name and password. These are the ones for an account on that other computer, not for yours. Folders asks for them on a small card called **Open** and the name of the share.

![The Open card for a share called Photos on the computer NAS-HOME, with a user name box, a password box and an Open button. Red words at the top say the user name or password was not right, with a round More button.](img/folders-share-login-card.png "What you will see: the card that asks for a share's login after one was refused.")

When the card says something is wrong, it is one of these.

- **The user name or password was not right.** Type them again. A user name on a Windows computer may need its work group in front, such as `WORKGROUP\jo`.
- **That account may not open it.** The login was right, but that account has no right to this share. Ask whoever looks after the computer.
- **Your account is locked or switched off, or its password has run out.** Fix this on the other computer. Folders cannot.
![The Open card for Photos on NAS-HOME. Red words say NAS-HOME answers at a different address from the one the kept login was used at, so the login was not sent, with a round More button.](img/folders-share-login-held.png "What you will see: the card when a kept login was held back.")

- **A login you kept was not sent.** Folders keeps a login you chose to remember for 90 days, and sends it only to an address it was used at before. If the computer now answers at a different address, at an address Folders could not find beforehand, or one only heard on the network, the kept login stays unsent. That is because any device on a network can answer to a name. The card says which, and shows where what you type would go. If it is your computer, type its user name and password and press **Open**. If you will not use the old addresses again, switch on **Forget the earlier addresses**. Only the last eight addresses a login was used at are remembered.
- **It is open, but the login was not kept.** The share opened, and the keyring did not keep your login. The words after say why.
- **The keyring could not be reached.** Folders keeps remembered logins in your keyring. If it could not be asked, the card says so in red, so a locked keyring is not taken for no login being kept. Unlock the keyring, or log out and in again, then try again.

More about remembered logins is in [A shared folder asks for a login again](troubleshooting.html#a-shared-folder-asks-for-a-login-again).

![The Open card for Photos on NAS-HOME. Red words say the keyring did not answer in time, with a round More button.](img/folders-share-keyring.png "What you will see: the card when the keyring could not be reached.")

:::expert
A kept login is stored in the keyring under the computer and share it was for. A second item remembers the addresses it was used at, with the time of each. It goes to Trindle's administrator helper on its standard input, which writes it to a file only the administrator can read and removes that file when the mount is over. No login is sent to a name alone: it goes to the address the card shows, and no other.
:::

## A share would not open
Questions: why will my network share not open; what does no computer called that could be found mean; why does it say the computer did not answer; what does only speaks SMB1 mean; what does does not offer NFS version 4 mean; why is smbclient not installed; what does shares nothing called mean; why could a share not be closed

Folders opens a Windows share (`smb://`) or an NFS export (`nfs://`) by mounting it, so it shows as an ordinary folder. When that cannot be done, the red words say why.

![The Folders window with its file list. At the foot, red words say NAS-HOME did not answer and it may be asleep or switched off, with a round More button at the right.](img/folders-share-not-opened.png "What you will see: the foot of the window when a share's computer did not answer.")

- **No computer called that could be found on this network.** Check the spelling, or use its number, such as `smb://192.168.1.20`.
- **It did not answer. It may be asleep or switched off.** Wake it, check that it is on the same network, and try again.
- **It only speaks SMB1.** SMB1 is an old way of sharing that is no longer safe, so Trindle does not use it. Switch on a newer version on that computer or NAS.
![The Folders window with its file list. At the foot, red words say NAS-HOME has no shared folder called Photos, with a round More button at the right.](img/folders-share-not-found.png "What you will see: a computer that has no share of that name.")

- **It has no shared folder called that.** The computer answered, but it shares nothing by that name. For NFS, the words are "shares nothing called" the path you typed. Open the computer by itself to see what it does share.
- **It does not let this computer open it.** For NFS, the server's list of who may use the export does not include this computer. Add it on the server.
- **It does not offer NFS version 4.** Trindle opens only version 4. See [An nfs:// address does not open](troubleshooting.html#an-nfs-address-does-not-open).
![The Folders window with its file list. At the foot, red words say a program needed to open the share is not installed, with a round More button at the right.](img/folders-share-program-missing.png "What you will see: a share that needs a program that is not installed.")

- **This needs a program that is not installed.** The words name the Debian package, such as smbclient. Install the waiting updates, or install that package.
- **The administrator's password was not given.** Mounting a share needs it. Try again and type it when asked.
- **That is not the name of a computer, or the path of a shared folder.** The address has something in it a computer name or an NFS path cannot hold.
- **It opened, but where is not known.** The share was mounted and Folders could not tell where. Open the Network page again.
- **A share could not be closed.** Something on it is still in use. Close the folders and files you have open on it, then try again.

:::expert
A Windows share is mounted with `mount.cifs` and its shares are listed with `smbclient`. An NFS export is mounted with `mount.nfs` at version 4. Both mounts go through Trindle's administrator helper, which asks for the administrator's password, and the mounts sit under your own folder of shares. The words come from reading what those programs said.
:::

## A folder could not be shared
Questions: why can I not share this folder; what does only folders that belong to you can be shared mean; why does it say that name is taken; why is the sharing password refused; what does a share name cannot hold mean; why does Samba need setting up

Share this folder, in a folder's Properties, lets other computers at home open it, Windows ones too. Folders refuses in these cases.

![The Share part of Properties for a folder called Albums. Red words say another of your shared folders already has that name and ask you to choose another, with a round More button.](img/folders-sharing-refused.png "What you will see: the Share part of Properties after a name was refused.")

- **This folder cannot be read.** Folders cannot look at it, so it cannot share it.
- **Only a folder itself can be shared, not a link to one.** Share the folder the link leads to.
![The Share part of Properties for a folder called Albums. Red words say only folders that belong to you can be shared and that this one belongs to someone else, with a round More button.](img/folders-sharing-not-yours.png "What you will see: the Share part of Properties for a folder that is not yours.")

- **Only folders that belong to you can be shared.** This one belongs to someone else.
- **Another of your shared folders already has that name.** Choose another name.
- **The name is not allowed.** A name must not be empty, may be at most 80 letters, and may not hold any of `% < > * ? | / \ + = ; : "` or a comma. It may not end in `$`, which would hide the share, and Samba keeps `global`, `homes`, `printers`, `print$` and `ipc$` for itself.
![The Share part of Properties for a folder called Albums. Red words say the password needs at least 8 letters, with a round More button.](img/folders-sharing-password-short.png "What you will see: the Share part of Properties after a password that was too short.")

- **The password needs at least 8 letters.** The sharing password is the one other computers use to sign in. It is kept apart from your login password.
- **Other computers cannot reach it yet.** This is not red. **Let other computers connect** is off in Settings, Network. The button beside the words switches it on after the administrator's password.

The first time you share, Trindle fetches Samba, the program that shares folders, and sets it up. This asks for the administrator's password first and takes about a minute. A failure then is said in the red words under the switches.

:::expert
A share is made with `net usershare`, which only ever offers the folder you own, with a name, whether others may change files, and whether guests need no password. No password is ever part of a share. A share offered to guests also needs the folders above it to be open to other accounts for the way through. **Let visitors through to this folder** does that.
:::

## A file could not be read
Questions: why can Folders not show my picture; why does it say this cannot be played; what does it may be damaged or cut short mean; why is a picture too large to show; why does the preview say it could not be read; why does ffmpeg need to be installed; why could checksums not be worked out

The preview panel, the viewer and the Checksums line in Properties read a file to show it. When they cannot, they say why in a few words, with a quiet **More** beside them.

![The viewer open on holiday.png, a picture. It says This picture is too large to show here: 150 million pixels, with a round More button and an Open in its app button.](img/folders-file-unread.png "What you will see: the viewer when a picture is too large to show.")

- **This file is no longer there.** It was moved or deleted after Folders showed it. Look at the folder again.
- **You do not have permission to read this file.** The file belongs to someone else, or its owner has closed it to you. Ask its owner.
- **This file could not be read, with a reason after it.** The disk or the other computer gave the reason in its own words.
![The viewer open on broken.pdf. It says its name says it is a PDF document but it is not one inside, and that it may be damaged or cut short, with a round More button and an Open in its app button.](img/folders-file-broken.png "What you will see: the viewer when a file is not what its name says.")

- **Its name says it is a picture, but it is not one inside. It may be damaged or cut short.** Folders trusts the name over the contents here, so a cut-short download says so instead of looking like odd text. Get the file again.
- **This picture could not be read.** The picture is damaged, or of a kind Folders cannot open.
- **This picture is too large to show here.** Folders does not open a picture file bigger than 100 MB, or one with more than 120 million pixels, however small the file is. Open it in an image app.
- **This cannot be played. It may be damaged, or of a kind ffmpeg cannot play.** Try the file in another player.
- **Videos and sounds are played by ffmpeg, which is not installed on this computer.** Install ffmpeg and try again.
- **Page N could not be drawn.** A page of a PDF could not be drawn. The PDF may be damaged.
- **The rest could not be read.** A long text file was read in parts. The part you asked for next could not be read.
- **It could not be fetched.** The file is on a server and the copy would not come. See [A server would not do that](folders.html#a-server-would-not-do-that).
![The viewer open on broken.ttf. It says it could not be read because it is not a font Folders can read, with a round More button and an Open in its app button.](img/folders-file-font.png "What you will see: the viewer when a font cannot be read.")

- **It is not a font Folders can read, or it is too large to show.** This is for the font sample page.
- **The file could not be read** beside Checksums. The sums are worked out from the whole file, so none can be shown.

:::expert
Pictures, text and fonts are read by Folders itself. Sounds, videos and PDFs are passed to other programs and are never read by Folders. A text file is read in parts of up to 4 MB, and the viewer offers to read more.
:::

## Details could not be changed
Questions: why can I not change the date taken; why does it say the photo's details could not be written; why does Folders say it can change the notes only in JPEG and PNG photos; what does the date taken must look like mean; why could the song's details not be written

Properties lets you change a file's dates, whether it is read-only or can be run, the notes in a photo and the tags of a song. If a change is refused, red words under the line say why, with a quiet **More** beside them. Nothing is half-changed: a photo is written beside the old one and swapped in, so a failure leaves the old one whole.

![The Details part of Properties for a photo called beach.jpg. The Taken box holds 2026-01-02 03:04 and red words below say the date taken must look like 2026-01-02 03:04:05, with a round More button.](img/folders-details-refused.png "What you will see: the Details part of Properties after a date was refused.")

- **That could not be changed, or the date could not be set.** The file belongs to someone else, or the disk or folder does not allow it. The reason follows.
- **The date taken must look like 2026-01-02 03:04:05.** Year, month and day, then hour, minute and second.
- **Folders can change the notes only in JPEG and PNG photos.** Other kinds can be read but not written.
- **There is no room left.** The disk has no room for the new copy of the photo.
- **The notes are too long to fit in a JPEG, a tag is too big, or a note cannot hold a nul character.** Shorten the note.
- **The way up must be 1 to 8.**
- **Damaged photo notes, with a reason such as a part that runs past the end.** The notes inside the photo do not make sense, so Folders will not write over them.
- **This is a link.** Change the file the link leads to.
- **The song's details could not be written.** The reason follows.

:::expert
Photo notes (EXIF) are written afresh in the photo's own part for them. The camera's maker notes stay where they were, because many count their own positions from the start. A song's tags are written in place when they still fit, and otherwise in a new file beside the old one, which takes its place.
:::

## A folder would not open
Questions: why will my folder not open; what does you are not allowed to open this folder mean; what does this folder is no longer there mean; what does there is nothing at that address mean; why does the pane say the other computer is not answering

When a pane cannot show a folder, it shows the reason in place of the files, with **Try again** and a quiet **More**.

![A Folders pane for a folder called Private. The pane is empty except for red words saying You are not allowed to open this folder, with a Try again button and a round More button.](img/folders-folder-not-open.png "What you will see: a folder that would not open.")

- **You are not allowed to open this folder.** It belongs to someone else, or its owner has closed it to you.
- **This folder is no longer there.** It was moved, renamed or deleted. Go up a level and look again.
- **This folder could not be read.** Folders was given no better reason. A drive that was pulled out, or a disk that is failing, can do this.
- **There is nothing at that place.** What you typed in the address box is not a folder, a file or a place Folders knows. Check it for spelling.
- **The other computer is not answering.** A place on the network, a share or a drive that has gone to sleep gets a short moment to answer. If it does not, the pane stops waiting, so the rest of the window stays quick, and offers **Cancel** and **Try again**.

:::expert
A place is asked about off the window's own thread, so one that never answers cannot freeze Folders. At most eight looks at places are out at one time, so trying again and again does not pile them up.
:::

## Colours and emblems did not follow
Questions: why did my folder colour disappear after I moved it; what does colours could not follow what moved mean; why did an emblem go after a rename

Folders keeps a colour you give a folder, and the emblems you add to a file or folder, in two small files of its own. They are `files-colours` and `files-emblems`, in the `trindle` folder of your settings. They belong to a place, so when you move or rename something, Folders moves its colour and emblems with it. If it cannot write the files, it says "Colours could not follow what moved."

![The Folders window with its file list. At the foot, red words say Colours could not follow what moved, with a round More button at the right.](img/folders-colours-lost.png "What you will see: the foot of the window after colours did not follow a move.")

What it means: the file or folder moved fine. Its colour and emblems may still show until you close Folders, and then they are gone from the new place. Nothing else is lost.

What to do: make room on the disk, or check that the settings folder is yours to write in, as on [Choices that were not kept](folders.html#choices-that-were-not-kept). Then give the colour or emblem again.

:::expert
Each file is written as a new file that takes the old one's place, so a failed write leaves the old one whole. A move or rename of a folder carries the colours and emblems of everything inside it too, and Undo carries them back.
:::

## A drag from or to another program did not finish
Questions: why did nothing happen when I dropped a file on Folders; what does nothing was brought mean; why does it say the other program did not take all of what was dragged; why does it say what it has is a copy; why was a dropped file left where it was

Another program can hand files to Folders by dragging, and Folders can hand files to another program. Neither side can see what the other does, so Folders checks and tells you when something did not go as it should.

![The Folders window with its file list. At the foot, red words say the other program did not take all of what was dragged, so nothing here was moved, with a round More button at the right.](img/folders-drag-short.png "What you will see: the foot of the window after a drag that fell short.")

- **It is not there, so nothing was brought.** The name the other program gave is not a file that is there, or is not an ordinary path on this computer.
- **A folder cannot be put inside itself.**
- **The place it is in is not answering, so nothing was brought.** The drive or computer holding it is asleep or away.
- **The last thing dropped is still being looked at.** Wait a moment and drop again.
- **Answer the question about what was dropped first.** Folders asks before it takes anything a program named itself that is hidden, or outside your home folder and your drives, because any program can name any file.
- **Nothing came, what was dropped is too big or too slow or too many, or is not a list of files on this computer.** Drop fewer at a time, or use Copy and Paste.
- **The other program did not take all of what was dragged, so nothing here was moved** (for a move); **so it may not have all of it** (for a copy). Your originals are still where they were. Check what arrived.
![The Folders window with its file list. At the foot, red words say the other program took what was dragged in a way that cannot show it read all of it, so nothing here was moved, with a round More button at the right.](img/folders-drag-unshown.png "What you will see: a drag the other program took in a way that cannot be checked.")

- **The other program took what was dragged in a way that cannot show it read all of it, so nothing here was moved: what it has is a copy.** The same, for a program that took a copy and could not say it was whole. Delete the originals yourself once you have checked it.
- **A thing changed after it was picked up, so it is still where it was.** It was edited between the pick-up and the drop.

:::expert
A name a program drops must be a plain absolute path with no dot parts, no control characters and a bounded length. A move to another program is not carried out as one: the originals are only removed once the copy is known to be whole, which for some programs cannot be known.
:::

## Something could not be made here
Questions: why could Folders not make a new folder; why could the link not be made; why can this picture not be the wallpaper; why could the font not be installed

Folders makes folders, documents, links and installed fonts, and sets wallpapers. When it cannot, red words say what it was and why, with a quiet **More**.

![The Folders window with its file list. At the foot, red words say New folder could not be made because permission was denied, with a round More button at the right.](img/folders-make-failed.png "What you will see: the foot of the window after a new folder could not be made.")

- **A new folder or document could not be made.** The folder is read-only, you may not write in it, or the disk is full. The reason follows. Make it in a folder of your own.
- **A link could not be made.** The words say how many and name them. A link needs a name that is free in that folder and a folder you may write in.
- **The picture cannot be the wallpaper.** Wallpapers can be up to 4K (3840 by 2160). A still picture can be 20 MB and a moving one 200 MB. The words say which limit it passed. See also [A wallpaper is left out](troubleshooting.html#a-wallpaper-is-left-out).
- **The font could not be installed.** Fonts are copied into the fonts folder in your own data folder. If that folder cannot be made or written in, the font is not installed.

## A comparison or search stopped
Questions: why did the compare stop; why could two files not be compared; why did the search for copies stop; what does 1 file could not be read and was left out mean

Compare the two, the folder comparison and Duplicates read a lot of files. If one of them stops or meets a file it cannot read, red words say so with a quiet **More**.

![The Compare window for notes.txt and readme.txt. Red words say They could not be compared, with the reason and a round More button.](img/folders-compare-stopped.png "What you will see: the Compare window after a comparison stopped.")

- **They could not be compared, the compare stopped, or the search stopped, with a reason.** A file changed or went while being read, a drive went, or you may not read it. Run it again.
- **N files could not be read and were left out.** Duplicates reads every file to be sure two are the same. A file it may not read is not in the answer, so the list may be missing sets.

Nothing is changed by any of these. They only read.
