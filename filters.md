# Filters
Both endpoints use the same filter

The json signature described in typescript:
```ts
type CardFilter = {
	// ---------------------------------------------
	// ------------------ paging -------------------
	// ---------------------------------------------
	page?: number; // int to select result page 
	count?: number; // int to choose page size


	// ---------------------------------------------
	// ------------------ filters ------------------ 
	// ---------------------------------------------
	// they are combined with an 'and' between all filters
	// if a filter is null, it's ignored
	// if it's a single value, it's a simple filter
	// if it's an array, it becomes an 'or' between array values

	// the filters bellow are done using a lower + like
	// so they don't need to be an exact match
	// and they are case insensitive
	name?: string | string[];
	text?: string | string[];
	flavorText?: string | string[];
	manaCost?: string | string[];

	// the filters bellow are done using a lower + not like
	// so they don't need to be an exact match
	// and they are case insensitive
	// they are the same as the fields above, but negated
	nameNot?: string | string[];
	textNot?: string | string[];
	flavorTextNot?: string | string[];
	manaCostNot?: string | string[];
	
	// the filters bellow are done using equality
	// so they need to be an exact match
	power?: string | string[];
	toughness?: string | string[];
	loyalty?: string | string[];
	manaValue?: number | number[];
	artist?: string | string[];
	borderColor?: string | string[];
}
```

[back to main page](./README.md)