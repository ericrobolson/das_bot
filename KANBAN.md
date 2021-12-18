Doing - 2
* Implement simple interpreter that queues up things on the timeline
* * Add loading of a program
* * Write an interpreter to read in and create keys + times from a script, Lisp based with with Elixir syntax
* * Add ability to define methods
* * Add method to load + execute other scripts
* * Add ability to start a process, wait for it to load, then execute things

Todo 
* Simplify execution of generator
* Do mouse movement
* Wire up `SendMessage`
* Abstract the two
* [This book on hacking](https://books.google.com/books?id=h4-7DQAAQBAJ&pg=PA211&lpg=PA211&dq=games+with+apis+you+can+use+to+play+against+other+bots&source=bl&ots=12zOqoAKE_&sig=ACfU3U29YcWJcVgYSPVCxJ1fUMw-wTpHOg&hl=en&sa=X&ved=2ahUKEwiG0ZO6ou70AhXiHTQIHcAzBwEQ6AF6BAgYEAM#v=onepage&q=games%20with%20apis%20you%20can%20use%20to%20play%20against%20other%20bots&f=false)
* Add ability to read memory and then do conditional things


Done
* Make generic keys based off of an enum
* Make a 'timeline' generator, which allows you to queue up key presses with a delay as well
* Wire up `SendInput` to send a single key
* Wire up Windows API
