---
title: Drives
summary: See every drive and what is on it, and change them. Every change needs an administrator's password.
group: Using Trindle
order: 70
---

Drives shows every drive and what is on it. A drive is a disk or a USB stick. You can also change drives here.

You can do these things.

- Rename a drive.
- Choose the folder a part appears in.
- Mount a part at start-up. Mounting means making a drive's files show up as a folder.
- Resize a part, check and repair it, or format it. Formatting wipes a part and sets it up again from scratch.
- Erase a whole drive, delete a part, or make a new one.

![The Drives window with a disk shown as a bar of parts, and a list of drives with their names and sizes.](img/drives.png "What you will see: the Drives window with made-up disks. Each part is a block on the bar.")

![A card over the Drives window for a part called Data, with boxes for its name and its folder, a switch for mounting at start-up, and a check for faults.](img/drives-part.png "What you will see: a card for one part of a drive, with made-up names. It has the name, the folder and the start-up switch.")

## A password before any change

Drives won't change anything without an administrator's password. An administrator is someone who is allowed to change the whole computer. Drives asks for the password at the moment you make the change.

This applies to a second disk inside the computer, the disk the system runs from, and a USB stick. A password you gave a few minutes ago doesn't count.

![A drawn diagram: you ask to change a drive, a box asks for a password, and then either the change is made or nothing changes.](img/drives-password.svg "What you will see: a diagram of the password step. It is drawn to explain, not a screenshot.")

The password box tells you which drive it is and what will be done to it. If you close the box without giving a password, nothing changes. A label under the row tells you this: "No password was given, so nothing changed."

If a change stops half way, the row tells you why. It also lists which steps were done and which were not.

## Making room for a new partition

A partition is one named part of a disk. On a Btrfs disk, Drives can make room for a new partition while you carry on working. Btrfs is a way of setting up a disk that can take snapshots. A snapshot is a saved copy of the disk as it was at one moment.

![One disk drawn as a bar of three parts: another system that is kept, Trindle, and free space.](img/disk-parts.svg "What you will see: one disk as a bar of parts, as an example. It is drawn to explain, not a screenshot.")

The disk the system runs from is Btrfs, whether it's encrypted or not. After you give the password, Drives does these steps.

1. It takes a snapshot first.
2. It makes the files' area smaller.
3. It makes the open (unlocked) copy of the encrypted part smaller.
4. It makes the part itself smaller, and checks the sizes after each step.

![The Drives window with the box of changes open over a made-up Btrfs part called system. A Make room for a new part line has a slider for how many gigabytes, and a button to make the room.](img/drives-room-slider.png "What you will see: the box of changes, with room that can be made.")

That leaves free space where you can make the new part. Drives asks for the password twice, once for the snapshot and once for the change. If anything looks doubtful, Drives refuses and tells you why.

For other kinds of disk, a label tells you why they can't be made smaller while in use. They can only be made smaller when nothing is using them.

## Hiding a drive

A drive's card has a small round eye button. So does each part's row.

1. Press the eye to hide the drive or part.
2. It no longer appears in the list in Drives or in the side bar of Folders.
3. The eye on a hidden drive has a line through it. Press it again to show the drive.

![The Drives window with a made-up second disk hidden. Its card is greyed out and the eye on it has a line through it.](img/drives-hidden.png "What you will see: a drive that is hidden.")

Hiding doesn't lock or protect a drive. It doesn't change anything on the drive, so it doesn't ask for a password. Hiding a drive hides everything on it.

The drive the system runs from has no eye, so nothing on it can be hidden.

## When something goes wrong

When Drives can't do what you asked, it shows red words under the thing you were doing, with a small round **More** button beside them. Press it and Help opens at the page for that message. Each section below explains what happened, why, and what to do.

![A drawn diagram of three steps: red words say what failed, you press More beside them, and Help opens at the section for those words.](img/errors-path.svg "What you will see: the way from the red words to the fix. It is drawn to explain, not a screenshot.")

