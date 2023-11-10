import os

FILENAME = "text.txt"

megafile = open(FILENAME, "w")
megafile.write("copyright wikipedia.org\n\n")

def filter(line):
    filtered = ""
    depth = 0
    for c in line:
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
        if depth <= 0 and (c.isalpha() or c in [" ", ",", ".", "/", ";"]):
            filtered += c.lower()
    return filtered

for filename in os.listdir("."):
    if filename.endswith(".txt") and filename != FILENAME:
        content = open(filename, "r").read()
        for line in content.splitlines():
            if line != "" and not line.startswith("=="):
                megafile.write(filter(line) + "\n\n")
            if line in ["== See also ==", "== Notes ==", "== References =="]:
                break

for filename in os.listdir("."):
    if filename.endswith(".txt") and filename != FILENAME:
        os.remove(filename)
