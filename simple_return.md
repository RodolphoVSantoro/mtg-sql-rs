# Simplified return type

The return type for the /card endpoint described in typescript
```ts
type CardSimple = {
    name?: string;
    mana_value?: number;
    mana_cost?: string;
    keywords?: string;
    text?: string;
    power?: string;
    toughness?: string;
    life?: string;
    loyalty?: string;
    colors?: string;
    color_identity?: string;
    type?: string;
    types?: string;
    subtypes?: string;
    supertypes?: string;
    border_color?: string;
    availability?: string;
    flavor_text?: string;
    artist?: string;
    leadership_skills?: string;
    printings?: string;
}
```

[back to endpoints](./endpoints.md)