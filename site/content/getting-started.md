---
title: Getting started
summary: Put Trindle on a USB stick, try it, install it, and find your way round on the first day.
group: Start
order: 10
---

Trindle is a whole operating system. It is not a program you add to the one you have. It replaces what is on the computer, or sits beside it.

> Back up everything that matters before you start. Trindle changes how the computer starts, logs in and looks after its disks. You use it at your own risk.

## What you need

- A computer with a 64-bit Intel or AMD processor.
- At least 20 GB of free disk space. That can be the whole disk, or room made beside another system.
- A USB stick of 4 GB or more. Everything on it is replaced.
- About an hour. It is often quicker, but do not start it ten minutes before you need the computer.

## Five steps from download to desktop

Nothing on the computer changes until the last step.

![A list of five steps: download the image, write it to a USB stick, start from the stick, try Trindle, press Install Trindle.](img/install-steps.svg "What you will see: the five steps in order. This is a diagram, not a screen.")

1. Download the image. It is one `.iso` file, about 1 GB. An image is a copy of a whole system in one file.
2. Write it to a USB stick with Impression, balenaEtcher or `dd`. If you use `dd`, check which device the stick is first. The wrong one wipes the wrong disk.
3. Start the computer from the stick. Plug it in, switch on and hold the boot menu key. That is F12 on most Lenovo and Dell machines, F9 on HP, and Escape or F11 elsewhere.
4. Try Trindle. It runs from the stick with the ring, Apps and Settings all working. It writes nothing to the computer's own disk.
5. Open the ring and press Install Trindle. The installer changes nothing until you press Install on its last page.

If Secure Boot is on, you may need to turn it off first. Secure Boot is a firmware setting that only lets approved software start. Firmware is the small program that runs when you switch on, before the system starts.

Want to try it in a virtual machine first? Give it plenty of memory and turn 3D graphics on. With too little, everything crawls, and then you are judging the virtual machine and not Trindle.

## What the installer asks

The installer asks four questions. Then it gives you a summary in sentences. A mark moves along a line for each step, so you always see where you are. Back works all the way, and nothing is written until you press Install.

![The installer's questions in order: language, the disk, encryption, you, and a last page called Ready.](img/installer-questions.svg "What you will see: the installer's pages in order. It is drawn to explain, not a screenshot.")

- **Language.** Choose it once. The keyboard and the time follow it. There is a box to type in, so you can check the keyboard before you rely on it for a password.
- **The disk.** Use the whole disk and replace everything on it, or keep what is there and share the disk. A slider sets how much room each side gets. Each disk is named by what is on it, so you pick the right one.
- **Encryption.** Encryption scrambles the disk so nobody can read it without the key. You can type a password before anything else appears. Or you can let the computer's TPM chip unlock it by itself. The installer says which of the two the computer can do. You can also leave the disk unencrypted.
- **You.** Your name, the name you sign in with, a name for the computer, and your password twice.

The last page, Ready, says the whole plan back to you. Nothing has changed yet.

![One disk drawn as a bar of three parts: another system that is kept, Trindle, and free space.](img/disk-parts.svg "What you will see: a disk shared by two systems, as an example. It is drawn to explain, not a screenshot.")

### Sharing the disk with another system

If there is a Windows or Linux partition with room in it, the installer offers to keep it and share the disk. A partition is one named part of a disk. The installer makes it smaller and puts Trindle in the space that opens up. Both systems then show in the boot menu.

Before you resize anything, back up the other system. Let Windows shut down properly rather than hibernate. Otherwise its partition cannot be made smaller.

## The first ten minutes

The first time you log in on an installed computer, the Welcome opens before the ring. It asks how much you know first, then shows the pages that fit, with moving pictures and very few words. If you have used Linux before, it leaves out what any Linux desktop does. If you have used Trindle before, it closes and does not come back.

![The first page of the Welcome: a picture of a screen with the ring in the middle, and a card listing what is in the ring.](img/welcome.png "What you will see: the first page of the Welcome. It opens once. It is in All apps as Welcome whenever you want it again.")

Trindle comes almost empty. The Welcome lets you pick the apps you want, such as a browser, email and an office suite. One click installs them all while you read on. There is no browser until you choose one.

Then do these three things.

1. **Join the Wi-Fi.** Open the ring, click the network icon in the middle, switch Wi-Fi on, find your network and press Connect.
2. **Set the screen size.** Open Settings, then Display. If a change looks wrong, do nothing. It puts itself back after fifteen seconds.
3. **Switch locking on.** Open Settings, then Power. Locking is off to begin with. Once it is on, your password unlocks the screen. A four-digit PIN can too, if you choose one.

![The Wi-Fi page of the ring: a switch for Wi-Fi at the top and a list of made-up networks below it.](img/ring-wifi.png "What you will see: the Wi-Fi page of the ring, with made-up network names.")

![The ring open on a dark screen: the time in the middle, small round buttons around it and an app on the outer circle.](img/ring.png "What you will see: the ring, open. You start almost everything from here.")

Help is in the computer too. Settings has a Help page that works with the network off. Every page in Settings has a small i beside its name that opens the part of Help about that page.

## Where to go next

- [The ring](ring.html): how you open everything.
- [Apps](apps.html): finds and installs more programs.
- [Quick answers](quick-answers.html): the twenty questions people ask first.
- [The lock screen](lock-screen.html): how the screen locks and how you get back in.
- [Troubleshooting](troubleshooting.html): for when something does not go to plan.
