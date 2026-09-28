# Game of Life

Implementação do Game of Life de Conway em Rust.

## As regras

A grade tem células vivas e mortas. A cada geração, todas as células são atualizadas ao mesmo tempo, olhando para os 8 vizinhos:

- Célula viva com 2 ou 3 vizinhos vivos continua viva.
- Célula viva com menos de 2 ou mais de 3 vizinhos vivos morre.
- Célula morta com exatamente 3 vizinhos vivos nasce.

A próxima geração é calculada numa grade nova, a partir da antiga. Se a grade fosse alterada durante a leitura, uma célula que acabou de mudar afetaria a contagem das vizinhas.

## Arquivos

| Arquivo | O que faz |
|---------|-----------|
| `grid.rs` | A grade e as regras. Não tem nada de gráfico. |
| `screen.rs` | Tudo que mexe no terminal: tamanho, entrar e sair do modo de jogo, desenhar a grade. |
| `main.rs` | Cria a grade e roda o loop até apertar Esc. |

## Como a grade é guardada

A grade tem `height` linhas e `width` colunas. Ela fica num `Vec<bool>` só, uma linha depois da outra (`true` = viva).

Numa grade 4×4:

```
 0  1  2  3
 4  5  6  7
 8  9 10 11
12 13 14 15
```

Conversão entre coordenada `(x, y)` e índice, com `x` = coluna e `y` = linha:

```
índice = y * width + x
x      = índice mod width
y      = índice / width
```

## Vizinhos e bordas

A grade dá a volta: quem sai pela direita volta pela esquerda, e quem sai por cima volta por baixo. Assim toda célula tem 8 vizinhos, inclusive nos cantos.

Cada vizinho é um deslocamento `dx` na coluna e `dy` na linha, cada um valendo -1, 0 ou +1:

|          | dx = -1       | dx = 0   | dx = +1        |
|----------|---------------|----------|----------------|
| dy = -1  | `top_left`    | `top`    | `top_right`    |
| dy = 0   | `left`        | célula   | `right`        |
| dy = +1  | `bottom_left` | `bottom` | `bottom_right` |

A fórmula separa o índice em duas partes, faz cada uma dar a volta sozinha e soma de novo. `W` é o `width` e `H` é o `height`:

```
coluna      = índice mod W
início      = índice - coluna
nova_coluna = (coluna + dx + W) mod W
novo_início = (início + dy*W + W*H) mod (W*H)
vizinho     = novo_início + nova_coluna
```

- A coluna dá a volta dentro da linha (`mod W`).
- O começo da linha dá a volta na grade inteira (`mod W*H`, o total de células).
- O `+ W` e o `+ W*H` evitam número negativo. Somar uma volta inteira não muda o resultado do `mod`, e `usize` não aceita valor negativo.

Exemplo: os vizinhos da célula 0 numa grade 4×4 são 15, 12, 13, 3, 1, 7, 4 e 5.
