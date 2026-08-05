# TODO リスト beta

> 主な目標点
> 1. `Query<..., With<LeftClick>>` などの `System Poling` ベースから、`Observer` を用いた `Trigger` ベースに移行。
> 2. `on_placed`（旧 `BasicNode trait` 内） 相当の処理を、`ComponentHook` を用いた自動発火へ移行する。
> 3. マップサイズを可変にする。
> 4. 物流をグラフ化して、物流システムをよりきれいにする。
> 5. `Sprite` の再利用を徹底して、アニメーションの負荷・滑らかさの改善。
> 6. 描画システムを `GUI` へ統合

- [x] `Camera` の初期化処理実装 
- [x] `Camera Movement System` を、`InputLayer::Camera` に実装
- [x] `Camera Movement System` のマウスミドルクリック参照をマウス左クリック参照に変更
- [x] `Camera Movement System` に `WASD` 移動を追加
- [ ] `Camera Zoom System` を、`scale` 大きさに応じて変化を大きくする
- [ ] `Camera Zoom System` がカーソル位置を考慮するようにする
- [ ] `Grid` にワールド初期化を実装
- [ ] `Consumable` に `Inventory<T>` と `Channel<T>` を実装
- [ ] `Grid` に `Trigger` たちを生やす
- [ ] `Node` のテスト実装（コンベア）
- [ ] `GUI` のテスト実装（`Node` の状態管理）
- [x] FPS 表示機能（クォリティ問わず）


:::archived
# TODO リスト alpha

alpha 時代の産物
- [x] クリックシステムの復旧
- [x] Fixed Update への移行
- [x] コンベアのテクスチャ差分追加
- [x] 物流の一般化
- [ ] display 用アイテムの追加
- [x] レシピの一般化
- [x] コンベアの port を変更できるようにする
- [ ] ワールド再生成機能追加
- [ ] 鉱石自動生成に伴う、生成ルールの形式化
- [x] 機械の追加（粘土加工台、粘土炉）（側だけ）
- [x] Port に Mode を追加（即時、一度、インターバルありなど）
- [ ] Recipe に RON ファイルを自動で読み込む機能を追加
- [ ] UI 規格を決定
- [x] 滑らかなアイテム移動を実装
- [x] Grid System 専用の ButtonInput<MouseButton> を追加
- [ ] PortMode::Always に、動作速度の上限を追加
- [ ] GUI に種類を持たせる（ポップアップ、フルスクリーンインベントリ、HUD）
- [x] カメラ操作も入力のレイヤに追加
- [ ] ウィンドウサイズ変更時のGUI再配置をなんとかする
- [ ] アイテム分配用の機械を追加
- [x] MaterialSlot に max volume を追加
- [ ] Channel に、代理 Inventory を追加（マルチブロック施設のため）
- [ ] TextureBuff を Atlas に対応させる
- [x] Channel に搬入アプローチ型の PortType::{Open, Pull} を作成
- [ ] Channel の gather port の具体処理を実装
- [ ] PortType::Pull により Filter 付き搬入を行うと適切なものが取れないバグの修正
- [x] Item Unloader の追加
- [x] 粘土を無限に生成する Clay Generator, 入ったアイテムを虚空に葬る Trash Can の実装
- [x] Slot への insert 時、u64 に対して 0 を下回る可能性のある計算を変更
- [ ] TextureBuff を GUI に集約
- [x] MaterialSlot<T> に、搬入の予約情報を追加する
- [x] 搬入の予約情報を更新するようにする
- [x] slot へ搬入時、T の上限または slot の上限を超えた際の処理における、搬入元から取りつくした際の slot.val がリセットされないバグを修正
- [x] オーバーフロー対策のフィルタが、物流の設計と矛盾しているのを修正
- [x] アイテム搬出が、搬出先の slot が空の場合に上限を超えて搬出されることがあるのを修正
- [ ] Inventory<T> の vol 計算（に伴う val 更新）の責任を Inventory に持たせる
- [ ] Channel に「アイテムを表示するかどうか」の情報を持たせる
:::
