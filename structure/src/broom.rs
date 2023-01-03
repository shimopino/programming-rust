// Enumも有するモンスターの構造体から、「..」でのアクセスを考える
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Copy, Clone)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

// 値としてBroomを受け取って、所有権も受け取る
fn chop(b: Broom) -> (Broom, Broom) {
    // 高さを半分にする
    // StringはCopyではないため、broom1はb.nameの所有権を得る
    let mut broom1 = Broom {
        height: b.height / 2,
        // スプレッドで残りのパタメータを展開する
        ..b
    };

    // nameはCopyではないため、明示的にクローンする
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    // それぞれ別の名前を与える
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

#[test]
fn chop_test() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.height, 30);
    assert_eq!(hokey2.health, 100);
}
