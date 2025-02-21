# opg3rust lavet af Emil, Mikkel, Magnus, Peter Og Mads
 Opgaveformål:

Formålet med denne opgave er at forstå datastrukturer i Rust ved at implementere en stack fra bunden. Stacken skal følge princippet "Last In, First Out" (LIFO), og I skal håndtere stackens vækst og hukommelsesallokering.

Opgavebeskrivelse:

I skal implementere en stack i Rust, som kan gemme generiske værdier (dvs. en stack, der kan arbejde med flere forskellige typer af data). Stacken skal kunne udføre de klassiske operationer: tilføje (push), fjerne (pop), og kigge på det øverste element (peek), samt tjekke om den er tom.

Trin-for-trin opgavevejledning:

1. Definér stack-strukturen Opret en ny struct kaldet Stack<T>, som kan gemme elementer af typen T. Stacken skal kunne vokse dynamisk, så brug en Vec<T> til at opbevare elementerne.

2. Implementér funktioner til stacken: push(&mut self, element: T)

Tilføjer et element til toppen af stacken. Fordi vi bruger en Vec<T> til at gemme vores elementer, er denne operation ganske ligetil. Vec har allerede en indbygget metode push, som vi kan benytte.

Vejledning:

· Vi har brug for en mutable reference til self, fordi vi vil ændre indholdet af stacken ved at tilføje et element.

· Vi kalder simpelthen self.elements.push(element) for at tilføje det nye element til slutningen af vektoren, som i en stack repræsenterer toppen.

pop(&mut self) -> Option<T>

Fjerner og returnerer det øverste element fra stacken. Hvis stacken er tom, skal vi returnere None. Hvis der er elementer i stacken, skal vi returnere det sidste element (øverst i stacken).

Vejledning:

· Vi har brug for en mutable reference til self, fordi vi vil ændre indholdet af stacken ved at fjerne et element.

· Vec har en metode pop(), som enten returnerer det sidste element eller None, hvis vektoren er tom. Vi kan direkte bruge denne metode til at implementere stackens pop().

peek(&self) -> Option<&T>

Returnerer en reference til det øverste element i stacken, men uden at fjerne det. Hvis stacken er tom, skal den returnere None.


Vejledning:

· Her behøver vi kun en immutable reference til self, fordi vi ikke ændrer indholdet af stacken.

· Vec har en metode last(), som returnerer en reference til det sidste element, hvis vektoren ikke er tom, ellers returneres None. Vi bruger denne metode til at implementere peek.

is_empty(&self) -> bool

Returnerer true, hvis stacken er tom, og false, hvis den indeholder elementer.

Vejledning:

· Vi behøver kun en immutable reference til self, fordi vi kun skal læse fra stacken.

· Vec har en indbygget metode is_empty(), som returnerer true, hvis vektoren er tom. Vi bruger denne direkte til at implementere stackens is_empty.


3. Test stacken

· Opret en main()-funktion, hvor du opretter en instans af din Stack og tester alle metoderne.

· Tilføj nogle elementer til stacken (f.eks. i32-værdier eller strenge).

· Brug pop(), peek(), og is_empty() for at sikre, at alle metoder fungerer som forventet.

4. Reflekter over hukommelsen

Tænk over, hvordan hukommelsen allokeres i din stack. Hvorfor bruger vi Vec<T> til dynamisk allokering? Hvordan håndterer Rust hukommelsen for denne struktur sammenlignet med en heap-allokeret datastruktur? Overvej, hvad der sker med elementerne, når stacken popper elementer af.

Ekstra opgaver:

· Udvid stacken, så den kan have en maksimal størrelse, og sørg for, at den kaster en fejl, hvis man forsøger at tilføje flere elementer end tilladt.

· Lav en funktion til at printe hele indholdet af stacken fra top til bund.

· Reflektér over, hvordan ydeevnen af jeres stack ville være i forhold til en heap-allokeret datastruktur.

Opsummering

· push tilføjer et element til slutningen af Vec (toppen af stacken).

· pop fjerner og returnerer det sidste element fra Vec (toppen af stacken).

· peek returnerer en reference til det sidste element uden at fjerne det.

· is_empty returnerer true, hvis stacken er tom.

· Disse metoder tilsammen implementerer en fuldt fungerende stack!

Søge hjælp fra dokumentationen og ChatGPT

Når I arbejder på opgaven og støder på udfordringer, er det vigtigt at vide, hvor I kan søge hjælp. Her er et par gode måder at få hjælp på:

Brug af Rust's officielle dokumentation:

Rust har en omfattende og veldokumenteret guide og referencemateriale, som kan hjælpe jer med at forstå sprogkonstruktioner, biblioteker og funktioner i Rust. Her er nogle vigtige dele af dokumentationen, I kan benytte:

· Rust Book (https://doc.rust-lang.org/book/): Dette er den officielle vejledning til Rust og dækker alt fra begynderniveau til mere avancerede koncepter. Bogen indeholder mange eksempler og forklaringer.

· Rust Reference (https://doc.rust-lang.org/reference/): Dette er en mere teknisk reference, der dykker ned i Rust's syntaks og regler.

· API Dokumentation (https://doc.rust-lang.org/std/): Her kan I finde detaljer om standardbibliotekerne i Rust, såsom Vec, som I bruger i denne opgave.

Tips til at bruge dokumentationen:

· Brug søgefunktionen (Ctrl + F eller browserens søgefelt) for hurtigt at finde relevante sektioner.

· Tjek kodeeksemplerne i dokumentationen, da de ofte viser, hvordan forskellige funktioner anvendes i praksis.

· Vær ikke bange for at læse flere gange og eksperimentere med små kodeeksempler for at forstå, hvordan tingene fungerer.

Brug af ChatGPT:

ChatGPT kan være en stor hjælp, når I har brug for hurtige svar på spørgsmål om Rust eller programmering generelt. Her er et par tips til, hvordan I kan få mest muligt ud af ChatGPT:

· Stil præcise spørgsmål: Jo mere specifik du er i din forespørgsel, jo mere nyttigt vil svaret være. Eksempelvis: "Hvordan implementerer jeg en generisk stack i Rust med Vec?" i stedet for "Hvordan laver jeg en stack?"

· Brug ChatGPT til at forklare fejlmeddelelser: Hvis I støder på en fejl i jeres kode, kan I kopiere fejlmeddelelsen og spørge ChatGPT, hvad den betyder, og hvordan I kan løse den.

· Få hjælp til dokumentationen: Hvis I ikke kan finde den rigtige dokumentation, kan I spørge ChatGPT om, hvor I skal lede, eller om en hurtig forklaring på et begreb eller en funktion.

· Kodeeksempler: Hvis I har brug for at se et konkret eksempel, kan I bede ChatGPT om at skrive et lille kodestykke, som I derefter kan tilpasse jeres egen opgave.


Eksempler på spørgsmål til ChatGPT:

· "Hvordan fungerer borrowing i Rust?"

· "Hvordan kan jeg teste, om min stack er tom i Rust?"

· "Kan du forklare, hvorfor jeg får en lifetime-fejl i min kode?"


Ved at kombinere Rust's dokumentation og ChatGPT kan I hurtigt finde løsninger på de fleste problemer og få dybere forståelse for de begreber, I arbejder med.
