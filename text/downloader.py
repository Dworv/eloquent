# RUN FROM THIS DIRECTORY

s = """97	Princess Margaret, Countess of Snowdon	77
97	John Cena	77
97	Charles Manson	77
97	Ryan Reynolds	77
97	Brad Pitt	77
97	Vladimir Putin	77
93	Winston Churchill	78
93	Bruce Lee	78
93	Nicki Minaj	78
93	Israel	78
87	Titanic[28]	80
87	Tom Brady	80
87	Jay-Z[29]	80
87	Singapore	80
87	Earth	80
87	Bill Gates	80
85	Manchester United F.C.	81
85	William Shakespeare	81
80	Ted Bundy	82
80	Pablo Escobar	82
80	Mila Kunis	82
80	Vietnam War	82
80	Mark Zuckerberg	82
77	Meghan, Duchess of Sussex	83
77	Muhammad Ali	83
77	Will Smith	83
75	Jennifer Aniston	84
75	Breaking Bad	84
69	Keanu Reeves	85
69	Arnold Schwarzenegger	85
69	How I Met Your Mother	85
69	Chernobyl disaster	85
69	France	85
69	Ariana Grande	85
67	Diana, Princess of Wales	86
67	Marilyn Monroe	86
65	John F. Kennedy	87
65	COVID-19 pandemic[27]	87
63	Queen Victoria	88
63	Jeffrey Dahmer	88
59	Scarlett Johansson	89
59	Lil Wayne	89
59	Tupac Shakur	89
59	Angelina Jolie	89
55	Prince Philip, Duke of Edinburgh	90
55	Harry Potter	90
55	Elvis Presley	90
55	The Walking Dead (TV series)	90
50	Joe Biden	91
50	Tom Cruise	91
50	Rihanna	91
50	Albert Einstein	91
50	Academy Awards	91
48	Kobe Bryant	92
48	Selena Gomez	92
47	Leonardo DiCaprio	93
40	LeBron James	94
40	Charles III[25]	94
40	Darth Vader	94
40	Star Wars	94
40	Miley Cyrus	94
40	Germany	94
40	September 11 attacks[26]	94
38	List of Marvel Cinematic Universe films	95
38	Abraham Lincoln	95
34	Russia	96
34	New York City	96
34	Japan	96
34	Kanye West	96
33	China	100
31	Stephen Hawking	101
31	List of highest-grossing films	101
30	Taylor Swift	102
29	The Big Bang Theory	103
28	List of presidents of the United States	104
26	Michael Jordan	107
26	Australia	107
24	Steve Jobs	108
24	Dwayne Johnson	108
23	Johnny Depp	109
22	Kim Kardashian	111
20	Canada	113
20	Freddie Mercury	113
19	Justin Bieber	114
18	The Beatles	116
17	World War I	121
16	Game of Thrones	122
15	Lionel Messi	125
14	Eminem	127
12	Lady Gaga	129
12	Adolf Hitler	129
11	Sex	132
10	Elon Musk	135
9	Michael Jackson	142
8	United Kingdom	144
7	World War II	145
6	Cristiano Ronaldo	151
5	Barack Obama	161
4	India	165
3	Elizabeth II	198
2	Donald Trump	243
1	United States	254"""

articles = []
for line in s.split('\n'):
    link = line.split('\t')[1].strip().replace(' ', '_')
    articles.append(link)

articles.reverse()

for article in articles:
    import wikipedia
    try:
        wiki = wikipedia.page(article)
    except:
        continue
    print(article)
    open(article.lower() + ".txt", "w").write(wiki.content)
