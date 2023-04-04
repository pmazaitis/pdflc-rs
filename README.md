# pdflc-rs

A simple command line interface PDF link checker.

The `pdflc` command will take a PDF, look for links within the text content areas, and then poll those links and report the status offered by the web server for those pages.

`pdflc` reports the links it found and the status of those links on the temrminal.

With the flag `-o`, a filename can be given for saving the output as a CSV file.

## Usage

List the links in the file _example.pdf_, and report the status of each link:

```
pdflc example.pdf  
```

Generate a list of links and link status in the file _example.pdf_ and report the list in the CSV file _example.csv_:

```
pdflc example.pdf -o example.csv
```

## Notes and Warnings

**Warning:** this utility will look for links in a PDF _and then poll that link to see if it's active_. Be aware that the activity of this utility will appear in server logs. Always be kind to server administrators: avoid using this tool with PDFs that contain lots of links if the amount of traffic may overwhelm a swebserver.

**Note:** This is a _minimal_ release of this software:

 * The error handling isn't.
 * Only CSV is supported as an output.
 * The documentation could (as always) use some work.

But it does a small job reasonably; may it be useful! 