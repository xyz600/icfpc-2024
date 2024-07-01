Lambda-Manコースへようこそ。

それは2014年のことで、私たちのコミュニティの多くのメンバーがLambda-Manを操作するために一生懸命働きました。今、10年後、この素晴らしいイベントは小さなLambda-Manコンペティションを開催することでまだ記憶されています。

このコースでは、すべてのピルを食べるためにLambda-Manを最適に操作する方法を教えます。果物（低い位置のものも高い位置のものも）関係なく、さらに良いことに、お化けもいません！各問題への入力は、以下のような単純な長方形のグリッドです：

```
###.#...
...L..##
.#######
```

このグリッドには正確に1つの `L` キャラクターが含まれており、これはLambda-Manの開始位置です。食べるべきピルの位置を示す `.` キャラクターが1つ以上あり、 `#` キャラクターは壁を示します。グリッドの外側の境界は壁であると見なされます。

解答は、進むべき経路を示す `U`, `R`, `D`, `L`（それぞれ上、右、下、左）の文字列であるべきです。例えば、上記の例のグリッドに対する可能な解答経路は以下の通りです：
```
LLLDURRRUDRRURR
```
Lambda-Manが壁を含むマスに移動するよう指示された場合、何も起こらず、その指示はスキップされます。解答は最大で `1,000,000` 文字で構成されることができます。

以下のレベルが利用可能です：
* [lambdaman1] Your score: 33. Best score: 33.
* [lambdaman2] Your score: 44. Best score: 44.
* [lambdaman3] Your score: 58. Best score: 58.
* [lambdaman4] Your score: 413. Best score: 111.
* [lambdaman5] Your score: 197. Best score: 105.
* [lambdaman6] Your score: 217. Best score: 73.
* [lambdaman7] Your score: 415. Best score: 111.
* [lambdaman8] Your score: 9809. Best score: 113.
* [lambdaman9] Your score: 2525. Best score: 109.
* [lambdaman10] Your score: 2320. Best score: 113.
* [lambdaman11] Your score: 10344. Best score: 132.
* [lambdaman12] Your score: 10215. Best score: 130.
* [lambdaman13] Your score: 10014. Best score: 129.
* [lambdaman14] Your score: 10013. Best score: 129.
* [lambdaman15] Your score: 10040. Best score: 129.
* [lambdaman16] Your score: 16394. Best score: 134.
* [lambdaman17] Your score: 2290. Best score: 113.
* [lambdaman18] Your score: 11178. Best score: 117.
* [lambdaman19] Your score: 16934. Best score: 237.
* [lambdaman20] Your score: 16704. Best score: 177.
* [lambdaman21] Best score: 114.

解答を提出するには、次のICFP式を送信してください：

```
solve lambdamanX path
```

あなたのスコアはICFP式のバイト数（つまり、POSTボディのサイズ）であり、スコアが低いほど良いです。