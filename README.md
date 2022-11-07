# 🖌 choose-Your-own-adventure: a game where You choose options to create Your own story built purely in Rust

[<img alt="github" src="https://img.shields.io/badge/github-apc-999" height="20">](https://bitbucket.amazondcl.com/users/anhcowen/repos/choose-your-own-adventure)

## What exactly does choose Your own adventure mean?

Choose Your own adventure is a game (although originally a book series) that is written in a 2nd person point of view, the game allows You to pick different options and ultimately Your journey (or adventure) would change based on the options so no 2 stories are ever the same (unless You pick the same options each time) there is always a chance You could end up at the same place as another option but the story would be slightly different and the ending could be different but You could never go backwards only forwards. The original children's book series was created for 7 to 14 year olds and typically the reader takes on a role. You can find out more about [the history of choose Your own adventure books on Wikipedia](https://en.wikipedia.org/wiki/Choose_Your_Own_Adventure)

My program is designed to be scalable so that anyone can write their own story for anyone to play, this system uses JSON (see below for more info) and is fully modular so You can add as many or as little options as You would like for Your story to work, if You create an invalid JSON there is some error handling but I need to add more if this was to be released as RUST panics quite a lot.

Expected outcomes:

* Each time a player runs the game the program logs the story created so that when the program ends it should output a text file with their story.
* The file is named Story Name - Date and Time in UTC.txt
* The story can be modified, new ones can be created by anyone - using JSON
* Option to choose which story You want to play

Desired outcomes/Things I would love to add if I was releasing it:

* The option for dependencies - the public domain story that I got off the internet requires the game to keep track of the character
* Multiple pre-created stories (Rather than just one unfinished/proof of concept one)
* Have a GUI, either in egui or IgnitionX - because our team uses both of these, although egui is only used for debugging
* Allow users to pause and save their progress so they can continue it afterwards
* Have a program that allows users to write stories without having to edit the JSON files (making it more intuitive for the end user)

## How does the game work?

The game is a terminal based game (although I'd love to have a GUI with it) and goes through the selected adventure (using JSON files to store the adventure) allowing users to pick their options as they go along, the adventure they end up with is written to a txt file at the end so that You can see the journey You took at the end.
``` JSON
{
  "1": {
    "display": "What the story is",
    "options": [{"page": 2,"display": "Can either be null or text"},{"page": 3,"display": "Each option is stored within the array/vector"}]
  }
}
```

## Writing new stories

In an ideal world there would be a program to write stories that is more intutive than editing the JSON file, unfortunately there isn't so here's a simple guide to making Your own story if You want to test the code Yourself (If You have an invalid JSON file, RUST will just panic because I haven't done lots of error handling, just enough for user inputs)
Let's break down this example:

``` JSON
{
  "1": {
    "display": "What the story is",
    "options": [{"page": 2,"display": "Can either be null or text"},{"page": 3,"display": "Each option is stored within the array/vector"}]
  }
}
```

The first value is the page number, this has to be saved as a string because it is defining the dictionary key however RUST will convert it to an integer so it must be a number inside the string not just any string, the first display in the dictionary is what the program should show as the story progresses (and what will be written to the file) The second variable is an array which contains the possible options that the story can progress to, if this is blank this signifies the end of the game and the program will automatically stop, if it has 1 option then it will automatically choose that option and skip to the next page, if it has more than one option it will print out the options in order and let the user choose, if one of the options is null it will print out "To continue, Choose Option #" with hashtag being the number of the option. Each option must contain a page number and display even if the display is null unless there are no options in which case the option needs to be an empty array [] and the program will automatically end, if anything is missing the game may panic.

## Latest Update

* Refactored code - split into seperate files for readability
* New readme file
* Memory dropping to avoid RUST using lots of memory when input checking (as the variable never goes out of scope)
