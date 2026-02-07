# Conway Obfuscate in Rust

[Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

- The idea for this project came from this [video](https://www.youtube.com/watch?v=fJbAiXIum0k)
- Tsoding's [project](https://github.com/tsoding/conway)

```rust
                    fn main(){let mut
                    f= [['.';16]; 8];
                    f[0][1]='#'; f[1]
                    [2]='#';f[2][0] =
                    '#';f[2][1] ='#';
                    f[2][2]='#';loop{

                                        print!("\x1b[2J")
                                        ;print!("\x1b[H")
                                        ; for y in 0..8 {
                                        for x in 0..16 {
                                        print!("{}", f[y]
                                        [x]);}println!()}

let mut b=f.clone   ();for y in 0..8{   for x in 0..16{let
mut o=0;for dy in   -1..2{for dx in-1   ..2{if dx==0&& dy
==0{continue;}let   ny=(y as isize+dy   ).rem_euclid(8)as
usize;let nx=(x as  isize+dx) /*###*/   .rem_euclid(16)as
usize;if f[ny][nx]  =='#'{o+=1;}}}b[y]  [x]=match(f[y][x]
,o){('#',2)|('#',   3)=>'#',('.',3)=>   '#',_=>'.'};}}f=b

;for _ in 0..50000000{}}} // sleeep
```

# Files

- [main.rs](./src/main.rs) - Obfuscated code
- [main_normal.rs](./src/main_normal.rs) - Original code

# Build

```console
C:\...\conway_obfus> cargo run
```