# Design process

## Brainstorming

#### 2023/10/18
Problem statement: Qwerty is a very unoptimized keyboard layout. By creating a better layout for myself and putting it onto a real keyboard, I could become a much faster typist. I plan to do a number of different tests before running simulating the speed of keyboard layouts, including testing speeds of different keypresses for different people to create a genereal picture of how fast the different keys are pressed.

#### 2023/10/20
I am current working on a method to test how fast different fingers are. My plan is to use the information I have about where different fingers belong on the keyboard and test how fast they can move between those keys. I will use this information later to create a realistic simulation of how fast a keyboard layout is for typing a specific text. Here is an example of what a test would look like.

![Keyboard Test](<images/keyboard.png>)

I will continue to work on this test until I have some strong data to help me create the ultimate keyboard layout.

#### 2023/10/24
After finishing the test, I realize that I approached it the wrong way. The test currently works by having the player hover their finger over the start key and then moving it to the end key when it turns purple. However, this is 
1. Boring as hell
2. Judges reaction time instead of finger speed.

In the coming weeks, I will rewrite the test to have the results better represent the finger speed of the user.

#### 2023/10/30
I have begun working on the new test. I am happy that I am rewriting it, from my experience last time I am able to make it much more organized and less complex. The new test even begins with some instructions to help the user begin.

![Instructions](<images/instructions.png>)

#### 2023/11/1
I FINISHED THE TEST :D. It works very well, and I have already used it on four people, which means that I have 200 samples. However, there are (3*2*6) + (6*5*2) = *96* possible movements that I am trying to measure, so it is imperative that I get much more data. I estimate that I will be able to get maybe 15 more people to do the test, which means I will have around 1000. I may end up just running a lot of tests on myself, as it will be a lot more efficient and I consider myself a pretty normal typer (unlike a lot of my accomplices) In terms of analysis, I will probably try to eliminate as many outliers as I can to normalize the data. 

#### 2023/11/7
After finishing my testing, I have written my data analysis. This involved
- Removing the outliers from the data
- Taking averages for each finger
- Taking averaages for each row
The results of my analysis are in `analysis/output.json`. Now that I have finished my research, I am ready to begin the ideating stage.

## Ideating

#### 2023/11/7
I need to find a keyboard layout that is much better than the Qwerty layout. My plan is to iterate through a lot of different layouts and find which one is the best. I have made a list of things I need to do in order for this to work.
1. Compile a large set of texts to test the keyboard layout on.
   1. English
   2. French
   3. German
   4. ... (more)
2. Create a simulation that tests how fast a keyboard layout can type this dataset.
3. Run this simulation on millions of keyboard layouts and find the fastest one.
This ideating phase was pretty easy for me since I have already thought about how I want this project to work. Thus, this will be the only entry in this section

## Prototyping

#### 2023/11/10
After a little bit of experimenting and scripting, I have finally collected a 3.2 million characters to be used as the speed test. My collection is a compilation of the top 100 wikipedia articles. I copied the list onto my computer, deconstructed it into a list of urls and then downloaded each on onto my computer for filtering. The `text.txt` file is now ready to be used in a simulation. This means that step 1 of my plan is now complete.
- [x] Compile a large set of texts to test the keyboard layout on.
