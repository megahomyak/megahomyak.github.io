from markdown import markdown

with open("index.md", encoding="utf-8") as f:
    contents = f.read()
with open("index.html", "w", encoding="utf-8") as f:
    f.write(markdown(contents))
