https://do7be.github.io/try-rust-bevy/

## 既知の問題点

- [x] ジャンプ時に天井を突き抜ける
- [x] ジャンプできないタイミングがある（ジャンプ連打を無効化した際に発生）
- [ ] ジャンプ時に微妙に壁を昇れるタイミングがある
- [x] 天井が横ぎりぎり当たる位置でジャンプするとすり抜けて上に乗れる
- [ ] ギリジャンのとこがかなり難しくなっている（横移動で落ちるかどうかを先に判定しているから？）
- [x] エンディング後に色々リセットできていない
- [ ] 重いので、横移動で床があるかの判定を高速化する（現在の場所から 1/2 タイルサイズ分下の 2 タイルに壁があるかチェックするようにする）
- [ ] 武器の速度調整
  - [ ] プレイヤー
  - [ ] 敵
  - [x] ボス
- [x] 各種 Z 軸の表示優先度調整
  - [x] 敵の武器をプレイヤーより前にする
- [ ] WebGL で最初に画像を表示するときにロードが走るので一瞬透明になる

## 残り

- [x] ソード
- [x] サンダー
- [x] 武器の残弾数
- [x] 武器ドロップ
- [x] レッドデーモン
  - [x] 弾
  - [x] プレイヤーの方向に撃つ
  - [x] 発射間隔
- [x] ウィザード
  - [x] 弾
  - [x] 発射間隔
- [x] すべての敵配置
- [x] 画面外の敵は動きを止める
- [ ] 画面外から再び画面内に入ったら敵は復活
- [x] エンディング
- [x] ボスの位置まで到達したら StageStatus を Boss にする
- [x] ボス戦に入ったら敵を全消去する
- [x] ボスの体力が 0 になったらエンディング
- [x] ボスがダメージを受けたら点滅
- [x] ボスの体力を表示
- [x] ボスの行動全般
  - [x] 移動
  - [x] ブルーファイア
  - [x] ウォーターバルーン
  - [x] ダークサンダー
  - [x] メテオ
- [x] ボス戦入ったらコンテニュー時の位置変更
- [x] WebGL
