<!-- komentarze:
- w szczególe opisywać tylko wykorzystane metody -->

-   # Wstęp

    -   cel pracy
    -   zakres pracy
    -   opis struktury pracy

-   # 1 Steganografia

    -   ## 1.1 Wprowadzenie

        -   definicja
        -   porównanie z kryptografią
        -   rys historyczny

    -   ## 1.2 Literatura

        -   prace będące _pionierami_ w dziedzinie
        -   dane w jakiej postaci i jakie dane maskujące stosowano
        -   prace w których danymi maskującymi są pliki graficzne
        -   lista metod ukrywania danych w obrazach
        -   wskazówki odnośnie doboru miejsc o większej pojemności i mniejszej degradacji obrazu

-   # 2 Systemy wieloagentowe/mrówkowe

    -   ## 2.1 Wprowadzenie

        -   geneza
        -   zasada działania
        -   systemy mrówkowe/mrowiskowe jako systemy wieloagentowe
        -   zastosowania

    -   ## 2.2 Literatura

        -   prace będące _pionierami_ w dziedzinie
        -   zastosowanie systemów mrowiskowych do problemu komiwojażera
        -   wyszczególnienie rodzajów systemów i metod wyboru krawędzi i aktualizacji feromonu
        -   prace łączące steganografię i sys. mrówkowe
        -   prace łączące steganografię i metody uczenia maszynowego

    -   ## 2.3 Ukrywanie informacji w plikach graficznych

<!-- jeśli będzie tyle treści na osobny rozdział 3 -->

-   # 3 Grafowa reprezentacja obrazów

    -   ## 3.1 Metoda sąsiedztwa pikseli

        -   na czym polega
        -   wady/zalety

    -   ## 3.2 Segmentacja obrazu

        -   na przykład podział na superpiksele
        -   na czym polega
        -   wady/zalety

    -   ## 3.3 Metoda zmiennej rozdzielczości (???)

        -   na przykład foveal sampling
        -   na czym polega
        -   wady/zalety

    -   ## 3.4 Podsumowanie

        -   jakie metody zdecydowano się zaimplementować
        -   uzasadnienie powyższego wyboru (i jego potencjalny wpływ na wyniki)

-   # 4 Zastosowanie systemów mrówkowych w steganografii

    -   ## 4.1 Istniejąca literatura
    -   ## 4.2 Opis `mojej` metody i poczynionych założeń
    -   ## 4.3 Graficzna reprezentacja śladu feromonowego
    -   ## 4.4 Zastosowanie feromonu do określania ilości ukrytych danych

-   # 5 Implementacja

    -   ## 5.1 Wykorzystane narzędzia
    -   ## 5.2 Specyfikacja zewnętrzna
    -   ## 5.3 Specyfikacja wewnętrzna
        -   ### 5.3.1 Moduł A
        -   ### 5.3.2 Moduł B
        -   ### 5.3.3 Moduł ...
    -   ## 5.4 Walidacja i testowanie
    -   ## 5.5 Uruchamianie

-   # 6 Eksperymenty

    -   ## 6.1 Miary jakości i ich definicje

        -   błąd średnio-kwadratowy
        -   szczytowy stosunek sygnału do szumu
        -   podobieństwo strukturalne (Structural Similarity Index Measure) (???)

    -   ## 6.2 Wyniki eksperymentów

        -   wyniki powyższych miar oraz ilość 'pomieszczonych' danych w zależności od parametrów kolonii oraz innych parametrów (sposób reprezentacji obrazu/sposób wizualizacji feromonu)

    -   ## 6.3 Ocena subiektywna

        -   czy ukrycie danych powoduje subiektywny spadek jakości (tak - od jakiej ilości danych jest on widoczny)
        -   czy różnice w przeprowadzonych eksperymentem są widoczne gołym okiem
        -   czy wrażenia subiektywne pokrywają się z wartościami miar

-   # 7 Wnioski

    -   czy zaproponowana metoda osiągnęła założenia pracy
    -   jakie były trudności
    -   w którą stronę należałoby rozszerzyć badania
