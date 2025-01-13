This project is an exploration into rust and bevy, relying heavily on egui for visual representation.
I'm working on it often, but the core mechanics of the "game" will never change.

"Once in a lifetime" is a game my family used to play on vacation when we were bored enough to do something, but too lazy to do something that required thought.
It's not even a game really, it's just shuffling cards and hoping they line up right.

The story is that getting a deck shuffled in such a perfect way by chance alone is so rare you'll only win "Once in a lifetime".
However, after simulating this game a few million times, It appears to be around 1/1000 games.
Considering it takes about 60 seconds to shuffle and play a round with a real deck, I think the name is a misnomer.

Rules:
Shuffle the deck, and draw cards one at a time.
Add the newly drawn card to the top of your hand.

If the card on "top" and the card "4th" in your hand having matching suits, discard the "2nd" and "3rd" card. 
(Check your hand again! Your new top card and new 4th card may match in some way!)

If the card on "top" and the card "4th in your hand have matching numbers/royalty, discard the top 4 cards in your hand.
Once the deck is empty, if your hand is also empty, you win!


TODOs:
Preload images
Refactor everything
Stack images of cards > position 4 in the hand
Make top lane GUI prettier
Animations?
