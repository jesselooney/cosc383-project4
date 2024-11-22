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

// TODO: I'm planning on writing more about our process for auto detection, especially with the header stuff, and pattern matching/text detection, but I'm planning on reading through more of what you wrote first.

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

== XOR Message

When searching inside `Ideal.png`, we found a square of black and white pixels that looks suspiciously like an image encrypted with a 1 time pad. Unfortunately we're unable to decrypt it without a second message encrypted with the same key.

== Encryption Key

When searching inside `Abominable.png` we found what looked like a node graph encryption key. We have yet to find what it encrypts, or whether its a public or private key, but we definitely have it.

== Pokemon Choosing

While searching inside `Cookies.png` we found what looks like a Pokemon trainer looking at three Poke Balls. We don't know what this means, but it might pertain to the question about Pr0Hax0r's preferred Pokemon.
