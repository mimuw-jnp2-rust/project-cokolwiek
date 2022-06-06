
### changes
zrobione stt, a poza tym bumped up egui o jedną wersję w górę, co
początkowo meeega zepsuło, ale dzięki temu super kręcące się kółeczko
jest (!)

##### na ile działa to stt?
_comme ci, comme ça_

### set up pod stt aka tutorial tzw
_shit to text_

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

### kompylacja

1. `sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev` (na linuxie)

#### kompylacja à la polaco

```
cargo run -- pl-model
```
po uprzednim ściągnięciu `pl-model` (ogólnie pierwszy z `argv` podany to `model-dir`,
default to jest `en-model`, można sobie przeróżne instalować ze [strony z modelami](https://coqui.ai/models))


### dependencies
also according to them:
>On Linux you need to first run:
>`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev`

i chyba `libgtk-3-dev` dla rfd?

jeśli jakieś problemy pojawią się z `alsa` to maybe install `alsa-lib`
czy cos? poszukamy najwyzej

aha i jesli cos nie dziala to consider deleting Cargo.lock, ale to powinno juz dzialac?
mi czasem rust-analyzer tyllko cos buguje, ale malo

## todo ważne dla io

- [ ] niech pyta o saving jedynie when edited, a nie zawsze, pytal o to pan g ostatnio :/
- ^ **ważne** actually, trochę nie wiem jak to zrobić, trzymać backup code między update;ami
  na wzór tego jak go trzymam podczas odbioru komunkatów od stt? compare with saved file?
  jakoś gdzieś jest miejsce koło `TextEdit` gdzie to widać? aucune idee
- [ ] saving podczas recordingu nie powinien być UB, tylko niedozwolony/coś sensible
- ^ duża szansa, że ktoś zechce to sprawdzic
- [ ] pewnie wpisaliśmy coś jeszcze do docsa z tymi planami?
- [ ] trzeba przetrenoać mówienie do komputerka 
- [ ] ktoś inny błagam niech to zreviewuje
- [X] quiting jak [tutaj](https://github.com/emilk/egui/blob/master/examples/confirm_exit/src/main.rs) bo system sie trikuje
- [X] dodać do structu aplikacji jakiś path currently edytowanego pliku
- [X] przy exicie powinien pytać o zapisanie
- [X] metoda dla structu na quit ktora jest callowana na ctrl-q i przycisk
- [X] ^ ona pyta o save/choose file i to zalatwaimy z `rfd`
- [X] ^ to mozna zalatwic `rfd::MessageDialog` typu `YesNo`, jakies "rly quit??"
- [X] przycisk open
- [X] przycisk save
- [ ] chyba tyle

### todo ważne, ale dla mądrych ludzi a nie dla io (więc np na przeklęte jnp)
- [ ] **wszystkie todo z kodu**
- [ ] lepiej handlować errory np `matches!(coś, Err(_))` (którę sam
      zasugerowałem,..) to syf i lepiej robić `coś.is_err()/is_ok()`. 
- [ ] also nwm czy nie mam przesadnie nasrane expectami, może lepiej
      robić unwrapy/znaki zapytania?
- [ ] clean code
- [ ] wywalić shit komentarze i printy, a dodać good komentarze
- [ ] _aesthetics_
  - [ ] kolorowe przyciski bardziej? są na to metody w egui chyba, np record green/red??
  - [ ] itp?
  - [ ] równe justified te przyciski, to już widzę że jest zrobione dla wiekszosci
  - [ ] pisać gdzieś currently edited file jeśli znamy nazwę, powinno to być łatwe
- [ ] _(opcjonalnie)_ użyć parsera opartego na _functional parsing_ i _parsing combinators_ 
      (fajne koncepty serio) jak ten
      [tutaj](https://github.com/hgm-king/prose/blob/master/src/parser.rs)
      (biblioteka [nom](https://docs.rs/nom/latest/nom/)) budującego
      takie [samo AST (drzewko markdownowe)](https://github.com/hgm-king/prose/blob/master/src/lib.rs)
      jak tam i przerobić parser do naszego formatu (easy chyba, malo zmian) oraz przerobić 
      później ten `viewer` by z `Vec<Markdown>` z drugiego linku tworzył `egui::RichText`
      
      myślę, że to nie jest jakieś kluczowe, ale można rozważyć by na
      jnp2 mieć mniej pozrzynany porjekt + te parsing combinators to
      fajny paradygmat?
- [X] dać własne nazwy structom miast tych skradzionych
- [ ] dać własne nazwy plikom...
- [X] wywalić te jakieś pozostałości z template'u, które polegały na tych webowych 
sprawach
- [X] pliki niektore wgl nieuzywane sa chyba serio 
- [ ] czemu to sie tak dlugo kompiluje xd
- [ ] clean code 
- [X] zrozumieć co co robi
- [ ] [log do pisania zamiast eprintln?](https://docs.rs/log/0.4.17/log/index.html)
jakoś ze zmienną env `RUST_LOG=debug`??
