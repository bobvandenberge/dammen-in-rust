# Dammen

Dit is een rust implementatie van het spel dammen. De spelregels worden gevolgd zoals beschreven op: https://nl.wikibooks.org/wiki/Dammen/Spelregels.

# Start van een spel

Voer het volgende commando in de root directory uit om het spel te starten:
``cargo run``.

Het bord wordt uitgetekend in je console. [   ] is een vlak. In een vlak kan een twee letterig code staan. De eerste letter van de code, Z of W, geeft de kleur aan; Zwart of Wit. De tweede letter, E of D, geeft aan of de schijf een Enkel of Dam is.

# Functionaliteiten
* Spel starten
* Een zet uitvoeren
* Regel: je mag alleen maar zetten op vrije vlakken
* Regel: je mag alleen je eigen stenen verplaatsen (oftewel alleen tijdens je eigen beurt)
* Regel: alleen zetten naar donkeren vlakken
* Regel: je mag maar 1 vlak per keer verplaatsen
* Regel: je kan slaan
* Regel: je moet slaan
* Regel: Spel is afgelopen als alle stenen van 1 partij weg zijn (winst/verlies)
* Regel: slaan verwijderd schijf

## Todo
* Regel: een Enkele schijf mag alleen vooruit zetten
* Regel: je beurt eindigd niet als je nog een keer kan/moet slaan (a.k.a meerslag)
* Regel: Als je op de andere kant van het bord komt, veranderd een Enkele steen in een Dam
* Regel: Dams mogen achteruit zetten
* Regel: Dams mogen meer dan 1 veld per keer verplaatsen
* Regel: Een dam mag een losse schijf of dam slaan die verderop de zelfde diagonaal staan
* Regel: Spel is afgelopen als een speler niet meer kan zetten (remise)
* Regel: Spel is afgelopen als bij spelers 1 Dam hebben (remise)

## Mogelijke verbeteringen
* String errors vervangen door errorcodes zodat consumer beter kan bepalen wat er moet gebeuren
* Support toevoegen voor grotere borden. Nu is alles 10x10 gesupport.
* Regel onderdeel van de engine dynamischer opzetten zodat regels makkelijk toegevoegd/verwijderd of gedeeld kunnen worden tussen verschillende engines.
* Verplaats COLUMN_BREEDTE const naar Spel
* In testen algemeen: beter controleren of error melding ook de verwachte is.
* VeldKleur onderdeel maken van Veld

# For developers
Het project heeft een aantal unit testen die de core functionaliteit testen. Deze testen zijn te runnen met het commando: `cargo test`.
