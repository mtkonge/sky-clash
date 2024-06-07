# api

## Models

```ts
enum HeroType {
    Centrist = 0,
    Strong = 1,
    Speed = 2,
    Tankie = 3,
}

type HeroStats = {
    strength: number,
    agility: number,
    defence: number,
}

type Board = {
    hero_1_rfid: string | null,
    hero_2_rfid: string | null,
}

type RgbColor = [number, number, number]

type BoardColors = [RgbColor, RgbColor]
```

## POST /create_hero

```ts
type RequestBody = {
    rfid: string,
    hero_type: HeroType,
    base_stats: HeroStats,
}
```

```ts
type Response = {
    status: 201 | 400 | 500,
    body: undefined,
}
```

## POST /update_hero_stats

```ts
type RequestBody = {
    rfid: string,
    stats: HeroStats,
}
```

```ts
type Response = {
    status: 200 | 400 | 500,
    body: undefined,
}
```

## GET /hero/{rfid}

```ts
type RequestParams = {
    rfid: string,
}
```

```ts
type Response = {
    status: 200,
    body: {
	id: number,
	kind: HeroKind,
	rfid: String,
	level: number,
	strength_points: number,
	agility_points: number,
	defence_points: number,
    },
} | {
    status: 404,
    body: null,
} | {
    status: 400 | 500,
    body: undefined,
}
```

## POST /update_heroes_on_board

```ts
type RequestBody = Board;
```

```ts
type Response = {
    status: 200 | 400 | 500,
    body: undefined,
}
```

## GET /heroes_on_board

```ts
type Response = {
    status: 200,
    body: Board,
} | {
    status: 400 | 500,
    body: undefined,
}
```

## POST /update_board_colors

```ts
type RequestBody = {
    hero_1_color: RgbColor,
    hero_2_color: RgbColor,
}
```

```ts
type Response = {
    status: 200 | 400 | 500,
    body: undefined,
}
```

## GET /board_colors

```ts
type Response = {
    status: 200,
    body: BoardColors,
} | {
    status: 400 | 500,
    body: undefined,
}
```

## POST /create_match

```ts
type RequestBody = {
    winner_hero_id: number,
    loser_hero_id: number,
}
```

```ts
type Response = {
    status: 200 | 400 | 500,
    body: undefined,
}
```
