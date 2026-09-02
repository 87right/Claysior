
---

# HUD の一般化

**ひとまずスパゲティを許容**

理想コード
```rust
// 生成するとき
commands.gui(Transform::from_xyz(/* 座標 */))
    .button()
    .display::<DisplayComponent>(|value| value.get_bool_mut());

fn display<T: Component>(&mut self, getter: Fn(&mut T) -> &mut bool) &'_ mut Self {
    // 適当な処理
}

// これを自動で実行させたい
// ついでに`T`に関して重複が無いように
// 特別な初期化が必要ないように
fn on_changed(
    displays: Query<(&mut Button, &mut T), Changed<T>>,
    /* 適当な引数 */
) {
    for (button, t) in displays {
        let value = button.getter(t); // これで好きな要素を取得
    }
}
```

妥協案の疑似コード
```rust
// 宣言
impl BasicNode for Conveyor {
    fn addons(&self) {
        // 使うものを宣言
        // 表示・非表示を切り替えたいので、処理と実体の登録を分離
        (
            GuiAddon::<MaterialSlotHud::<Item>, Inventory::<Item>>,
            SampleAddonA,
            SampleAddonB,
        )
    }
}

// 自動生成されるもの（GuiAddon が実装）
fn system(
    data: Query<&Inventory<Item>, Changed<Inventory<Item>>>,
    hud: Query<&mut MaterialSlotHud::<Item>>
) {
    for hud in hud {
        if let Ok(mut inventory) = data.get_mut(hud.target_entity) {
            hud.update(inventory);
        }
    }
}

// 生成
commands.entity(conveyor_entity)
    // gui を構成できる trait の中に type (target_type, handle_type) を持たせる
    // MaterialSlotHud::<Item> なら MaterialSlot::<Item> 
    // add_gui (GuiAddon が定義) がアクセス関数の定義を担当して、Fn(target_type) -> handle_type
    .add_gui::<MaterialSlotHud::<Item>>( /* アクセス関数 */ |inventory| { 
            inventory.get(SlotID(0)) 
        }
    );


```

---

# アイテム（`Displayable`）の動きを自動で表示する機能の実装

最上位関数の理想コード
```rust
fn temp(
    temp
) {
    for (from, to, time_need) in logs { // logs: { pos: GridPos, diff: Diff }
        if from.diff.content_remains() // from が空になっていない
        && let Some(to) = grid.get(to)
        && let Ok(to) = display_q.get_mut() {
            to.entity = Some(commands.spawn().id()); // 変更予定
        } else {
            let mut entity = None;
            if let Some(from) = grid.get(from)
            && let Ok(from) = display_q.get(from) {
                entity = Some(from.entity.take());
            }
            if let Some(to) = grid.get(to)
            && let Ok(to) = display_q.get_mut(to) {
                to.entity = entity;
            }
        }
    }
}
```

---

# この魔物の整理

```rust
fn logistics_system<T>(
    mut commands: Commands,
    mut channel_q: Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos, Entity)>,
    mut registered_slot: ResMut<RegisteredSlot>,
    grid: Res<GridEntityMap>,
) where
    T: Consumable,
{
    let mut t_moved: Vec<(Vec2, Vec2, u64, String)> = vec![];

    let mut active_tasks: Vec<(Port<T>, Entity, GridPos, usize)> = vec![];
    let mut passive_tasks: Vec<(Port<T>, Entity, GridPos, usize)> = vec![];
    for (channel, _, pos, e) in channel_q.as_readonly() {
        for (index, port) in channel.output.iter().enumerate() {
            active_tasks.push((*port, e, *pos, index));
        }
        for (index, port) in channel.pull.iter().enumerate() {
            passive_tasks.push((*port, e, *pos, index));
        }
    }
    for (port, e, from_pos, index) in active_tasks {
        let Some((Some(mut buff), time_cost)) = get_buff::<T>(&channel_q, port, e) else {
            continue;
        };
        let tasks = get_entity_tasks::<T>(&channel_q, port, e, &grid);
        for e2 in tasks {
            let mut pos = GridPos(ivec2(0, 0));
            let Some(item) = buff.content.val else {continue;};
            if e != e2 
            && input::<T>(&mut channel_q, &mut buff, e2, e, from_pos, &grid, time_cost, &mut registered_slot, &mut pos) {
                t_moved.push((from_pos.to_world_pos(), pos.to_world_pos(), time_cost, format!("textures/item/{}.png", item.get_id())));
                break;
            }
        }
        apply(&mut channel_q, e, buff, index);
    }
    for (pull_port, pull_entity, pull_pos, index) in passive_tasks {
        let tasks = get_entity_tasks::<T>(&channel_q, pull_port, pull_entity, &grid);
        let mut open_ports = vec![];
        for open_entity in tasks {
            if let Ok((channel, _, open_pos, _)) = channel_q.get(open_entity) {
                for port in &channel.open {
                    if port.grid.check(*open_pos, pull_pos)
                    && let Some((Some(buff), _)) = get_buff(&channel_q, *port, open_entity) {
                        open_ports.push((buff, open_entity, *open_pos));
                    }
                }
            }
        }
        for (buff, open_entity, open_pos) in &mut open_ports {
            let mut open_pulled = None;
            let Some(item) = buff.content.val else {continue;};
            if let Ok((mut channel, mut inventory, _, _)) = channel_q.get_mut(pull_entity) {
                let time_cost = channel.time_cost;
                if pull_entity != *open_entity
                && let Some(pull_port) = channel.pull.get_mut(index)
                && pull_port.insert(&mut inventory, &mut buff.content, time_cost, &mut registered_slot, pull_entity) {
                    open_pulled = Some((buff.clone(), *open_entity));
                    channel.inserted(PortType::Pull, index);
                    t_moved.push((open_pos.to_world_pos(), pull_pos.to_world_pos(), time_cost, format!("textures/item/{}.png", item.get_id())));
                }
            }
            if let Some((buff, open_entity)) = open_pulled 
            && let Ok((_, mut inventory, _, _)) = channel_q.get_mut(open_entity) {
                inventory.apply_buff(buff);
                break;
            }
        }
    }

    for (from, to, ticks, texture_source) in t_moved {
        crate::gui::util::spawn_free_sprite(&mut commands, from, to, ticks, texture_source);
    }
}
```

