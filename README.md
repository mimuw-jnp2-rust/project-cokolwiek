
### todo, or rather sbd _(should've been done)_ but we suffered from _time deprivation_
list of things that could've been done and known issues
- parsing of markdown wasn't improved at all
- stt always inserts spoken text at the back instead of at where the cursor's at,
  only noticed that just a few days ago. Could be done similarily to how shortcuts
  use the cursor's position but would need moving recording habdling inside TextEdit's
  ui.
- no unit tests :/
- some functions could be refactored

# STT

#### modele
jeśli chcemy na razie używać jedynie _engelsk_ modelu to droga prosta:

```sh
cd en-model
./setup.sh
```
to zajmie chwilkę poniewaz to sążny model jest (zwłaszcza tzw scorer)

dla polskiego analogicznie w `pl-model` jeśli wola

#### libstt
ważne jest również zainstalowanie samej biblioteczki

```sh
cd libstt
./setup.sh
```

#### stt à la polaco

```
cargo run -- pl-model
```
po uprzednim ściągnięciu `pl-model` (ogólnie pierwszy z `argv` podany to `model-dir`,
default to jest `en-model`, można sobie przeróżne instalować ze [strony z modelami](https://coqui.ai/models))


### pakiety potrzebne
być może potrzeba doinstalować `libsound2-dev` jakimś `apt`em, ale to różnie bywa, 
niektórzy musieli niektórzy nie.

Generalnie linux i dźwięk to smrodliwa sprawa, ale u mnie działa, że tak powiem.

# Pierwsza iteracja

## edytor tekstowy w ruście

Pierwsza część projektu zawiera edytor tekstowy obsługujący:
- formatowanie w stylu MarkDown'a, m.in: \*pogrubienie\*, \/kursywa/\, \`kod\`, \$indeks dolny\$, \^indeks górny\^, \~przekreślenie\~, \_podkreślenie\_, \<url\>
- wyświetlanie sformatowanego tekstu na bieżąco w równoległym oknie (podgląd można wyłączyć)
- skróty klawiszowe: ctrl + B: *strong*  ctrl + D: `code`  ctrl + I: /italics/  ctrl + L: $lowered$  ctrl + R: ^raised^  ctrl + S: ~strikethrough~  ctrl + U: _underline_  ctrl + Q: quit
- obsługę plików: otwieranie pliku, zapis pliku, zapis pliku pod nową nazwą, zapis pliku przy wychodzeniu z edytora

## uruchamianie

1. `sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev` (na linuxie)

2. `rustup update`

3. `cargo run`

---

# plan projektu

## Edytor tekstu z mozliwoscia wprowadzania glosowego i wsparciem dla pracy zdalnej 🤠
that was a handful
チーム

## Autorzy
- Agnieszka Pałka (gr 4, @kaorixxx)
- Grzegrz Cichosz (gr. 9, @ggegoge)
- Maja Wiśniewska (gr 4, @miwisniewsk)

oraz io w 4 osoby to jeszcze jeden człowiek spoza grup rustowych

## Opis
Od zawsze chcieliśmy napisać projekt na io.

W tymże projekcie chodzi o to, że napiszemy w ruście edytor tekstu z mozliwoscia wprowadzania glosowego i wsparciem dla pracy zdalnej 🤠, a przynajmniej w jakiejś części. Edytor będzie służył do edycji tekstu plain, ale będzie może wyświetlał obok sformatowany à la markdown tenże tekst.

Z grubsza będziemy wzorować się na (TBD, tutaj jestesmy otwarci na sugestie jak zacząć coś takiego w ogóle mając 0 doświadczenia z gui jakimkolwiek).

## Funkcjonalność
- pisanie tekstu
- wmawianie tekstu
- widok na sformatowany markdownem tekst czy cos
- jakies basic rzeczy typu skroty klawiszowe, zapisanie pliku
- ta praca zdalna to taka jak sie uda

## Propozycja podziału na części
edytor jako cz 1 i pozniej dodatki speech to tekstowe jako part 2

ale to dość treściwie może się rozwinąć i pytanie czy ta tzw pierwsza część nam nie wypadnie wcześniej w związku ze wspomnianym już io

## Biblioteki
- egui do gui? seems legit i ma [taki cool example](https://www.egui.rs/#easymark)
- do stt [coqui-stt](https://github.com/tazz4843/coqui-stt) wydaje się dość sensowne (mowa o j. angielskim)
- jeśli jakieś są sensowne uwagi do dodania do tego, to z chęcią się dowiemy, ale jesteśmy troszeczkę dziećmi we mgle
- komunikacja jakoś do ogarnięcia
