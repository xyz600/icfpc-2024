# ICFP 言語

星間通信関数型プログラム（ICFP）は、スペースで区切られたトークンのリストで構成されます。トークンは、ASCIIコード33（'!'）からコード126（'~'）までの印刷可能なASCII文字の1つ以上で構成されます。つまり、94種類の文字があり、トークンはそれらの文字の非空のシーケンスです。

トークンの最初の文字はインジケーターと呼ばれ、トークンのタイプを決定します。トークンの残りの部分（空である可能性もある）はボディと呼ばれます。異なるトークンのタイプは次のセクションで説明されます。

# ブール値

インジケーターが T でボディが空の場合、定数の真を表し、インジケーターが F でボディが空の場合、定数の偽を表します。

# 整数

インジケーターが I で、空でないボディが必要です。

ボディは94進数として解釈されます。例えば、数字は感嘆符が0を表し、二重引用符が1を表す94の印刷可能なASCII文字です。例えば、I/6 は数値1337を表します。

# 文字列

インジケーターが S

バウンド変数のカルトは、文字をエンコードするためにASCIIに似たシステムを使用しているようですが、順序が若干異なります。具体的には、ボディからのASCIIコード33から126は、次の順序に従って変換することで人間が読みやすいテキストに変換できます：

abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~<space><newline>
ここで<space>は1つのスペース文字を、<newline>は1つの改行文字を表します。例えば、SB%,,/}Q/2,$_ は文字列 "Hello World!" を表します。

# 単項演算子

インジケーターが U で、ボディは正確に1文字の長さである必要があり、それに続いてそれを解析できるICFPが続きます。

| 文字 | 意味 | 例 |
|------|------|----|
| - | 整数の否定 | U- I$ -> -3 |
| ! | ブール値の否定 | U! T -> false |
| # | 文字列を整数として解釈する | U# S4%34 -> 15818151 |
| $ | 整数を文字列に変換する | U$ I4%34 -> test |

この表の -> 記号は「評価結果」を意味します。評価については後述します。

# 二項演算子

インジケーターが B で、ボディは正確に1文字の長さである必要があり、それに続いて2つのICFPが続きます（これらをxとyと呼びます）。

| 文字 | 意味 | 例 |
|------|------|----|
| + | 整数の加算 | B+ I# I$ -> 5 |
| - | 整数の減算 | B- I$ I# -> 1 |
| * | 整数の乗算 | B* I$ I# -> 6 |
| / | 整数の除算（ゼロに向かって切り捨て） | B/ U- I( I# -> -3 |
| % | 整数の剰余 | B% U- I( I# -> -1 |
| < | 整数の比較 | B< I$ I# -> false |
| > | 整数の比較 | B> I$ I# -> true |
| = | 等価比較、int, bool, stringに対応 | B= I$ I# -> false |
| \| | ブール値の OR | B\| T F -> true |
| & | ブール値の AND | B& T F -> false |
| . | 文字列の連結 | B. S4% S34 -> "test" |
| T | 文字列yの最初のx文字を取る | BT I$ S4%34 -> "tes" |
| D | 文字列yの最初のx文字を削除する | BD I$ S4%34 -> "t" |
| $ | 項xをyに適用する（ラムダ抽象参照） |

# If

インジケーターが ? でボディが空の場合、3つのICFPが続きます。最初のものはブール値に評価され、その値が真なら2番目のものが結果として評価され、偽なら3番目のものが評価されます。例えば：

? B> I# I$ S9%3 S./
は "no" に評価されます。

# ラムダ抽象

インジケーターが L の場合、それはラムダ抽象であり、ボディは整数と同じ方法で94進数として解釈され、変数番号を示します。インジケーターが v の場合、それは変数を示し、再びボディは94進数の変数番号です。

ラムダ抽象が二項適用演算子 $ の最初の引数として現れる場合、適用の2番目の引数がその変数に割り当てられます。例えば、ICFP

B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK
は次のようなプログラムを表します（例：Haskellスタイルで）

((\v2 -> \v3 -> v2) ("Hello" . " World!")) 42
これは文字列 "Hello World!" に評価されます。

# 評価

最も普及しているICFPメッセージングソフトウェアであるMacroware Insightは、ICFPメッセージを名前渡し戦略を用いて評価します。これは、二項適用演算子が非厳密であり、2番目の引数が束縛変数の場所に置き換えられることを意味します（キャプチャ回避の置換を使用）。ラムダ抽象のボディで引数が使用されない場合（例：上記の例でのv3）、それは決して評価されません。変数が複数回使用される場合、式は複数回評価されます。

例えば、評価は以下のステップを取ります：

```
B$ L# B$ L" B+ v" v" B* I$ I# v8
B$ L" B+ v" v" B* I$ I#
B+ B* I$ I# B* I$ I#
B+ I' B* I$ I#
B+ I' I'
I-
```

# 制限

地球との通信が複雑であるため、カルトはMacroware Insightソフトウェアにいくつかの制限を設けたようです。具体的には、メッセージ処理は10,000,000回のベータ簡約を超えると中断されます。組み込み演算子は厳密であり（もちろんB$を除く）、ベータ簡約の制限にはカウントされません。したがって、参加者のメッセージはこれらの制限内に収まる必要があります。

例えば、評価に109回のベータ簡約を使用して16に評価される以下の項：

B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L" L# ? B= v# I! I" B$ L$ B+ B$ v" v$ B$ v" v$ B- v# I" I%
研究者は、ベータ簡約の量に対する制限が参加者が直面する唯一の制限であると予想していますが、メモリ使用量や総実行時間にも（未知の）制限があるようです。

# 未知の演算子

上記の言語構造セットは研究者が発見したすべてであり、カルトが地球への通信において他の何かを使用することはないと推測されています。しかし、他の言語構造が存在するかどうかは不明です。