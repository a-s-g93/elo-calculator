# Elo Calculator

An elo calculator built in Rust.

Elo scores represent the relative skill of a player or team in a game. The amount of points awarded is based on the difference in scores of the participants. For example if a player with a low elo beats a player with a high elo, the player with the low elo will receive a large gain, while the other player will lose the same number of points. 

This elo calculator allows for 1v1 as well as multiplayer match calculations. For multiplayer matches, each player match up is calculated and the sum total elo change is applied. 

---

## Implementations

### PyPI Package (Unreleased)

Python bindings expose access to the primary classes and functions
* **Entry** &rarr; represents a player or team
* **update_elos_for_group** &rarr; calculates the new elos for a single match
* **update_elos_for_sequence** &rarr; calculates the changes in elos for a sequence of matches 

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

