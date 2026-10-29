# Magic the gathering sql rust server

A rust server that interacts with [this sqlite database](https://mtgjson.com/changelogs/mtgjson-v5/)

This sqlite database is an extensive list of all magic the gathering cards

- Supports [multiple filters](filters.md)
- Has 2 [endpoints](./endpoints.md), one for a simplified card view, and another with the entire data from the database for each card
