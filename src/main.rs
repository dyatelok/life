use raylib::prelude::*;
use euler::*;
use rand::Rng;

//Размера поля
const X : usize = 150;
const Y : usize = 96;

//Массивы поля и ботов
static mut P : [[Option<usize>; Y]; X] = [[None; Y]; X];
static mut Bots : Vec<bot> = Vec::new();

//Глобальные константы для ботов
const max_e  : i32 = 999; //Максимальный показатель энергии
const max_m  : i32 = 999; //Максимальный показатеь минералов
const mut_ch : f32 = 0.5; //Шанс мутации
const lifetime : i32 = 1500; //Ограничение времени жизни бота в ходах. Если -1, то боты безсмертны.
const commands : usize = 64; //Расмер массива команд


struct bot {
    x : usize,      //Координаты на поле
    y : usize,
    e : i32,        //энергия бота
    m : i32,        //минералы бота
    c : Color,      //Цвет бота
    t : i32 ,       //Жив ли бот? 1 -> да 0 -> трупик 2 -> отсутствует полностью
    g : [usize; commands],//Геном бота
    G : usize,      //Указатель накоманду
    dir : i32,      //Направление бота 
                    //   107
                    //   2 6
                    //   345
    l : i32         //Оставшееся время до смерти бота
}

impl bot {
    fn new(x : usize, y : usize) -> bot {
        unsafe { P[x][y] = Some(Bots.len()); }
        let mut b = bot {
            x : x,
            y : y,
            e : 10,
            m : 0,
            c : Color::WHITE,//Color{r : 0, g : 0, b : 0, a : 255},
            t : 1,
            g : [43; commands],
            G : 0,
            dir : 2,
            l : lifetime,
        };
        b
    }
    fn repl(b : usize, xy : (i32, i32)) {
        unsafe { 
        P[((Bots[b].x as i32 + X as i32 + xy.0) % X as i32) as usize][((Bots[b].y as i32 + Y as i32 + xy.1) % Y as i32) as usize] = Some(Bots.len());
        Bots[b].m /= 2;
        Bots.push(
        bot {
            x : ((Bots[b].x as i32 + X as i32 + xy.0) % X as i32) as usize,
            y : ((Bots[b].y as i32 + Y as i32 + xy.1) % Y as i32) as usize,
            e : Bots[b].e,
            m : Bots[b].m / 2,
            c : Color::WHITE,
            t : 1,
            g : Bots[b].g,
            G : 0,
            dir : Bots[b].dir,
            l : lifetime,
        })};
    }
    fn mutate(b : usize, xy : (i32, i32)) {
        unsafe {
        let mut rng = rand::thread_rng();
        P[((Bots[b].x as i32 + X as i32 + xy.0) % X as i32) as usize][((Bots[b].y as i32 + Y as i32 + xy.1) % Y as i32) as usize] = Some(Bots.len());
        Bots[b].m /= 2;
        let mut bt = bot {
            x : ((Bots[b].x as i32 + X as i32 + xy.0) % X as i32) as usize,
            y : ((Bots[b].y as i32 + Y as i32 + xy.1) % Y as i32) as usize,
            e : Bots[b].e,
            m : Bots[b].m / 2,
            c : Color::WHITE,
            t : 1,
            g : Bots[b].g,
            G : 0,
            dir : Bots[b].dir,
            l : lifetime,
        };
        bt.g[rng.gen_range(0..commands)] = rng.gen_range(0..64);
        Bots.push(bt);
        }
    }
    fn inc_r(b : usize, i : i32) {
        unsafe{ Bots[b].c.g = (Bots[b].c.g as i32 -i).max(0) as u8; Bots[b].c.b = (Bots[b].c.b as i32 -i).max(0) as u8;}
    }
    fn inc_g(b : usize, i : i32) {
        unsafe{ Bots[b].c.r = (Bots[b].c.r as i32 -i).max(0) as u8; Bots[b].c.b = (Bots[b].c.b as i32 -i).max(0) as u8;}
    }
    fn inc_b(b : usize, i : i32) {
        unsafe{ Bots[b].c.r = (Bots[b].c.r as i32 -i).max(0) as u8; Bots[b].c.g = (Bots[b].c.g as i32 -i).max(0) as u8;}
    }
    /*fn inc_r(b : usize, i : i32) {
        unsafe{ Bots[b].c.r = (Bots[b].c.r as i32 +i).max(0).min(255) as u8;}
    }
    fn inc_g(b : usize, i : i32) {
        unsafe{ Bots[b].c.g = (Bots[b].c.g as i32 +i).max(0).min(255) as u8;}
    }
    fn inc_b(b : usize, i : i32) {
        unsafe{ Bots[b].c.b = (Bots[b].c.b as i32 +i).max(0).min(255) as u8;}
    }*/
    fn dec_e(b : usize, i : i32) {
        unsafe{ Bots[b].e = (Bots[b].e as i32 - i).max(0).min(max_e); } 
    }
    fn mv(b : usize, xy : (i32, i32)) { //Передвигаем бота в относительном направлении 
    unsafe {
        if bot::who(b,xy) == Some(0) { //В клетку невозможно пройти -> ничего не делаем
        P[Bots[b].x][Bots[b].y] = None;
        Bots[b].x = ((Bots[b].x as i32 + X as i32 + xy.0) % X as i32) as usize;
        Bots[b].y = ((Bots[b].y as i32 + Y as i32 + xy.1) % Y as i32) as usize;
        P[Bots[b].x][Bots[b].y] = Some(b);
        }
    }
    }
    fn genom_diff(a : usize, b : usize) -> i32{
    unsafe {
        let mut c = 0;
        for i in 0..commands {
            if Bots[a].g[i] != Bots[b].g[i] {c+=1;}
        }
        c
    }
    } 
    