流れはきれいなのに、処理がここに集中している気がするので。

概要
1. 物流の起爆剤（`Output` / `Pull` の `Port`）を抽出
2. `Output` アプローチの処理
   1. `Output` を持つインベントリから処理するスロットのバッファを持ってくる（搬出元）
   2. `Output` の搬出先を取得
   3. 各搬出先の `Channel` にバッファを渡す（アイテム移動が行われたら `break`）
   4. もしアイテムの移動が行われたのなら
      1. 搬出元の `Inventory` に変更後 `buff` を適用
      2. `Output` に対して実行時処理を実行
3. `Pull` アプローチの処理
   1. `Pull` の搬入元を取得
   2. 各搬入元の `Open` な `Port` からバッファを取得
   3. バッファをもらって `Channel` に渡す（アイテム移動が行われたら `break`）
   4. もしアイテムの移動が行われたのなら
      1. 搬入元の `Inventory` に変更後 `buff` を適用
      2. `Pull` の実行時処理を実行
4. 移動したアイテムの情報を `GUI` に送信

守るべきこと:
* `Channel`内部のポート達 `Input` / `Output` / `Pull` / `Open` （ただし `Gather` はリストラ）を、`Channel` 内部に閉じ込める
* 関数の単一責任性
* 引数が多くなりすぎないよう、意味単位で区切ってラップする
* `Entity` を扱うのは、`Query` から直接取ってくる場合のみ。

じゃあ何をしてほしいか:
* `Inventory`
  * `MaterialSlotBuff` の適用
  * `MaterialSlot` の発行（コピー）
  * `InventorySlice` の認識
  * `quick_insert` 

```rust
fn logistics_system<T>(
    mut log_node_q: Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos)>,
    grid: Res<GridEntityMap>
) 
where 
    T: ManuMaterial
{
    let mut orders = Vec::<LogisticsOrder::<T>>::default();
    for (mut channel, _, pos) in &mut log_node_q {
        // タスクの確保
        channel.pull_order(*pos, &mut orders);
    }
    for mut order in orders {
        // from から書き込み
        if let Some(from_entity) = grid.get(order.from)
        && let Ok((mut from_channel, mut from_inventory, _from_pos)) = (&mut log_node_q).get_mut(from_entity) {
            from_channel.write_order(&mut from_inventory, &mut order);
        } else {
            continue;
        }
        // to が受け取る
        if let Some(to_entity) = grid.get(order.to)
        && let Ok((mut to_channel, mut to_inventory, _to_pos)) = (&mut log_node_q).get_mut(to_entity) {
            to_channel.response_order(&mut to_inventory, &mut order);
        } else {
            continue;
        }
        // from が変更を受け取る
        if let Some(from_entity) = grid.get(order.from)
        && let Ok((mut from_channel, mut from_inventory, _from_pos)) = (&mut log_node_q).get_mut(from_entity) {
            from_channel.check_order(&mut from_inventory, &order);
        } else {
            continue;
        }
    }
}
```

あとはこれに、`ManuMaterial` の移動通知をくっつけるだけ

それと、等分配機能の実装もできそうなら
