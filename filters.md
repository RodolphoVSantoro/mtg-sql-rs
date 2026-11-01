# Filters
Both endpoints use the same filter

You can pass a page and count as http query params

Count can be up to 100

If count * page is higher than the total, it returns an empty array

Example:

`<url>/card?page=1&count=50`

### The body json signature described in typescript:
The body's type should be CardFilter
```ts
type NumberFilter = 'manaValue';
type StringFilter = 
    'name' |
    'text' |
    'flavorText' |
    'keywords' |
    'manaCost' |
    'power' |
    'toughness' |
    'life' |
    'loyalty' |
    'power' |
    'toughness' |
    'loyalty' |
    'colors' |
    'colorIdentity' |
    'type' |
    'types' |
    'subtypes' |
    'supertypes' |
    'borderColor' |
    'artist' |
    'leadershipSkills' |
    'printings';

type LikeFilter = {
	operation: "like",
	field: StringFilter,
	value: string,
};
type EqualFilter = {
	operation: "equal",
	field: StringFilter,
	value: string,
} | {
	operation: "equal",
	field: NumberFilter,
	value: number,
};
type NotFilter = {
	operation: "not",
	filter: CardFilter,
};
type AndFilter = {
	operation: "and",
	filters: Array<CardFilter>,
};
type OrFilter = {
	operation: "or",
	filters: Array<CardFilter>,
};

type CardFilter = OrFilter | AndFilter | NotFilter | LikeFilter | EqualFilter;
```

### Example:
A query looking for "(mana value not equal 3) and (name like medallion or flavorText like ruler)"
```json
{
    "operation": "and",
    "filters": [
        {
            "operation": "or",
            "filters": [
                {
                    "operation": "like",
                    "field": "name",
                    "value": "medallion"
                },
                {
                    "operation": "like",
                    "field": "flavorText",
                    "value": "ruler"
                }
            ]
        },
        {
            "operation": "not",
            "filter": {
                "operation": "equal",
                "field": "manaValue",
                "value": 3
            }
        }
    ]
}
```

[back to main page](./README.md)