    fn del(b : usize, xy : (i32, i32)) { //Подготавливаем бота к удалению
        unsafe {
            if bot::who(b,xy) != None {
                match P[((Bots[b].x as i32 + X as i32 + xy.0) % X as i32) as usize][((Bots[b].y as i32 + Y as i32 + xy.1) % Y as i32) as usize] {
                    Some(a) => {
                        Bots[a].t = 2;
                    }
                    _ => {}
                }
            }
        }
    }
    fn dir2xy(d : i32) -> (i32,i32) {
        match d {
            0 => {return ( 0,-1);}
            1 => {return (-1,-1);}
            2 => {return (-1, 0);}
            3 => {return (-1, 1);}
            4 => {return ( 0, 1);}
            5 => {return ( 1, 1);}
            6 => {return ( 1, 0);}
            7 => {return ( 1,-1);}
            _ => {return ( 0, 0);}
        }
    }
    fn upd(b : usize) { //Обновляем состояние бота
    unsafe {
        match Bots[b].t  {
            1 => {//Если бот жив, то выполняем команду, пытаемся размножиться и пополняем минерылы
            bot::add_m(b,bot::minerals_from_level(Bots[b].y));
            bot::try_replicate(b);
            bot::command(b);
            Bots[b].l -= 1;
            if  Bots[b].e == 0 || Bots[b].l == 0 { //Проверяем, энергию бота. Если его энергия равна нулю, то бот умирает и становится трупом/органикой
                Bots[b].c = Color::from((160,160,160,255)); 
                Bots[b].t = 0;
        } }
            0 => { //Если бот мертв, то проверяем, может ли он упасть вниз, если так, то делаем это
            match bot::who(b,(0,1)) {
                Some(0) => {bot::mv(b,(0,1));}
                _ => {}
            }
        }
            _ => {} //Если бот полностью мертв, то ничего не делаем
        }
    }
    }
    fn inc_C(b : usize, G : usize) { //Обновляем номер исполняемой инструкции
        unsafe{ Bots[b].G = (Bots[b].G + G) % commands; }
    }
    fn eat(b : usize, xy : (i32,i32)) {
    unsafe {
        match bot::who(b,xy) {
            Some(1) => { //труп
                bot::add_e(b,100);
                bot::del(b,xy);
                bot::inc_r(b,100);
            }
            Some(2) => { //другой бот
                match P[((Bots[b].x as i32 + X as i32 + xy.0) % X as i32) as usize][((Bots[b].y as i32 + Y as i32 + xy.1) % Y as i32) as usize] {
                    Some(a) => {
                        if Bots[b].m > Bots[a].m {
                            Bots[b].m -= Bots[a].m;
                            Bots[a].t = 2;
                            let cl = 100 + (Bots[a].e / 2);
                            bot::add_e(b,cl);
                            bot::inc_r(b,cl);
                        } else {
                            Bots[a].m -= Bots[b].m;
                            Bots[b].m = 0;
                            if Bots[b].e  >= 2 * Bots[a].m {
                                Bots[a].t = 2;
                                let cl = 100 + (Bots[a].e / 2) - 2 * Bots[a].m;
                                bot::add_e(b,cl);
                                bot::inc_r(b,cl);
                            } else {
                                Bots[b].t = 2;
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }}
    fn give(b : usize, xy : (i32,i32)) {
    unsafe {
        match bot::who(b,xy) {
            Some(2) => { //другой бот
                match P[((Bots[b].x as i32 + X as i32 + xy.0) % X as i32) as usize][((Bots[b].y as i32 + Y as i32 + xy.1) % Y as i32) as usize] {
                    Some(a) => {
                        let eng = (Bots[b].e + Bots[a].e) / 2;
                        Bots[b].e = eng;
                        Bots[a].e = eng;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }}
    // Список команд, которые будет выполнять бот и их значения
    // 0-7 команды передвижения в клеrrи абсолютно
    // 107
    // 2 6
    // 345
    // 8-15 сменить направление абсолютно
    // 16-23 съесть в абсолютном направлении
    // 24-31 посмотреть в абсолютном направлении
    // 32-39 поделиться в абсолютном направлении
    // 40 передвижение относительно направления в котором смотрит бот
    // 41 , 42 сменить направление относительно налево - направо
    // 43 фотосинтез
    // 44 съесть в относительном направлении   
    // 45 посмотреть в относительном направлении
    // 46 поделиться в относительном направлении
    // 47 минералы в энергию
    fn command(b : usize) { //Основной блок с номерами инструкций и их содержимым
        unsafe {
        let Q = Bots[b].g[Bots[b].G] as i32;
        match Q {
                 0..=7   => {bot::mv(b,bot::dir2xy(Q)); bot::inc_C(b,1); bot::dec_e(b,1);}
                 8..=15  => {Bots[b].dir=Q-8; bot::inc_C(b,1); bot::dec_e(b,1);}
                 16..=23 => {bot::eat(b,bot::dir2xy(Q-16)); bot::inc_C(b,1);}
                 24..=31 => {bot::inc_C(b,bot::who2n(b,bot::dir2xy(Q-24))); bot::dec_e(b,1);}
                 32..=39 => {bot::give(b,bot::dir2xy(Q-32)); bot::inc_C(b,1); }
                 40      => {bot::mv(b,bot::dir2xy(Bots[b].dir)); bot::inc_C(b,1); bot::dec_e(b,1);}
                 41      => {Bots[b].dir = (Bots[b].dir + 1) % 8; bot::inc_C(b,1); bot::dec_e(b,1);}
                 42      => {Bots[b].dir = (Bots[b].dir + 7) % 8; bot::inc_C(b,1); bot::dec_e(b,1);}
                 43      => {bot::add_e(b,bot::energy_from_level(Bots[b].y)); bot::inc_g(b,2); bot::inc_C(b,1);}
                 44      => {bot::eat(b,bot::dir2xy(Bots[b].dir)); bot::inc_C(b,1);}
                 45      => {bot::inc_C(b,bot::who2n(b,bot::dir2xy(Bots[b].dir))); bot::dec_e(b,1);}
                 46      => {bot::give(b,bot::dir2xy(Bots[b].dir)); bot::inc_C(b,1);}
                 47      => {let m = Bots[b].m.min(100); bot::add_e(b,m * 4); Bots[b].m = (Bots[b].m - 100).max(0); bot::inc_b(b,m); bot::inc_C(b,1); bot::dec_e(b,1);}

                 _ => {bot::inc_C(b,Bots[b].g[Bots[b].G]);} //Если команда не находится в наборе стандартных инструкций, 
            }                                               //то мы увеличиваем номер исполняемой команды на номер инструкций
        }
    }
    fn energy_from_level(y : usize) -> i32{
        0.max((60 - y as i32) / 10)
    }
    fn minerals_from_level(y : usize) -> i32{
        0.max((60 - (Y - y) as i32) / 20)

    }
    fn add_e(b : usize, e : i32) {
        unsafe{ Bots[b].e = (Bots[b].e + e).min(max_e).max(0); }
    }
    fn add_m(b : usize, m : i32) {
        unsafe{ Bots[b].m = (Bots[b].m + m).min(max_m).max(0); }
    }
    fn try_replicate(b : usize) { //Бот пробует размножиться, замещая собой окружающие пустые клетки, делая 3 попытки
    unsafe {
        if Bots[b].e >= max_e * 3 / 4 {
            let mut rng = rand::thread_rng();
            let mut xx : i32;
            let mut yy : i32;
            let r : f32;
            for _ in 0..3 {
                let xy = (rng.gen_range(-1..2), rng.gen_range(-1..2));
                let s = bot::who(b,xy);
                match s {
                    Some(0) => {    bot::dec_e(b,max_e / 3); r = rng.gen();
                                    if r <= mut_ch {bot::mutate(b,xy);} else {bot::repl(b,xy); }
                                    break;
                               }
                    _ => {}
                }
            }
        }
    }
    }
    fn who(b : usize, xy : (i32, i32)) -> Option<u8> {                                                       //Посмотреть, кто находится в клетке относительно бота
    unsafe {                                                                                                    //в клетку невозможно пройти -> None
        if !(0 <= Bots[b].y as i32 + xy.1 && Bots[b].y as i32 + xy.1 < Y as i32) {return None;}                     //никого -> Some(0)
        match P[((Bots[b].x as i32 + X as i32 + xy.0) % X as i32) as usize][((Bots[b].y as i32 + xy.1 + Y as i32) % Y as i32) as usize] {   //трупик -> Some(1)
            None => {return Some(0);}                                                                           //бот    -> Some(2) 
            Some(a) =>  {
                            match Bots[a].t {
                                0 => {return Some(1);} 
                                1 => {return Some(2);}
                                _ => {return Some(0);}
                        }
        }}
    }
    }
    fn who2n(b : usize, xy : (i32, i32)) -> usize {                                                       //Посмотреть, кто находится в клетке относительно бота и исходя из этого сделать 
    unsafe {                                                                                             // на выходе   2-пусто  3-стена  4-органика 5-бот 6-родня
        if !(0 <= Bots[b].y as i32 + xy.1 && Bots[b].y as i32 + xy.1 < Y as i32) {return 3;}            
        match P[((Bots[b].x as i32 + X as i32 + xy.0) % X as i32) as usize][((Bots[b].y as i32 + xy.1 + Y as i32) % Y as i32) as usize] {
            None => {return 2;}
            Some(a) =>  {
                            match Bots[a].t {
                                0 => {return 4;} 
                                1 => {if bot::genom_diff(a,b) <= 1 {return 6;} else {return 5;}}
                                _ => {return 0;}
                        }
        }}
    }
    }
}

fn main() {
    unsafe {
    let pixel_size : i32 = 7;
    let (mut rl, thread) = raylib::init()
        .size(X as i32 * pixel_size, Y as i32 * pixel_size)
        .title("life")
        .build();

    Bots.push(bot::new(0,0));
    let mut Q : bool;
    let mut l = 1;
    let mut T = 0.0;
    rl.set_target_fps(0);
    while !rl.window_should_close() {
        let tt = rl.get_frame_time();
        let k = rl.get_key_pressed();
        match k {
            Some(K) => {
                if K == KeyboardKey::KEY_L {if l == 100 {l = 1;} else {l = 100;}}
            }
            _ => {}
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        for b in 0..16_i32 {
            d.draw_rectangle(0,b * pixel_size * 8,X as i32 * pixel_size, pixel_size * 8, Color::from((0,0,255,16 * b as u8))); 
        }
        for b in 0..Bots.len() {
            d.draw_rectangle((Bots[b].x as i32 * pixel_size) + pixel_size / 6,     (Bots[b].y as i32 * pixel_size) + pixel_size / 6, 
                                    pixel_size - pixel_size / 3, pixel_size - pixel_size / 3, Bots[b].c); 
        }
        for _ in 0..l {
        //Обновляем ботов
        let mut B = 0;
        while B < Bots.len() {
            bot::upd(B);
            B+=1;
        }
        //Уничтожаем полностью мертвых ботов
        let mut BotsN : Vec<i32> = vec!(-1;Bots.len());
        let mut B = 0;
        for i in 0..Bots.len() {
            if Bots[i].t != 2 {BotsN[i]=B; B+=1;}
        }
        for i in 0..X {
            for j in 0..Y {
                match P[i][j] {
                    Some(a) => {
                        P[i][j] = Some(BotsN[a] as usize);}
                    _ => {}
                }
            }
        }
        let mut B = 0;
        loop {
            if Bots[B].t == 2 {P[Bots[B].x][Bots[B].y] = None; Bots.remove(B);} else {B+=1;}
            if B >= Bots.len() {break;}
        }
        }
        Q = true;
        T = 0.0;
        for i in 0..Bots.len() {
            if Bots[i].t == 1 {Q=false; T+=1.0;}
            if Bots[i].t == 0 {T+=0.2;}
        }
        if Q {P = [[None; Y]; X];
                   Bots = Vec::new();
                   Bots.push(bot::new(0,0));} //Перезапуск при полном вымирании
        println!("{}",tt / l as f32 / T as f32);
    }
    }
}
