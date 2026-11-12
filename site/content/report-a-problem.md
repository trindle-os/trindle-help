---
title: Report a problem
summary: Tell Trindle's community what went wrong, and see exactly what is in the report before anything is sent.
group: Help
order: 10
---

If something goes wrong, you can write it down in a small window and send it to Buildmorphic, the community that helps build Trindle. Nothing is sent until you press Send.

## Switching it on

Reports are off until you switch them on. Switch on "Ask about problems and sharing reports" in Settings, Crash reports, or answer "Yes, ask me" in the first-run tour. While reports are off, the ring doesn't show the Report a problem button, Trindle doesn't ask you anything after a crash, and nothing is sent.

## How to make a report

1. Open the ring and press the Report a problem button in its lower row.
2. In the box "What were you doing?", type what you were doing.
3. Press Show details to see the whole report, word for word.
4. Press Send. Or press Cancel to close the window without sending anything.

If you press the ring button a second time, it brings the first window to the front instead of opening another.

Show details also has Save, which saves the report in your Downloads folder, and Copy, which copies the same text to the clipboard.

## What a report holds

The line in the window tells you what is sent: your computer model, graphics card and the error, never your files, passwords or Wi-Fi names. Show details shows all of it before you press Send.

- Your own words.
- The last 200 lines of Trindle's own log. A log is a written record of what the computer's programs have done.
- The Trindle version.
- The computer: its model, graphics card, processor and memory.
- After a crash, which program stopped and where it stopped.
- Your Buildmorphic username or GitHub name, only if you filled them in on the Crash reports page.

Nothing else.

![A diagram of two routes. In the Report a problem window, you press Send to post the report to Buildmorphic, or you press Save or Copy under Show details. After a crash, Trindle asks whether to send a report, and you press Send or Don't send.](img/report-flow.svg "What you will see: a diagram of where reports go. It's a drawing, not a screenshot.")

## What is blanked

Before Trindle shows, saves or sends the text, it blanks out anything that names you.

- Your home folder becomes `~`.
- Any other home folder becomes `/home/[name]`.
- Your login name, the words of your full name and the computer's name become `[name]`.
- Wi-Fi names, network addresses and hardware addresses are blanked.
- Every email address becomes `[email]`.

The same goes for your own words.

## Where it goes

Send posts the report to Buildmorphic. The window first checks whether reports are being taken, and only then shows Send. While it checks, it shows "Checking." If reports aren't being taken, it shows "Reports are not being taken yet." where Send would be.

If there's no connection, a report you sent waits on this computer. It goes on its own when the connection comes back. The window shows "Waiting for a connection."

## What the window says when something is wrong

Messages appear right where you are working.

- "Write a few more words to say what happened." appears if you haven't written enough to send.
- "There is no log to send with it, so it cannot be sent. Save it instead." appears if Trindle can't read its log.
- "Already sent." and "Already saved." appear once you have sent or saved the report.
- "Could not save it", with a few words about why, appears if Trindle can't write the file.

## When a program crashes

When one of Trindle's own programs crashes, you get a notification. If reports are switched on, a window asks "Something went wrong. Send a report?" with Send and Don't send. It asks once for each crash.

Crashes of apps stay on the computer. Trindle never sends them.
