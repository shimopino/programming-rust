fn main() {
    // Rustにはメモリアドレスを表すための型がある
    // 例えばJavaでは、class RectangleにフィールドVector2D upperLeft;があるとする
    // upperLeftは別に作られたVector2Dオブジェクトへの参照となる
    // Javaではあるオブジェクトに別のオブジェクトが物理的に含まれることはない

    // Rustはメモリ消費を最小するために値はデフォルトでネストする
    // ((0, 0), (1440, 900))は4つの隣接した整数として格納される

    // メモリ効率的には良いが、結果的に、ある値から別の値を指す場合には、必ず明示的にポインタを使わなければならない
    // ただし、ポインタ型は未定義動作を使用しないように制限されており、安全にポインタを使うことができる

    // 参照
    // &StringはString値への参照であり、&i32はi32値への参照
    // 実行時にはi32への参照はそのアドレスを保持した１ワード長のデータとなる
    // Rustでは「&x」は「xへの参照を借用する」という
    // 参照rに対して「*r」はrが指す値を取得する
    // またRustの参照はnullにはならない

    // &T
    // 変更不能な共有参照
    // 値を読み出すことしかできず、参照先の値を変更できない

    // &mut T
    // 排他的な可変参照
    // 参照先の値を読み出し、変更することが可能である
    // ある値に対してこの種の参照が存在する間は、その値に対する他の参照は共有参照であれ可変参照であれ作ることはでき

    // Box
    // ヒープ上に値を確保する簡単な方法は Box::new を使う
    // tの型は(i32, &str)なので、bの型はBox<(i32, &str)>
    // Box::new()はタプルを格納するのに十分なメモリをヒープ上に確保する
    // bがスコープから外れるとメモリは即座に解放される
    let t = (12, "eggs");
    let b = Box::new(t); // ヒープ上にタプルを確保

    // rawポインタ
    // 「*mut T」や「*const T」などのrawポインタ型
    // 安全ではない
    // unsafeブロックの中でしか使うことができず、オプトイン式の機構
}
