# Banjo-Kazooie XBLA patcher

This executable patches an uncrypted default.xex from a .json file. Later a UI will be made to generate a compatible .json file, and therefore becoming a randomizer.

No checks are done on the data, so you can set that one needs more than one jiggy to open MM and therefore softlocking the game. Or spawn 5,000 grublins and probably crash the game.

## Usage

```console
$ bk360-rando-patcher <file>.xex <file>.json
```

## JSON

```json
{
	"transformations": {
		"termite": 5,
		"croc": 10,
		"walrus": 15,
		"pumpkin": 20,
		"bee": 25
	},
	"mm": {
		"huts": [
			{
				"actor": 81,
				"count": 5
			},
			{
				"actor": 82,
				"count": 5
			},
			{
				"actor": 6,
				"count": 1
			},
			{
				"actor": 98,
				"count": 1
			},
			{
				"actor": 73,
				"count": 1
			}
		]
	},
	"puzzles": {
		"mm": 1,
		"ttc": 2,
		"cc": 5,
		"bgs": 7,
		"fp": 8,
		"gv": 9,
		"mmm": 10,
		"rbb": 12,
		"ccw": 15,
		"dog": 25,
		"dh": 4
	},
	"note_doors": [50, 180, 260, 350, 450, 640, 765, 810, 828, 846, 864, 882]
}
```

* transformations: The cost of each transformations
* mm.huts: the actor that is spawned on destruction and how many to spawn
* puzzles: number of jiggies necessary to open a level
* note_doors: number of notes needed to open the doors
