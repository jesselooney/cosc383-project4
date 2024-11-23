= Process

== Phase 1

When approaching this project during phase 1, the first thing we decided we needed was a general API that could gather and modify specific bits in an image in an ergonomic way. We started development in Python, and quickly settled on an API where users could pass in a lambda function which would specify which bits get manipulated. All of this code can be found on the `main` branch of our repository, which can be found #link("https://github.com/jesselooney/cosc383-project4/tree/main")[here]. We then developed some tooling which would amplify specific least significant bits of an image, which would let hidden messages be detected with the human eye.

However, we quickly ran into a performance problem, especially with our amplification tooling. Transformations would take upwards of minutes to complete, especially on larger images, due to having to iterate over every bit, of every channel, of every pixel.

So we took the logical choice and switched to Rust. We quickly rewrote the simple API we had so far in a vastly superior language, and with that our time in phase 1 was almost up.

You can find this rewritten code base #link("https://github.com/jesselooney/cosc383-project4/tree/rust")[here].

== Phase 2

We started phase 2 with some basic reconnaissance, trying to get a basic idea of what images had data encoded in them, and in what way. We used our amplification tooling to amplify each bit in the image, and this gave us a rough idea which images had data encoded in which bits, and what orientation that data was stored in. You can find our detailed notes from this #link("https://github.com/jesselooney/cosc383-project4/blob/rust-refactor/src/decode.rs")[here].


After this, we decided our codebase needed some refactoring (we _are_ programmers after all, rewriting the whole codebase is a completely logical thing to do). Our API came short in a couple ways, mainly in that fact that it was incredibly manual, and also lacked the ability to iterate top to bottom, or bottom to top.

This is our current codebase, and it can be found #link("https://github.com/jesselooney/cosc383-project4/tree/rust-refactor")[here].

The new codebase implements a new, more flexible data extraction API that allows for iteration in different directions across rows and columns. Using this, we implemented many different automatic detection methods. The current methodology tries iterating over the image as many ways as possible, and then sees which ones return valid data. We check both for reasonable sounding headers, and data that looks like text.

= Findings

== Sample Images

Near the end of phase 1 we managed to use our tooling to successfully decode one of the sample images, and found incomplete instructions on how the telepathy trick worked.

== Duplicate Images

Many of the images simply contained rotated or mirrored, and then downscaled, versions of themselves. These images include:
- `Acorn.png`
- `Dreamm.png`
- `Friendship.png`
- `Phishing.png`
- `ProfessorAlfeld.png`
- `RobotOnRealCat.png`
- `Steganography.png`
- `Teach.png`
- `Spyware.png`
- `Security.png`

== Encryption Key

When searching inside `Abominable.png` we found what looked like a node graph encryption key. We have yet to find the data that it encrypts, but we know from some of the other pieces of text we found that it encrypts something that will tell us Pr0Hax0r's favorite color.

== Pokemon Choosing

While searching inside `Cookies.png` we found what looks like a Pokemon trainer looking at three Poke Balls. We don't know what this means, but it might pertain to the question about Pr0Hax0r's preferred Pokemon.

== Text

We found the following snippets of text in various images:

```
To revert hex into executable file:
xxd -r -p hexnumbers.txt > backtoexec.txt
```

```
Roses are red,
Black roses are black,
Now add the right nodes
To get the message back.
```

```
P.S. I tried encrypting my favorite color with the graph to test it.
I jot down the node values somewhere, but I can't find it anymore...
```

```
From: Scott Alfeld <salfeld@amherst.edu>
Date: Mon, 14 Oct 2024 09:52:42 -0400
Subject: [COSC383F24] Final Exam
To: Soyon Choi <sochoi25@amherst.edu>
Content-Type: text/plain; charset="UTF-8"

Can you take a gander at this and let me know what I should use for this year's final?
Link: https://tinyurl.com/cosc383finalf22

~SA
[the link goes to "Ylvis - The Fox (What Does The Fox Say?) [Official music video HD]"]
```
