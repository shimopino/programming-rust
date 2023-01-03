// ジェネリクスを定義する
pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

// もしも特定の型に対してのみメソッドを定義したい場合は下記のように宣言する
// impl Queue<f64> { ... }
// implではジェネリクスを下記のように定義する
impl<T> Queue<T> {
    // Queue<T> の代わりに特別な型パラメータ Self を使うことでより省略できる
    // pub fn new() -> Queue<T> {
    pub fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    // 呼び出し元に所有権を渡すこともできる
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

#[test]
fn generics_test() {
    // 初期化時に型を指定することができる
    let mut q = Queue::<char>::new();

    // Rustに型を遅延評価させることも可能 -> Queue<&str>
    let mut r = Queue::new();
    r.push("CAD");
    r.push("BTC");

    // Queue<f64>
    let mut s = Queue::new();
    s.push(0.74);
    s.push(13764.0);
}
