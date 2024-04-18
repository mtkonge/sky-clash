### H3-Smart-Kommunikationsplatform - SkyClash

Projekt kan findes her: ["Notion link"](https://mercantec.notion.site/Projekt-H3-IoT-og-Serverside-med-Christoffer-og-Kasper-e8980638a8584a72b0c7d718252dbba4?pvs=4)

- board/
- docs/
	- ER Diagrammer/
	- Flowcharts/
	- Kravsspecificationer.md
	- Manual.md
- game/
- server/
- .gitignore
- README.md

### Projekt pitch:

Vi vil lave et 1-mod-1 fighting-spil lignende spillet [Brawlhalla](https://www.brawlhalla.com/) eller [Smashbros](https://www.nintendo.dk/nintendo-switch-familien/spil/super-smash-bros-ultimate) der gennem 2 NFC-skannere kan bruges til at vise 'helte' i spillet, man så derefter kan bruge til at spille mod hinanden. Spillet gør brug af disse hardwarekomponenter: NFC-skanner, LCD-display og knapper. Vores mål er at udvikle en løsning, hvor man bruger en fysisk figur til at spille spillet i stedet for, det kun foregår på skærmen. Dette løser vi ved at integrere et interaktivt system, der kan indsamle og reagere på data. For at opnå dette, har vi tænkt os at anvende programmeringssproget Rust til selve spillet, Rust eller Typescript til en server med en HTTP-API og en SQL-database.

Vores system vil kunne interagere med brugerne gennem et dashboard (selve spilklienten), der viser spillet, dvs. de relevante data til spillet, hvilket giver brugerne mulighed for at spille spillet, som så vil foretage ændringer i dataen. Dette projekt vil ikke kun give os praktisk erfaring med Rust, Spilarkitektur, -design, -fysik, -logik, -mekanik, Arduino, embedded C++, kommunikation over HTTP, interaktion med Database, men også muligheden for at udforske, hvordan teknologi kan anvendes til at løse reelle problemer eller forbedre dagligdagen.

