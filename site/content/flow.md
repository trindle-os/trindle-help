---
title: Flow
summary: One mouse and keyboard for two Trindle computers. Flow is a plugin and is still being built.
group: Using Trindle
order: 80
---

> Flow is still being built. This page describes what it is meant to do. The list of what makes Trindle different will tell you when it is finished.

- [Read about what makes Trindle different](features.html)

With Flow on two Trindle computers, one mouse and keyboard work on both. Move the pointer off the side of the screen where the other computer sits, and it carries on there. The keyboard and the clipboard go with it. Move the pointer back to return to the first computer.

There is nothing to take a screenshot of yet, so this page uses a diagram of the plan. The diagram is clearly labelled as a plan.

![A diagram of two computers side by side. The pointer leaves the right edge of the first screen and appears on the second.](img/flow-two.svg "What you will see: a diagram of the plan for Flow. Flow isn't built yet, so this is a drawing, not a screenshot.")

## Setting it up

1. Open Apps and go to the Plugins page. Install Flow on both computers.
2. Pair the two computers once, using a short code.
3. In Flow's settings, choose which computer shares its mouse and keyboard. It can be either one.

Nothing is installed until you press Install. Flow has a folder of its own, and it can't do anything until you allow it.

## What you are asked to allow

Flow asks your permission to control your keyboard, mouse and clipboard. Trindle shows a plain warning with this, because these are powerful things to allow. Even when you allow it, Flow can't see your screen or your other windows.

Flow also needs the other computer to be able to reach it. Trindle says so plainly and lists the ports. A port is a numbered entry point that a computer uses to talk to another one. The wording is something like "Let other computers on your home network reach it."

If you allow it, Trindle opens exactly those ports, and only for the computers on your own home network. Trindle closes them as soon as you take the permission away or remove Flow.

To carry a file you copied over to the other computer, Flow asks to read the files you copy. It can then ask Trindle for only the files on the clipboard. It can't see the folders they are in, or your hidden files. It can ask once every two seconds, and only for two minutes after you copy.

## You can always take it back

![A diagram of what Flow may do once you allow it, what it can't do, and three ways to take the permission back.](img/flow-allow.svg "What you will see: a diagram of what Flow may and may not do. Flow isn't built yet, so this is a drawing, not a screenshot.")

- While Flow is controlling your mouse and keyboard, Trindle shows its violet in-use dot at the top middle of every screen. Point at the dot to see that it's Flow. See [The in-use dots](in-use-dots.html).
- When the screen locks, the mouse and keyboard always come back to this computer.
- Hold Ctrl, Alt and Shift together for a second, and the mouse and keyboard come back to this computer, whatever is happening.
- Flow can only take over the mouse and keyboard within a few seconds of you pressing its own shortcut or choosing one of its own menu items.

## Related

- [Apps](apps.html): where Flow is installed.
- [The lock screen](lock-screen.html): what locking the screen does to Flow.
- [Glossary](glossary.html): the words plugin and permission.
