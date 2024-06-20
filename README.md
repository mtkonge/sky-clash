# SkyClash

<!-- TOC start (generated with https://github.com/derlin/bitdowntoc) -->

- [Opgave](#opgave)
   * [H3-Smart-Kommunikationsplatform - SkyClash](#h3-smart-kommunikationsplatform-skyclash)
   * [Projekt pitch:](#projekt-pitch)
- [Opsætning](#opsætning)
- [Brugerguide](#brugerguide)
- [Encyclopedia](#encyclopedia)
   * [Heltespiller](#heltespiller)
   * [Heltespillerfigurer](#heltespillerfigurer)
   * [Sky Board](#sky-board)
   * [Programmet](#programmet)
      - [Startmenuen](#startmenuen)
      - [Hero Creator](#hero-creator)
      - [Start Game](#start-game)

<!-- TOC end -->


<!-- TOC --><a name="opgave"></a>
## Opgave

<!-- TOC --><a name="h3-smart-kommunikationsplatform-skyclash"></a>
### H3-Smart-Kommunikationsplatform - SkyClash

Projekt kan findes her: ["Notion link"](https://mercantec.notion.site/Projekt-H3-IoT-og-Serverside-med-Christoffer-og-Kasper-e8980638a8584a72b0c7d718252dbba4?pvs=4)

Filstrukturen ser sådan ud:

- board/
- docs/
	- Brugerguide/ 	
	- ER Diagrammer/
	- Flowcharts/
  	- images/
	- Kravsspecificationer.md
	- Manual.md
- game/
- server/
- .gitignore
- README.md

<!-- TOC --><a name="projekt-pitch"></a>
### Projekt pitch:

Vi vil lave et 1-mod-1 fighting-spil lignende spillet [Brawlhalla](https://www.brawlhalla.com/) eller [Smashbros](https://www.nintendo.dk/nintendo-switch-familien/spil/super-smash-bros-ultimate) der gennem 2 NFC-skannere kan bruges til at vise 'helte' i spillet, man så derefter kan bruge til at spille mod hinanden. Spillet gør brug af disse hardwarekomponenter: NFC-skanner, LCD-display og knapper. Vores mål er at udvikle en løsning, hvor man bruger en fysisk figur til at spille spillet i stedet for, det kun foregår på skærmen. Dette løser vi ved at integrere et interaktivt system, der kan indsamle og reagere på data. For at opnå dette, har vi tænkt os at anvende programmeringssproget Rust til selve spillet, Rust eller Typescript til en server med en HTTP-API og en SQL-database.

Vores system vil kunne interagere med brugerne gennem et dashboard (selve spilklienten), der viser spillet, dvs. de relevante data til spillet, hvilket giver brugerne mulighed for at spille spillet, som så vil foretage ændringer i dataen. Dette projekt vil ikke kun give os praktisk erfaring med Rust, Spilarkitektur, -design, -fysik, -logik, -mekanik, Arduino, embedded C++, kommunikation over HTTP, interaktion med Database, men også muligheden for at udforske, hvordan teknologi kan anvendes til at løse reelle problemer eller forbedre dagligdagen.

<!-- TOC --><a name="opsætning"></a>
## Opsætning

SkyClash er et spil-system for 2 spillere. Systemet består af selve spillet og *SkyBoard* til ens *SkyHero*-figurer.

Du skal bruge:
- En computer
- *SkyBoard*
- Mindst 2 *SkyHero*-figurer


Sæt *SkyBoard* til strøm. Dette gøres med USB-ledningen, som følger med. *SkyBoard* vil derefter oprette forbindelse til *SkyServer* via. Wifi\*. LED-indikatorne vil herefter lyse.

Hent spillet ned på din computer. Spillet hentes fra [udgivelsessiden](https://github.com/Mercantec-GHC/h3-projektv2-24q2h3-skyclash/releases/latest). Hent pakken til dit operativsystem. 
Hvis du bruger Windows, hent `skyclash-windows.zip`. Udpak Zip-filen i en seperat mappe. Åben spillet ved at køre filen `sky-clash.exe`. Hvis du bruger Linux, hent `skyclash-linux.tar.gz`. Udpak filer (`tar xvf <file>`) og åben spillet, ved at køre den eksekverbare fil `sky-clash`\*\*.

Initialiser dine *SkyHero*-figurer\*\*\*. Start spillet med én enkelt *SkyHero*-figur på *SkyBoard*. Åben spillet. Naviger til *Hero Creator*-menuen. Her vil der blive vist en Pop-up-menu, med hver slags *SkyHero*. På billedet nedenunder ses alle figurer i rækkefølgen: `Tankie`, `Strong`, `Speed` og `Centrist`, Vælg den rigtige helt.

<img src="/docs/images/heroes.jpg" height="300"/>

\* Nuværende er Wifi opsat til at bruge netværket `TP-Link_1912`.

\*\* På Linux kræver det, at man henter de nødvendige SDL2-pakker. Disse hedder eksempelvis `sdl2`, `sdl2_image` og `sdl2_ttf` på Archlinux, `libsdl2-2.0-0`, `libsdl2-image-2.0-0` og `libsdl2-ttf-2.0-0` på Debian og Ubuntu.

\*\*\* Dette er kun relevant, hvis figurene ikke allerede er oprettet i serveren.

<!-- TOC --><a name="brugerguide"></a>
## Brugerguide

Sæt 2 *SkyHero*-figurer på *SkyBoard*. Åben spillet og naviger til *Start Game*-menuen. Her ses et overblik over de valgte *SkyHero* og deres evner. Spillet går i gang, når der trykkes på *Start Game*. Knapperne til at styre hver figur, ses på tabellen nedenfor.

Handling | Venstre spiller | Højre spiller
---|---|---
Hop | `W` | `↑`
Gå til venstre | `A` | `←`
Gå ned | `S` | `↓`
Gå til højre | `D` | `→`
Angrib | `J` | `Numpad Enter`
Undvig | `K` | `Numpad ,`

Når en spiller angribes, tager den skade. Jo mere skade, jø større effekt har følgende angreb mod spilleren. Spillerens skade kan ses på farven boksene i hjørnerne af skærmen. Farven går hvid til grøn, rød og til sort.

Når en spiller falder ud af det synlige areal, mister de et liv. Alle spillere starter med 3 liv. Mængden af liv tilbage kan ses på tallet i boksene i hjørnerne. Når en spiller har mistet alle 3 liv, vinder den anden spiller.

Som man vinder kampe, optjener man *Skill Points*. Disse kan benyttes i *Hero Creator*-menuen, til at forbedre helten. Dette gøres ved, man sætter én enkelt *SkyHero* på *SkyBoard* og navigere til *Hero Creator*-menuen. I menuen kan man se mængden af *Skill points* til rådighed. Kan man allokere sine point med `-`- og `+`-knapperne for hver evne: *Strength*, *Agility* og *Defence*. Når man er tilfreds, kan man trykke på *Save*-knappen for at gemme og *Back*-knappen for at gå tilbage til startmenuen.

<!-- TOC --><a name="encyclopedia"></a>
## Encyclopedia

<!-- TOC --><a name="heltespiller"></a>
### Heltespiller

En heltespiller er en figur som du kontrollerer når du spiller Sky Clash.

De kan have op til 24 point i 3 kategorier, styrke, beskyttelse og hastighed. De point er uddelt gennem Sky Clash' Hero Creator, forklaret senere.

Der findes 4 typer af heltespillere, som hver starter med en hvis mængde point uddelt på en unik måde; man får flere point at kunne uddele ved at spille og vinde kampe.

De 4 heltespillertyper er Tankie, som har en fokus på beskyttelse, Strong, som har fokus på styrke, Speed, som har fokus på hastighed og Centrist, som ikke har nogen fokus.

En heltespiller er bundet til en fysisk heltespillerfigur.

<!-- TOC --><a name="heltespillerfigurer"></a>
### Heltespillerfigurer

Heltespillerfigurerne er figurer, der bruges sammen med dit Sky Board for at kunne spille Sky Clash. De representerer én af de 4 heltespillertyper, og bruges til at gemme information om dine kampe vundet, og de valgt du har truffet i Sky Clash’ Hero Creator.

<img src="/docs/images//heroes.jpg" alt="heroes" height="300"/>

<!-- TOC --><a name="sky-board"></a>
### Sky Board

Dit Sky Board bruges sammen med heltespillerfigurer for at kunne spille Sky Clash. Man bruger det ved at placere en eller flere heltespillerfigur på en eller begge af to markerede pladser.

<img src="/docs/images//sky_board.jpg" alt="skyboard" height="300"/>


<!-- TOC --><a name="programmet"></a>
### Programmet

Det kræver mindst 2 heltespillerfigure og et Sky Board for at kunne spille.

<!-- TOC --><a name="startmenuen"></a>
#### Startmenuen

Det første du kommer til at møde når du starter Sky Clash er startmenuen.

<img src="/docs/images//main_menu.png" alt="main menu" height="300"/>

Her mødes du af 3 knapper, “start game”, “hero creator” og “exit”. Du kan bruge musen eller Tab knappen til at navigere, og venstre klik eller enter for at vælge en mulighed.

Exit lukker programmet.

Start game og Hero creator leder dig til menuen for at hhv. Starte spillet og skabe eller modificere en helt. Vi starter med at skabe en helt.

For at opnå dette, skal du med dit Sky Board have præcist en heltespillerfigur på en af de markerede pladser, og vælge muligheden "Hero Creator".

<!-- TOC --><a name="hero-creator"></a>
#### Hero Creator

Før du kan gå ind på hero creator'en skal du sikre dig at du har sat én helt på Sky Board'et

<img src="/docs/images//sky_board_with_hero.jpg" alt="skyboard with hero" height="300"/>

Hvis helten ikke er oprettet endnu, bliver du mødt af et valg af hvilken slags heltespillertype du har at gøre med.

<img src="/docs/images//select_hero.png" alt="select hero" height="300"/>

Her vælger du et af 4 muligheder der matcher til din heltespiller.

Til at starte med har du ingen ekstra point at uddele, men efter du har vundet et par kampe får du muligheden til at komme tilbage for at uddele flere point. Du uddeler eller fjerner de point ved at klikke på "+" eller "-" ved siden af den kategori du vil uddele eller fjerne point fra. Du kan ikke fjerne point igen efter du har uddelt og gemt dem, så vær sikker på, at du har taget de rigtige valg!

Når du er færdig, kan du så trykke på "Confirm" knappen for at gemme, og derefter "Back" knappen for at komme til startmenuen.

<img src="/docs/images//hero_creator.png" alt="hero creator" height="300"/>

Derefter kan du bytte din heltespillerfigur på brættet ud med en anden heltespillerfigur og gøre det samme.

Nu hvor du har 2 heltespillerfigure forberedt kan du så placere begge heltespillerfigure på hver deres plads på dit Sky Board og vælge muligheden "Start Game" for at begynde at spille.

<!-- TOC --><a name="start-game"></a>
#### Start Game

Før du kan gå ind på "Start Game" skal du sikre dig, at du har sat to helte på Sky Board'et

<img src="/docs/images//sky_board_with_heroes.jpg" alt="skyboard with heroes" height="300"/>

Det første du ser er en menu, hvor du kan se hvilke heltespillerfigure du har på brættet, og hvordan deres point er uddelt. Når du har 2 spillere på brættet, kan du trykke "Start Game" igen for at spille en kamp.

<img src="/docs/images//start_game.png" alt="start game" height="300"/>

Når du har 2 spillere på brættet, kan du trykke "Start Game" igen for at spille en kamp.

<img src="/docs/images//game.png" alt="game" height="300"/>

Spiller 1 bevæger med WASD, angreber med J og undviger med K.

Spiller 2 bevæger med piletasterne, angreber med 'Numpad Enter' og undviger med 'Numpad .'

Hvis du står stille, angreber du over digselv.

Hvis du holder din "Bevægelse Ned" knap nede mens du angreber, angreber du ved dine fødder, som er god til at få den anden spiller op i luften for at kunne lave et andet angreb efter.

Hvis du holder din "Bevægelse til Højre" eller "Bevægelse til Venstre" knap nede angreber du hhv. til højre eller venstre.

<img src="/docs/images//hero_information_left.png" alt="hero information left" height="300"/>
<img src="/docs/images//hero_information_right.png" alt="hero information right" height="300"/>

helteinformationsboks viser informationer for spillets gang og kan ses i de to øverste hjørner. helteinformationsboksen viser spillernes liv, deres skade og hvilken figur de spiller.

Hvert angreb giver skade der rammer den anden spiller skader den anden spiller. Omridset af spillerne i helteinfomrtionsboksen skifter fra hvid til gul til orange til rød og til sort. Jo tættere på rød/sort jo mere skade har spilleren taget dvs. spilleren bliver skubbet mere når spilleren bliver ramt af et angreb.  

Når spilleren falder ud af banen mister spilleren et liv og placeres tilbage på banen. 

Målet er at få den anden spiller ud af skærmen indtil han ikke har flere liv tilbage, hvor derefter er kampen vundet.
