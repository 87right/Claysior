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
