# Парсер математических выражений

## Статусы парсера
- Started
- ProcessingInteger
- FinishedInteger
- ProcessingFloat
- FinishedFloat
- WaitingAfterNegation
- WaitingAfterFloatPoint
- WaitingAfterOpenBrace
- WaitingAfterCloseBrace
- WaitingAfterOperator

## Ожидания

После того, как парсер начал работу он ожидает только:
- "-" Negation
- "0-9" Digit
- "(" OpenBrace

После "Negation" ожидаем только:
- "0-9" Digit
- "(" OpenBrace

После "Digit" ожидаем только:
- "0-9" Digit
- "." FloatPoint
- " " Space
- "(" OpenBrace
- "+-*/^" Operator

После "FloatPoint" ожидаем только:
- "0-9" Digit

"Space" это просто признак того, что закончилось число

После "OpenBrace" ожидаем только:
- "-" Negation
- "0-9" Digit
- "(" OpenBrace

После "Operator" ожидаем только:
- " " Space
- "0-9" Digit
- "(" OpenBrace
- "-" Negation

## Приоритеты
1. ()
2. ^
3. */
4. +-
