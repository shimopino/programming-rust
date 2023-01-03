// キューで保持するフィールドを含むをデータ構造を定義する
pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

// implブロック内の関数は、特定の型に紐づいているため関連関数と呼ぶ
// implブロック外の関数は、反対に自由関数と呼ぶ
impl Queue {
    // selfを引数にとらない場合、型関連関数と呼ばれる
    // コンストラクタとして提供されることが多い
    // 慣習的に new がメソッド名として使われることが多い
    pub fn new() -> Queue {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    /// 文字をキューの後ろに追加する
    // 第1引数は自身であることは確定なので「self」に型定義は必要ない
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    /// キューの先頭から文字を取得する
    /// 要素が存在している場合には Some(c)、存在しない場合は None を返す
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // youngerが存在している場合には older に移して順番を入れ替える
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        // olderには必ずなにか要素が含まれているはず
        // Vecのpopメソッドをそのまま返す
        self.older.pop()
    }

    // 自信を変更しない場合は、共有参照の形式で引数を指定すればいい
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    // 呼び出し元に所有権を渡すこともできる
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

#[test]
fn queue_test() {
    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);

    assert!(q.is_empty());
    q.push('☉');
    assert!(!q.is_empty());
}

#[test]
fn split_test() {
    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };

    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    // 所有権は移ったので、q 自体は未定義状態になってしまった
    // 以下のように指定すると既に移動済みのエラーが表示される
    // q.older;
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);
}

#[test]
fn constructor_test() {
    let q = Queue::new();

    assert_eq!(q.older.len(), 0);
    assert_eq!(q.younger.len(), 0);
}
