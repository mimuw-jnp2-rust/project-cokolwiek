
### set up pod stt
shit to text

#### modele
jeśli chcemy na razie używać jedynie engelsk modelu to droga prosta:
```sh
cd en-model
./setup.sh
```
to zajmie chwilkę poniewaz to sążny model jest

dla polskiego analogicznie w `pl-model`, ale to na rzie wgl unimplemented więc no

#### lib-stt
ważniejsze jest natomiast zainstalowanie samej biblioteki
```sh
cd lib-stt
./setup.sh
```

**to są rzeczy jednorazowe**

#### kompilacja
po otwarciu naszrgo katalogu trzeba powiedziec rustowi gdzie ma to stt
```
. setup_paths.sh
```
nie można po prostu odpalić skryptu, trzeba go tak skropkować

To trzeba robić raz na jedną sesję terminala ponieważ to ustawia zmienne środowiskowe.

bez tego cargo run ani cargo build nie zadzialaja

## uruchamianie

1. `sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev` (na linuxie)

2. `rustup update`

3. `cargo run`

---

## todo ważne dla io

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
