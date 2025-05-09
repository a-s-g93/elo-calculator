# Elo Calculator

An elo calculator built in Rust.

Elo scores represent the relative skill of a player or team in a game. The amount of points awarded is based on the difference in scores of the participants. For example if a player with a low elo beats a player with a high elo, the player with the low elo will receive a large gain, while the other player will lose the same number of points. 

This elo calculator allows for 1v1 as well as multiplayer match calculations. For multiplayer matches, each player match up is calculated and the sum total elo change is applied. 

---

## Implementations

### PyPI Package

```bash
pip install elo-calculator
```
Python bindings expose access to the primary classes and functions
* **Entry** &rarr; represents a player or team
* **quick_calc** &rarr; calculate new elos between two players without using Entry object
* **update_elos_for_group** &rarr; calculates the new elos for a single match
* **update_elos_for_sequence** &rarr; calculates the changes in elos for a sequence of matches 

```python

from elo_calculator import Entry, update_elos_for_group, update_elos_for_sequence, quick_calc

k = 32 # this determines the scale of change applied

# create our entries using the Entry object
a = Entry(id="1", name="dk", place=1, input_elo=1234)
b = Entry(id="2", name="toad", place=2, input_elo=888)

# calculate new elos for our event
res = update_elos_for_group([a, b], k)

print(res)
# [
#   Entry(id='1', name='dk', place=1, input_elo=Some(1234), output_elo=Some(1238)), 
#   Entry(id='2', name='toad', place=2, input_elo=Some(888), output_elo=Some(884))
# ]

# add another event with the same participants
# note that the `input_elo` only needs to be provided if it is the first occurance of the entry id
a2 = Entry(id="1", name="dk", place=1)
b2 = Entry(id="2", name="toad", place=2)

res2 = update_elos_for_sequence([[a,b], [a2,b2]], k)

print(res2)
# [
#   [
#       Entry(id='1', name='dk', place=1, input_elo=Some(1234), output_elo=Some(1238)),
#       Entry(id='2', name='toad', place=2, input_elo=Some(888), output_elo=Some(884))
#   ],
#   [
#       Entry(id='1', name='dk', place=1, input_elo=Some(1238), output_elo=Some(1242)),
#       Entry(id='2', name='toad', place=2, input_elo=Some(884), output_elo=Some(880))
#   ]
# ]

# simple function for quick calculations without the Entry object
print(quick_calc(1000, 1234, k))
# (1025, 1209)

```

### Cargo Crate (Unreleased)

The native Rust crate may be used via the same struct and functions named above

### Elo Calculator Server

A server built with the `actix_web` crate that exposes endpoints for calculating the elo changes for a single match or a sequence of matches. To deploy:
* Navigate to `root/api/`
* From the CLI run `cargo run`


```
// Example Request
{
    "k": 32,
    "elo_entry_sequence": [
        [
            {
                "id": "a",
                "name": "Donkey Kong",
                "place": 1,
                "input_elo": 1234
            },
            {
                "id": "b",
                "name": "Mario",
                "place": 2,
                "input_elo": 1000
            }
        ],
        [
            {
                "id": "a",
                "name": "Donkey Kong",
                "place": 1,
            },
            {
                "id": "c",
                "name": "Luigi",
                "place": 2,
                "input_elo": 1234
            }
        ]
    ]
}

// Example Response

[
    [
        {
            "id": "a",
            "name": "Donkey Kong",
            "place": 1,
            "input_elo": 1234,
            "output_elo": 1241
        },
        {
            "id": "b",
            "name": "Mario",
            "place": 2,
            "input_elo": 1000,
            "output_elo": 993
        }
    ],
    [
        {
            "id": "a",
            "name": "Donkey Kong",
            "place": 1,
            "input_elo": 1241,
            "output_elo": 1257
        },
        {
            "id": "c",
            "name": "Luigi",
            "place": 2,
            "input_elo": 1234,
            "output_elo": 1218
        }
    ]
]
```

### CLI

A command line interface that allows for single match calculation with minimal participant details

