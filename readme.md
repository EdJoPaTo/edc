# EMC - EdJoPaTos Media Converter

This tool helps with converting media from one format to another or compressing it.

There are already a bunch of tools out there doing exactly what it needed.
Remembering their command line can be annoying.
Especially when they are lacking features like multiple files at once.

Guiding principles:
- Use other tools (they know already how to do their job best)
- Never override the original
- Provide a way (--dry-run) to export a script to run yourself

## Usage example

Assuming we have a directory full of images (IMG_42.jpg and so on).
You want to compress a bunch of them into a version more suitable for adding them to a webpage.

You can use ImageMagick for that.
But how was the exact syntax again which worked especially well?
Wait, havn't I found something better a few months ago?
That's where this small tool hopes to help:
Simple CLI for the most common tasks.

Lets take this example:
I just took a bunch of photos with my phone which are huge in size and want to have them on the background of another blog post.
I dont want to keep the metadata like location information (`--strip`) and have it in a compressed manner without much visual loss (already done per default).
The input file shouldnt be overriden as I want to keep the original (which is also done by default).

```sh
emc photo --dry-run --strip IMG_42.heic IMG_1337.heic
```

Running this with `--dry-run` shows just the script that would be run.
In this case this is the output:
```sh
mkdir -p converted
convert IMG_42.heic -background black -alpha remove -sampling-factor 4:2:0 -strip -quality 85 converted/IMG_42.jpg
convert IMG_1337.heic -background black -alpha remove -sampling-factor 4:2:0 -strip -quality 85 converted/IMG_1337.jpg
```

You can see: Its just ImageMagick again.
Remove `--dry-run` or pass the lines yourself to a shell and these images are getting compressed.
The result will end up in the `converted` folder (and override everything with the same name).
This folder is, from experience, never there so its fine to have the output ending up in that place.

Thats it, I can now simply take the output files and add them to my blog as backgrounds.

## Ideas and suggestions

Feel free to provide ideas how to improve the tool itself or how to improve compressions and conversions.

Please keep in mind:
This tool was created to simplify my life so I will do what helps me most ;)

If your idea can simplify / improve that even more: Im grateful for that :)