- [The drive service did not answer](drives.html#the-drive-service-did-not-answer): Drives couldn't show the list of drives or start a change.
- [A change was not made without a password](drives.html#a-change-was-not-made-without-a-password): you closed the password box, the password was refused, or it couldn't be taken back.
- [A change did not finish](drives.html#a-change-did-not-finish): a step failed, a drive was busy, or Drives was closed part-way through.
- [Room could not be made](drives.html#room-could-not-be-made): the reasons Drives can't make a part smaller to free up room.
- [A name or size was not accepted](drives.html#a-name-or-size-was-not-accepted): a drive's name, its folder, or a size that can't be set.
- [A drive needs attention](drives.html#a-drive-needs-attention): a drive reports that it's failing, a repair didn't fix it, or a drive isn't responding.
- [The computer is nearly full](drives.html#the-computer-is-nearly-full): there's almost no room left.
- [Hiding a drive was not kept](drives.html#hiding-a-drive-was-not-kept): Drives couldn't remember that you pressed the eye.

## The drive service did not answer
Questions: why does it say the drives cannot be read right now; why does it say the drive service could not be reached; why does it say the drive service is not available; why is the list of drives empty; what is udisks2; what does the system bus is not available mean

Drives doesn't talk to your drives itself. It asks a service the computer already has, called udisks2. This service sees your disks and USB sticks and does the work for Drives. When it doesn't answer, Drives has nothing to show and nothing it can change.

![The Drives window on a made-up computer that has no drive service. A red heading says the drives cannot be read right now. A card below says the drive service is not available and how to install it, with a round button on its right to try again.](img/drives-service-missing.png "What you will see: Drives with no drive service.")

- **"The drives cannot be read right now."** This is the red sentence at the top. A card under it tells you what is missing: **"The drive service is not available."** or **"The system bus is not available."**, along with the command that installs it. Once it's installed, press **Try again**, the round arrow at the right of the card.
- **"The drive service could not be reached, so nothing changed."** You pressed something, but Drives couldn't ask the service to do it. Nothing was started. Wait a moment, then press the button again.

![The Drives window. Under the row of a made-up part called Backup, red words say the drive service could not be reached, so nothing changed, with a round help button.](img/drives-no-service.png "What you will see: a change when the drive service did not answer.")

- **"The drive service could not be asked, so nothing changed."** Drives couldn't send a step to the service. If other steps were already done, the message tells you the work stopped part-way and lists them, as [A change did not finish](drives.html#a-change-did-not-finish) explains.
- **"The drive service could not be reached."** You may see this after a part was formatted. The part is formatted, but its folder choice couldn't be saved. Choose the folder again from the part's row.

If the service doesn't come back, restart the computer.

:::expert
Drives talks to `org.freedesktop.UDisks2` on the system bus. "The system bus is not available" means Drives couldn't connect to the bus itself. It is shown with `dbus` as the package to install. "The drive service" is `udisks2`. A change that can't be sent at all counts as a change that did nothing, which is why you see "nothing changed".
:::

## A change was not made without a password
Questions: why does it say no password was given so nothing changed; why does it say the administrator's password was not accepted; why does it say another change is still being made; why does it say the password could not be taken back; why does it say this login is not allowed; why does Drives ask for a password every time

Drives asks for an administrator's password every time you change a drive. An administrator is someone who is allowed to change the whole computer. Drives never reuses the password for the next change, and it checks that the next change will ask for one again.

![The Drives window. Under the row of a made-up part called Data, red words say no password was given, so nothing changed, with a round help button beside them.](img/drives-no-password.png "What you will see: a change that was not made without a password.")

- **"No password was given, so nothing changed."** You closed the password box without giving a password. Press the button again and give it.
- **"No password was given, so the work stopped part-way."** This is the same, but some steps were already done. The words after it tell you which. See [A change did not finish](drives.html#a-change-did-not-finish).
- **"The administrator's password was not accepted, so nothing changed."** The password was wrong. Press the button again and type it carefully.

![The Drives window. Under the row of a made-up part called Data, red words say the administrator password was not accepted, so nothing changed, with a round help button.](img/drives-password-wrong.png "What you will see: a password that was not accepted.")

- **"This login is not allowed to do that, whatever password is given, so nothing changed."** This user isn't allowed to change drives at all. Log out, log in as an administrator, and try again.
- **"Nothing was there to ask for the administrator's password, so nothing changed."** The box that asks for the password didn't appear. Log out and back in, then try again.
- **"Another change is still being made. Try again when it has finished."** Drives makes one change at a time. Otherwise a second change could use the first change's password.

![The Drives window. Under the row of a made-up part called Photos, red words say another change is still being made and to try again when it has finished, with a round help button.](img/drives-one-at-a-time.png "What you will see: a change that didn't go ahead because another is under way.")

- **"It could not be made sure that the password would be asked for, so nothing changed."** Before each change, Drives checks that it will really ask for the password. This time it couldn't be sure, so it did nothing.
- **"The password just given could not be taken back, so for a few minutes another change to a drive may not ask for one."** The change itself is done. The system (polkit) can remember a password for a few minutes. Drives tries again every fifteen seconds, for about five minutes, to take it back. The red words stay until that works. When it does, the row tells you.

![The Drives window. Under the row of a made-up part called Data, red words say the password just given could not be taken back, so another change may not ask for one, and that Drives is trying again. A round help button sits beside them.](img/drives-password-standing.png "What you will see: a password that could not be taken back.")

- **"... was unmounted for this, and mounting it again needs a password of its own: mount it from its row."** The change didn't go ahead after Drives had unmounted a part for it. Mount the part again from its row.

:::expert
Polkit asks for the password for each step that changes a drive. Mounting, unmounting, ejecting and powering off don't ask for one. As soon as the change ends, Drives asks polkit to take back the permission it was given. If that can't be done straight away, Drives shows the red "could not be taken back" words and tries again, up to 21 times, 15 seconds apart.
:::

## A change did not finish
Questions: why does it say the work stopped part-way; why does it say something is still using it; why does it say there is not enough room for what is on it; why does it say a part is still unmounted; why does it say Drives was closed while this change was being made; how do I know what was done

A change to a drive is a list of steps, such as unmount, resize, check and mount. If a step can't be done, Drives stops there and doesn't carry on. It then tells you exactly what happened, so you're never left guessing.

![The Drives window. Under the row of a made-up part called Photos, red words give the reason the work stopped, say it stopped part-way, and list what was done (unmounting Photos) and what failed (resizing what is on Photos). A round help button sits beside them.](img/drives-part-way.png "What you will see: a change that stopped part-way.")

The red words follow this order.

1. Why it stopped, such as "The work stopped part-way." followed by the reason.
2. **Done:** the steps that were done.
3. **Failed:** the step that failed. There's no way to know how far it got.
4. **Not done:** the steps after that one, which Drives left alone.
5. **Not known:** shown only when Drives itself was closed part-way through. The step it was in the middle of may or may not have happened.

![The Drives window. Under the row of a made-up part called Photos, red words give the reason the work stopped, then list what was done, what failed and what was not done. A round help button sits beside them.](img/drives-part-way-lists.png "What you will see: the account of a change that stopped part-way.")

Then, for each part that Drives unmounted for the change and didn't mount again, you'll see one of these.

- **"... was unmounted for this and is still unmounted: mount it from its row."** Nothing had been changed before it stopped.
- **"... was unmounted for this and is not mounted now."** A step had already changed the drive, so mount it from its row when you're ready.
- **"... may have been unmounted: look at its row."** It stopped during a step that unmounts, and there's no way to know whether it did.
- **"... could not be mounted again (...)."** Drives tried to mount it again and couldn't. The reason follows.

You may also see these reasons:

- **"Something is still using it. Close any windows or programs showing its files, then try again."** A part can't be unmounted, resized or formatted while a program has its files open.
- **"There is not enough room for what is on it at that size."** The drive service reported that there was no space left. Or the size was too small for what is stored. Choose a larger size.

![The Drives window. Under the row of a made-up part called Photos, red words say there is not enough room for what is on it at that size, with a round help button.](img/drives-no-room.png "What you will see: a size with not enough room.")

- **"Drives was closed while this change was being made."** You see this the next time Drives opens. Look at the drive before trying again. Drives stays open until a change has finished, so it can tell you how it ended.
- **"Formatted, but it was not given its folder ... again."** The part was formatted, but its folder or its start-up choice was lost along the way. Choose the folder again from its row.

![The Drives window. Under the row of a made-up part called Photos, red words say it was formatted but not given its folder again, and to choose the folder again from its row.](img/drives-folder-not-kept.png "What you will see: a folder that was not given back.")

:::expert
Each step is one call to udisks2. While the changes run, Drives writes the list of steps to a note on disk. That is how it can tell you about a change that was cut short the next time it opens. A step that only mounts, unmounts, ejects or powers off changes nothing on the drive. "Nothing changed" appears only when no step that changes the drive was done before the stop.
:::

## Room could not be made
Questions: why does it say room cannot be made; why can I not make a partition smaller; why does it say mount it first; why does it say a swap file lies on it; why does it say it is mounted read-only; why does it say it is not a GPT drive; why does it say a snapshot can only be taken of the system's own disk

Making room means making one part smaller while the system is using it, so the freed space can hold a new part. Drives only does this when it is certain it can. It checks everything before it changes anything. If it isn't certain, it tells you why in a line under the part and doesn't show the slider.

![The Drives window with the box of changes open over a made-up part called system. Its Make room for a new part line says the part is mounted read-only, so room cannot be made. A round help button sits beside it.](img/drives-room-refused.png "What you will see: room that cannot be made.")

This only works for a Btrfs part, the kind the system's own disk uses. These are the reasons, in the order Drives checks them.

- **For a part that is not Btrfs:** an ext2, ext3 or ext4 part "can only be made smaller when it is not in use"; an xfs part "cannot be made smaller"; and any other kind "cannot be made smaller while it is in use". Unmount it, then use the ordinary resize in its pencil box instead.

![The Drives window with the box of changes open over a made-up ext4 part called system. Its Make room line says the part cannot be made smaller while it is in use.](img/drives-room-other-kind.png "What you will see: a kind of part that cannot be made smaller while in use.")

- **"It is not a part of a drive's table, so room cannot be made from it."** It's the whole drive, not a part.
- **"The drive service cannot make Btrfs smaller while it is in use here, so room cannot be made."** This computer's drive service doesn't offer that.
- **"It holds other parts"**, or **"Its drive does not have a GPT table"**. Only a plain part on a GPT drive can be made smaller this way.
- **"Mount it first: Btrfs is made smaller while it is mounted."** and **"It is not mounted in exactly one place"**. Mount it in exactly one place.
- **"It is mounted read-only"**, or Drives couldn't tell whether it is.
- **"The files spread over more than one drive"**, or Drives couldn't tell.
- **"A swap file lies on it"**, or Drives couldn't tell. A swap file is a file the computer uses as spare memory.

![The Drives window with the box of changes open over a made-up Btrfs part called system. Its Make room line says a swap file lies on it, so room cannot be made.](img/drives-room-swap-file.png "What you will see: a part with a swap file on it.")

- **"How much of it is in use could not be read"**; **"The drive service does not say how big the files' area is"**; **"The open copy of it could not be found"**; and **"The sizes given for it do not agree"** (also for the encryption). Drives won't guess a size.
- **"Another change is still being made."** Try again when it has finished.
- **"A snapshot can only be taken of the system's own disk, so room cannot be made here."** Drives takes a snapshot before it makes room. That way you can go back. A snapshot is a saved copy of the disk as it was at one moment. Only the system's own disk can have one.
- **"It holds ... and needs more than that kept, so too little can be freed."** The part has to keep what is in it. It also needs a tenth more, or 2 GB, whichever is more. At least 1 GB has to be free to give. If less than that can be freed, Drives offers nothing.

![The Drives window with the box of changes open over a made-up Btrfs part called system. Its Make room line says it holds 299 GB and needs more than that kept, so too little can be freed.](img/drives-room-too-little.png "What you will see: room that would leave too little.")

- **"Room from ... to ... can be made."** You see this when the size you chose is outside that range. Choose one inside it.

If something goes wrong once Drives has started, the snapshot is your way back. Drives also checks after every step:

- **"The snapshot could not be found afterwards."** and **"Nothing was resized."** The snapshot didn't appear, so nothing was changed.
- **"Things changed while the snapshot was taken, so nothing was resized."** Drives worked out the sizes again after you gave the password, and they no longer fitted.
- **"... so the work was stopped."** A check after a step found that the files, the encrypted copy or the part wasn't the size it should be. Or the drive service no longer listed one of them. Drives stops before the next step, so your files never end up bigger than the space that holds them.

![The Drives window with the box of changes open over a made-up Btrfs part called system. Its Make room line says the files are no longer mounted, so the work was stopped.](img/drives-room-stopped.png "What you will see: a run of making room that a check stopped.")

:::expert
The order is always the files' area first, then the open copy of an encrypted part, then the part itself. Each step is followed by a check of the sizes the drive service reports. The part keeps what is used plus the larger of a tenth of it and 2 GiB. Room of less than 1 GiB isn't offered. An encrypted part's header, between its start and the open copy, must be between 1 MiB and 64 MiB. A size is accepted if it is within 4 MiB under the target, to allow for rounding.
:::

## A name or size was not accepted
Questions: why does it say that name is too long; why does it say use only letters, numbers and dashes; why does it say a folder is already in use; why does it say FAT names cannot hold some marks; why does it say this kind cannot be made smaller; why does it say mount it first to make it larger

Drives checks what you type before it asks for a password, so a name it won't accept never changes anything. The message appears under the box you typed in.

![The box of changes for a made-up part called Photos. The Folder box holds a name with a space in it, and red words under it say to use only letters, numbers, dashes and underscores, with a round help button.](img/drives-name-refused.png "What you will see: a folder name that was not accepted.")

For the folder a drive appears in (Drives makes it under `/mnt`):

- **"Give it a name first."** The box is empty.
- **"That name is too long: 32 letters at most."**
- **"Use only letters, numbers, - and _ (no spaces)."**
- **"Start it with a letter or a number."**
- **"/mnt/name is already in use."** Another drive already has that folder. Choose another.

For the name of a part, which is its label, these are the longest names each kind allows:

- FAT: 11 letters, and no `* ? . , ; : / \ | + = < > [ ] "`.
- xfs: 12. exFAT: 15. ext2, ext3 and ext4: 16. NTFS: 32. Btrfs: 255.

For a size:

- **"Mount it first: this kind can only be made larger while mounted."** (or smaller). Mount the part, then try again.
- **"This kind cannot be made larger."** (or smaller). Some kinds can only be resized one way, or only when unmounted. Drives goes by what the drive service says each kind can do.

:::expert
Drives counts lengths in bytes, so a name with accented letters may allow fewer letters than the figure. A part that isn't mounted can be resized offline if its kind allows it. Drives checks ext filesystems before an offline change.
:::

## A drive needs attention
Questions: why does it say a drive is failing; why does it say copy anything you want to keep off it now; why does it say it needs a look; why does it say a drive is read-only; why does it say no drive answering; why does it say some faults could not be put right

A drive can report on its own health. Drives shows what the drive reports, and nothing more.

![The Drives window. A red sentence at the top tells you a made-up drive called Old disk is failing and to copy anything you want to keep off it now. Its card says the same in red, and a round help button sits beside the sentence and the card.](img/drives-failing.png "What you will see: a drive that says it is failing.")

- **"Old disk is failing. Copy anything you want to keep off it now."** The drive's own check says it is going to fail. Copy what matters to another drive straight away. Don't repair, format or resize it first.
- **"Old disk needs a look: ..."**, followed by the reason. It might have damaged spots, be wearing out, be running out of spare space, be too hot or have made itself read-only. Or its memory backup might have failed, part of it might have made itself read-only, or it might have worn past what its maker expects. These are the drive's own warnings. Copy the files you care about and plan to replace it.
- **"Some faults could not be put right. Copy off anything you want to keep."** You pressed the spanner to repair a part, but the repair didn't fix it.
- **"Faults found: press the spanner to repair them."** The check found faults. The part is unmounted while the check runs. Repair it, or copy your files off first.
- **"No drive answering."** The case responds, but the drive inside it doesn't. Unplug it, wait ten seconds, then plug it into a blue or USB-C port using its own cable. If it stays like this, the drive inside may have come loose or failed.

![The Drives window. A sentence at the top tells you a made-up drive called WD Elements needs a look and names its damaged spots. The drive card repeats it, with a round help button.](img/drives-needs-a-look.png "What you will see: a drive that needs a look.")

If a drive gives no health reading, Drives shows none. That isn't a fault.

:::expert
Drives gets health information from the drive's SMART report through udisks2. For ATA drives it uses the failing flag, the damaged sectors and the number of failing attributes. For NVMe drives it uses the critical warnings and the percentage of life used (100 or more counts as "worn past what its maker expects"). Drives never reads more into the report than the drive reports.
:::

## The computer is nearly full
Questions: why does it say nearly full; why does it say delete or move some files soon; how much space is left; what is the headline at the top of Drives; why does it say getting full

The sentence at the top of Drives tells you how much room is left where your own files are stored. If your files have no place of their own, it's the system's disk. The sentence turns red when less than half a gigabyte is free, or when 95 per cent is used.

![The Drives window. A red sentence at the top tells you the computer is nearly full, with only 400 MB free, and a round help button sits beside it. Two rings below show the system and your files, the second one full and red.](img/drives-nearly-full.png "What you will see: the computer nearly full.")

What to do:

1. Open Folders and look for large files you no longer need. Empty the Trash.
2. Move big files, such as videos, to a USB drive.
3. Remove apps you don't use, in Apps.

An amber sentence, "Getting full", appears earlier, at 85 per cent used or when less than 5 GB is free. It's a warning, not a fault.

:::expert
The figures are the filesystem's own counts of used and free blocks. Drives says nearly full when the used share is at least 95 per cent or less than 500 MB (decimal) is free. It shows "getting full" at 85 per cent or when less than 5 GB is free.
:::

## Hiding a drive was not kept
Questions: why does it say that could not be kept; why does the eye not work; where does Drives remember which drives are hidden; why is a drive still showing in Folders

The eye on a drive or a part hides it from the list and from Folders. Drives remembers your choice in a small file, `hidden-drives.conf`, in your own settings folder. If Drives can't write that file, the choice is lost, and Drives tells you.

![The Drives window. On the card of a made-up drive called SanDisk Cruzer, red words say that could not be kept and why, with a round help button. The eye that hides the drive is in its row of buttons above.](img/drives-hide-failed.png "What you will see: a hidden drive that was not kept.")

- **"That could not be kept: ..."** The reason follows, such as a full disk or a read-only settings folder. Free up some room on the system's disk, or fix the settings folder, then press the eye again.

Hiding doesn't change anything on the drive and doesn't ask for a password, so the drive itself is unchanged.

:::expert
Drives saves the hidden drives one to a line. It identifies each drive by what the drive reports about itself (an identifier made from its model and serial number), never by its device name or its port. Two drives that report exactly the same details are hidden together. A drive that gives nothing to identify it by has no eye.
:::
