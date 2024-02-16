# Revise 

Revise is a command-line program used to schedule the review of items using spaced repetition. Like other spaced-repetition software (Anki, Mnemosyne), the scheduling algorithm is based on the SM-2 algorithm. Unlike other spaced-repetition software, this is not flashcard-based. An "item" in "revise" is just a description of the thing you want to review. The actual information to be reviewed is assumed to be elsewhere (in a text file somewhere, or in some note-taking software, or written down in a notebook, or maybe carved into clay tablets).

``` sh
# Create a new deck
$ revise create-deck leetcode
$ revise list-decks
1. leetcode

# Add cards to the deck
$ revise add 1 "Two Sum"
$ revise list # list cards to review
1. Two Sum

# Revise cards from deck or individually
$ revise review 1 
$ revise review-card 1 

# List all cards
$ review list --all
```