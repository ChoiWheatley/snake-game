# 뱀게임 with Rustlang

[다음 튜토리얼](https://www.youtube.com/watch?v=DnT_7M7L7vo)을 참고함.

## How To Build

1. Ensure you have rust build tool installed
2. Clone this repo
3. `cargo run`

## 종료조건

- 뱀 머리가 자기 몸통에 닿았을때
- 뱀 머리가 벽에 닿았을때

## 점수 획득조건

- 뱀 머리가 랜덤한 위치에 생성된 사과에 닿았을때. 이때 뱀의 길이가 1 증가한다.

## Dependency

- [piston_window](https://docs.rs/piston/0.52.0/piston/) A modular game engine written in Rust
- [rand](https://docs.rs/rand/latest/rand/) Utilities for random number generation

## piston_window

- window 객체

```rust
let mut window: PistonWindow =
    WindowSettings::new("Snake game", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
```

- event listener

이벤트가 마치 이터레이터처럼 되어있는게 신기하다. 마우스 클릭이나 키보드 누름, 다음 프레임 등 어떤 이벤트가 호출될 때까지 일종의 busy wait를 하는 것으로 보인다.

```rust
while let Some(event) = window.next() {
}
```

- key pressed event

event의 메서드 중에 `press_args()` 를 추출하여 키보드 입력을 확인한다.

```rust
if let Some(Button::Keyboard(key)) = event.press_args() {
    do_something_with(key);
}
```

- draw event

`g2d`는 [GfxGraphics](http://docs.piston.rs/piston_window/gfx_graphics/struct.GfxGraphics.html) 를 의미하며, 내부적으로 2D 그래픽을 렌더링 할 때 사용한다.

```rust
window.draw_2d(&event, |context, g2d, _device| {
    clear(color, g2d);
    rectangle(color, [x, y, width, height], context.transform, g2d);
});
```

- update

여타 게임엔진들과 같이 매 프레임마다 호출되는 `update` 함수가 있다.

```rust
event.update(|arg| do_something_with(arg.dt));
```

## verdict of piston_window

문서화가 거의 되어있지 않아 아쉽다. 하지만 OpenGL기반이라서 플랫폼 독립적이라는 점은 좋은 것 같다.
