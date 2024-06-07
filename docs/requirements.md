# Kravspecification

## Intro
Vi vil lave et 1 mod 1 fighting spil lignende Brawlhalla eller Smashbros der gennem 2 nfc skannere kan bruges til at vise 'helte' i spillet, man så derefter kan bruge til at spille i mod hinanden.  Spillet gør brug af disse hardwarekomponenter: NFC-skanner, display og knapper. Vores mål er at udvikle en løsning, hvor man bruger en fysisk figur til at spille spillet i stedet for det kun foregår på skærmen. Det løser vi ved at integrere et interaktivt system, der kan indsamle og reagere på data. For at opnå dette, tænker vi at anvende Rust til selve spillet, sql til databasen, og til api bliver der brugt Rust eller C#.

## Use cases
Jeg vil gerne have et spil. 

Jeg vil gerne have en spillefigur, jeg kan holde ved fysisk.
Jeg vil gerne kunne få min spillefigur til at deltage i spillet med en fysisk handling af en art. 
Jeg vil gerne kunne se, hvor godt jeg klarer det i spillet. 
Jeg vil gerne kunne spille mod min kammerat. 
Jeg vil gerne vide, hvor mange spil jeg har vundet og hvor mange spil jeg har tabt. 
Jeg vil gerne kunne level-up min spiller, så den bliver bedre til næste spil. 
Jeg vil gerne have et fysisk bræt, hvorpå jeg kan sætte spillere, som er uafhængig af skærmen, hvor jeg ser selve spillet. 
På brættet vil jeg gerne kunne se statistik over spillet og min spiller. 
Jeg vil gerne have en måde at skifte mellem statistikken på spillet og min spiller på brættet. 
Jeg vil gerne have det sjovt, når jeg spiller det, hvis jeg vinder, men kun hvis jeg vinder. 
Jeg vil gerne kunne lukke computeren ned, starte den op igen, starte programmet igen og få computeren til at huske mine spillers statistik og hvor mange kampe jeg har vundet og tabt.

## Constraints

Projektet skal indeholde brug af database, API, arduino, mindst 1 aktuator, mindst 2 sensorer og dashboard. API'en skal kommunikere mellem arduinoen, dashboardet og databasen.
Vi har 8 uger til projektet

## Acceptance test

### Assigner skillpoints til helt
    • Opfør opsætningen, hvortil den vil vise hovedmenuen.
    • Naviger i hovedmenuen til skillpoints-skærmen, hvortil den vil efterspørge en helt.
    • Sæt en helt på pladen, herefter vil helten vises på skærmen.
    • Assigner de tilgængelige skillpoints til forskellige af heltens skills.
    • Vælg at gemme valgene.
    • Tag spilleren af pladen.
    • Luk og genstart opsætningen.
    • Naviger til skillpoints-skærmen.
    • Sæt spilleren på pladen.
    • Tjek at skillpoints'ne er assigneret som tidligere dikteret.
### Start spil
    • Opfør opsætningen, hvortil den vil vise hovedmenuen.	
    • Naviger i hovedmenuen til spil-skærmen, hvortil den vil efterspørge to helte.
    • Sæt de to helte på pladen, herefter vil de to helt vises på skærmen.
    • Naviger til "start spil", hvorefter spillet starter.
    • På arduinoens display vises heltenes navne, hvor meget skade heltene har taget og deres liv.
    • De forskellige informationer kan navigeres på arduinoens knapper.
### Ektra
    • Lyssensor der ændre baggrundsfarve i spillet
