#!/usr/bin/env python3

# Original script by @atluft with inspiration from http://link.medium.com/0D6TnUKTIab located at:  # noqa: E501
# https://github.com/o2sh/onefetch/wiki/ASCII-Art-From-Image-Using-Python-Image-Library/9c454b390273ffedd60db9d525fb001f89d581b1

# lint with
# flake8 --max-line-length 88 image_to_ascii.py
# black --color --diff image_to_ascii.py
# isort --color --diff image_to_ascii.py

from argparse import ArgumentParser
from math import floor

from PIL import Image, ImageDraw

__version__ = "1.0.0"

parser = ArgumentParser(description="convert an image file to ASCII art")
requiredNamed = parser.add_argument_group("required argument")
requiredNamed.add_argument(
    "-c",
    "--characters",
    help='characters available to be used, default " .:_-"',
    default=" .:_-",
)
requiredNamed.add_argument(
    "-f",
    "--file",
    help="image file to convert to ASCII",
    required=True,
)
parser.add_argument(
    "-r",
    "--resolution",
    nargs=2,
    metavar=("height", "width"),
    help="set custom resolution, default 25 45",
    type=int,
    default=[25, 45],
)
parser.add_argument(
    "-s",
    "--save_image",
    help="optionally save ASCII to image file",
    action="store_true",
)
parser.add_argument(
    "-v",
    "--version",
    help="show version",
    action="version",
    version="image_to_ascii.py {}".format(__version__),
)
args = parser.parse_args()

image = Image.open(args.file)
scaleFac = 1.0
charWidth = int(args.resolution[1])
charHeight = int(args.resolution[0])
w, h = image.size
image = image.resize((charWidth, charHeight), Image.NEAREST)
w, h = image.size
pixels = image.load()

outputImage = Image.new("RGB", (charWidth * w, charHeight * h), color=(0, 0, 0))
draw = ImageDraw.Draw(outputImage)


def getSomeChar(h):
    charArr = list(args.characters)
    mul = len(charArr) / 256
    return charArr[floor(h * mul)]


for i in range(h):
    for j in range(w):
        # REF: https://stackoverflow.com/a/52667876
        p = pixels[j, i]
        r, g, b = p[0], p[1], p[2]
        grey = int((r / 3 + g / 3 + b / 3))
        pixels[j, i] = (grey, grey, grey)
        draw.text((j * charWidth, i * charHeight), getSomeChar(grey), fill=(r, g, b))
        print(getSomeChar(grey), end="")
    print("")

if charHeight > 25 or charWidth > 45:
    print(
        "WARNING: Custom resolution exceeds maximum 25*45 (height*width). See: \n"
        "https://github.com/o2sh/onefetch/blob/master/CONTRIBUTING.md#ascii-logo"
    )


def fileRename(file):
    fileSpilt = file.split(".")
    outputFile = ".".join((fileSpilt[0:-1])) + "-ascii." + fileSpilt[-1]
    return outputFile


if args.save_image:
    asciiFile = fileRename(args.file)
    outputImage.save(asciiFile)
    print("INFO: image saved as {}".format(asciiFile))
