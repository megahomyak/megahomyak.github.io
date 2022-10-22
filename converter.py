import re
import bs4
import os
from markdown import markdown


INPUT_NAME = "index.data"


def process():
    root = os.environ["ROOT"]

    with open(INPUT_NAME, encoding="utf-8") as f:
        contents = f.read()
    tags = {
        tag_name.casefold(): tag_contents.strip()
        for tag_name, tag_contents in re.findall(
            r"\<(?P<tag_name>.+?)\>(?P<tag_contents>.*?)\</\>", contents, re.DOTALL
        )
    }

    def nullable(function):
        def wrapper(*args, **kwargs):
            if all(arg is not None for arg in args + tuple(kwargs.values())):
                return function(*args, **kwargs)

        return wrapper

    @nullable
    def parse_tuple(s):
        return tuple(map(int, s.split()))

    @nullable
    def bias(seq, n):
        return tuple(map(lambda i: i + n, seq))

    def tuple_to_hex_color(seq):
        return "".join(hex(int(max(0, min(n, 255))))[2:].rjust(2, "0") for n in seq)

    main_color = parse_tuple(tags.get("main color"))
    background_color = parse_tuple(tags.get("background color")) or bias(
        main_color, +215
    )
    content_color = parse_tuple(tags.get("paragraph color")) or bias(main_color, -50)

    if main_color is None and (background_color is None or content_color is None):
        raise Exception("Colors are not set")

    contents = tags["contents"]
    contents = re.sub(
        r"<slogan>(.+?)</slogan>",
        r'<p class="horizontallyCentered"><em>\1</em></p>',
        contents,
        re.DOTALL,
    )
    contents = re.sub(
        r"<filler>(.+?)</filler>",
        r'<div class="tight centeredContents takesAllSpace"><p class="bigText tight">\1</p></div>',
        contents,
        re.DOTALL,
    )
    contents = contents.replace(
        "{contact me}",
        '<button class="contactMe"><img src="../telegram_logo.svg" alt="Telegram logo"><span>Contact me on Telegram</span></button>',
    )

    markup_type = tags["markup type"]
    title = tags.get("title")

    if markup_type == "markdown":
        body = markdown(contents)
    else:
        raise Exception("Unknown markup type")
    soup = bs4.BeautifulSoup(body, features="html.parser")

    if title is None:
        title = soup.find(re.compile("^h[1-6]$"))
        if title is not None:
            title = title.text
        else:
            raise Exception("No title found")

    with open(os.path.join(root, "styles.data"), encoding="utf-8") as f:
        styles_string = (
            f.read()
            .replace("{background color in hex}", tuple_to_hex_color(background_color))
            .replace("{text color in hex}", tuple_to_hex_color(content_color))
        )

    styles = []
    for line in styles_string.split("\n"):
        if line.strip():
            name, value = line.split("=", 1)
            styles.append((name.strip(), value.strip()))

    classes = set()
    tag_names = set()
    for element in soup.find_all():
        try:
            classes.update(element["class"])
        except KeyError:
            pass
        tag_names.add(element.name)

    styles_list = []
    for name, value in styles:
        trimmed_name = name.split(":")[0].split(">")[0].strip()
        if (
            (trimmed_name.startswith(".") and trimmed_name[1:] in classes)
            or trimmed_name in tag_names
            or trimmed_name == "body"
        ):
            styles_list.append(f"{name} {{{value}}}")

    anchors = set()

    def escape_heading(text):
        text = re.sub(r"\s", "-", text)
        text = re.sub("[^abcdefghijklmnopqrstuvwxyz1234567890]", "_", text.casefold())
        return text

    for heading in soup.find_all(re.compile("^h[1-6]$")):
        escaped_name = escape_heading(heading.text)
        anchor_number = 1
        while True:
            anchor_name = escaped_name
            if anchor_number != 1:
                anchor_name += f"-{anchor_number}"
            if anchor_name in anchors:
                anchor_number += 1
            else:
                break
        anchors.add(anchor_name)
        link = soup.new_tag("a")
        link.attrs["href"] = "#" + anchor_name
        heading.attrs["id"] = anchor_name
        link.contents = heading.contents
        heading.contents = [link]
    body = str(soup)

    styles = "\n".join(styles_list)

    html = f"""<html><head><style>\n{styles.strip()}\n</style><title>{title.strip()}</title></head><body>\n{body.strip()}\n</body></html>"""

    with open("index.html", "w", encoding="utf-8") as f:
        f.write(html)


try:
    process()
except Exception as e:
    print(f"-- At {os.getcwd()}/{INPUT_NAME}:")
    raise e
