# Design process

## Brainstorming

### 2023/10/18
Problem statement: Qwerty is a very unoptimized keyboard layout. By creating a better layout for myself and putting it onto a real keyboard, I could become a much faster typist. I plan to do a number of different tests before running simulating the speed of keyboard layouts, including testing speeds of different keypresses for different people to create a genereal picture of how fast the different keys are pressed.

### 2023/10/20
I am current working on a method to test how fast different fingers are. My plan is to use the information I have about where different fingers belong on the keyboard and test how fast they can move between those keys. I will use this information later to create a realistic simulation of how fast a keyboard layout is for typing a specific text. Here is an example of what a test would look like.

![Keyboard Test](<images/Screenshot from 2023-10-20 22-43-21.png>)

I will continue to work on this test until I have some strong data to help me create the ultimate keyboard layout.

### 2023/10/24
After finishing the test, I realize that I approached it the wrong way. The test currently works by having the player hover their finger over the start key and then moving it to the end key when it turns purple. However, this is 
1. Boring as hell
2. Judges reaction time instead of finger speed.

In the coming weeks, I will rewrite the test to have the results better represent the finger speed of the user.
