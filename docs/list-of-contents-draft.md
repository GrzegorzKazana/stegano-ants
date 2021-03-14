-   # Wstęp

    -   cel pracy
    -   zakres pracy
    -   opis struktury pracy

-   # 1 Wprowadzenie do dziedziny

    -   ## 1.1 Steganografia

        -   definicja
        -   porównanie z kryptografią
        -   rys historyczny

    -   ## 1.2 Systemy wieloagentowe

        -   definicja
        -   problemy w których wykorzystuje się SW
        -   przykłady systemów wieloagentowych

    -   ## 1.3 Optymalizacja mrowiskowa
        -   geneza
        -   zasada działania
        -   systemy mrówkowe/mrowiskowe jako systemy wieloagentowe
        -   zastosowania

-   # 2 Przegląd literatury

    -   ## 2.1 Steganografia

        -   prace będące _pionierami_ w dziedzinie
        -   dane w jakiej postaci i jakie dane maskujące stosowano
        -   prace w których danymi maskującymi są pliki graficzne
        -   lista metod ukrywania danych w obrazach
        -   wskazówki odnośnie doboru miejsc o większej pojemności i mniejszej degradacji obrazu

    -   ## 2.2 Optymalizacja mrowiskowa

        -   prace będące _pionierami_ w dziedzinie
        -   zastosowanie systemów mrowiskowych do problemu komiwojażera
        -   wyszczególnienie rodzajów systemów i metod wyboru krawędzi i aktualizacji feromonu

    -   ## 2.3 Zastosowanie uczenia maszynowego i systemów wieloagentowych w steganografii

        -   prace łączące steganografię i sys. mrówkowe
        -   prace łączące steganografię i metody uczenia maszynowego

    -   ## 2.4 Odniesienie do istniejących prac
        -   jakie aspekty były pomijanie w przytoczonych pracach
        -   zastosowań jakich metod nie znaleziono w literaturze
        -   co nowego wnosi poniższa praca

-   # 3 Metody ukrywania informacji w plikach graficznych

    -   ## 3.1 Metoda przestrzenne

        -   ### 3.1.1 LSB - najmniej znaczących bitów

            -   na czym polega
            -   wady/zalety

        -   ### 3.1.2 ...
            -   na czym polega
            -   wady/zalety

    -   ## 3.2 Metody domenowe

        -   ### 3.2.1 Wavelet transform - kompresja falkowa

            -   na czym polega
            -   wady/zalety

        -   ### 3.2.2 DCT - Dyskretna transformacja kosinusowa
            -   na czym polega
            -   wady/zalety

    -   ## 3.3 Podsumowanie
        -   jakie metody zdecydowano się zaimplementować
        -   uzasadnienie powyższego wyboru (i jego potencjalny wpływ na wyniki)

-   # 4 Sposób grafowej reprezentacji obrazów

    -   ## 4.1 Metoda sąsiedztwa pikseli

        -   na czym polega
        -   wady/zalety

    -   ## 4.2 Segmentacja obrazu

        -   na przykład podział na superpiksele
        -   na czym polega
        -   wady/zalety

    -   ## 4.3 Metoda zmiennej rozdzielczości (???)

        -   na przykład foveal sampling
        -   na czym polega
        -   wady/zalety

    -   ## 4.4 Podsumowanie

        -   jakie metody zdecydowano się zaimplementować
        -   uzasadnienie powyższego wyboru (i jego potencjalny wpływ na wyniki)

-   # 5 Systemy mrówkowe

    -   ## 5.1 Problem wyboru krawędzi grafu

        -   opisy metod, równania

    -   ## 5.2 Problem aktualizacji śladu feromonowego

        -   opisy metod, równania

-   # 6 Wykorzystanie śladu feromonowego do ukrywania informacji

    -   ## 6.1 Graficzna reprezentacja śladu feromonowego

        -   w jaki sposób możemy odnieść uzyskany ślad dla grafu reprezentującego obraz

    -   ## 6.2 Zastosowanie feromonu do określania ilości ukrytych danych

        -   posiadając graficzną reprezentację śladu feromonowego, rozważamy i określamy metodę jego wykorzystania w celu ukrywania danych

-   # 7 Implementacja

    -   ## 7.1 Wykorzystane narzędzia
    -   ## 7.2 Specyfikacja zewnętrzna
    -   ## 7.3 Specyfikacja wewnętrzna
        -   ### 7.3.1 Moduł A
        -   ### 7.3.2 Moduł B
        -   ### 7.3.3 Moduł ...
    -   ## 7.4 Walidacja i testowanie
    -   ## 7.5 Uruchamianie

-   # 8 Eksperymenty

    -   ## 8.1 Miary jakości i ich definicje

        -   błąd średnio-kwadratowy
        -   szczytowy stosunek sygnału do szumu
        -   podobieństwo strukturalne (Structural Similarity Index Measure) (???)

    -   ## 8.2 Wyniki eksperymentów

        -   wyniki powyższych miar oraz ilość 'pomieszczonych' danych w zależności od parametrów kolonii oraz innych parametrów (sposób reprezentacji obrazu/sposób wizualizacji feromonu)

    -   ## 8.3 Ocena subiektywna

        -   czy ukrycie danych powoduje subiektywny spadek jakości (tak - od jakiej ilości danych jest on widoczny)
        -   czy różnice w przeprowadzonych eksperymentem są widoczne gołym okiem
        -   czy wrażenia subiektywne pokrywają się z wartościami miar

-   # 9 Wnioski

    -   czy zaproponowana metoda osiągnęła założenia pracy
    -   jakie były trudności
    -   w którą stronę należałoby rozszerzyć badania
