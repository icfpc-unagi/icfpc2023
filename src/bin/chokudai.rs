#![allow(unused_imports)]
use icfpc2023::{self, Input, read_input, P, mcf::weighted_matching, write_output, compute_score};

fn main() {

    let inp = read_input();

    //let mut dist = Vec::new();

    /*
    for p in inp.pos{
        let d = get_stage_diff(p, inp.stage0, inp.stage1);
        dist.push((d, p));
        //dbg!(d);
    }
    dist.sort_by(|a, b| a.partial_cmp(b).unwrap());
    */

    let mut candidate = Vec::new();

    let mut ar = Vec::new();
    for _ in 0..inp.musicians.len() {
        ar.push(Vec::new());
    }    
    
    let stage_x = ((inp.stage1.0 - inp.stage0.0) / 10.0) as usize;
    let stage_y = ((inp.stage1.1 - inp.stage0.1) / 10.0) as usize;

    //y=0
    for x in 1..stage_x - 1  {
        let nx =  inp.stage0.0 + (x as f64) * 10.0;
        let ny =  inp.stage0.1 + 10.0;

        candidate.push(P(nx, ny));
        
        for i in 0..inp.musicians.len() {
            let mut pt = 0;
            for p in 0..inp.pos.len() {

                if inp.pos[p].1 < ny {
                    pt += ( inp.tastes[p][inp.musicians[i]] / (inp.pos[p] - P(nx, ny)).abs2() * 1000000.0) as i64;
                }
            }
            ar[i].push(pt);
        }
    }

    //x=0
    for y in 2..stage_y - 1 {
        let nx =  inp.stage0.0 + 10.0;
        let ny =  inp.stage0.1 + (y as f64) * 10.0;
        candidate.push(P(nx, ny));
        
        for i in 0..inp.musicians.len() {
            let mut pt = 0;
            for p in 0..inp.pos.len() {

                if inp.pos[p].0 < nx {
                    pt += ( inp.tastes[p][inp.musicians[i]] / (inp.pos[p] - P(nx, ny)).abs2() * 1000000.0) as i64;
                }
            }
            ar[i].push(pt);
        }
    }

    //y=maxy
    for x in 1..stage_x - 1  {
        let nx =  inp.stage0.0 + (x as f64) * 10.0;
        let ny =  inp.stage1.1 - 10.0;

        candidate.push(P(nx, ny));
        
        for i in 0..inp.musicians.len() {
            let mut pt = 0;
            for p in 0..inp.pos.len() {

                if inp.pos[p].1 > ny {
                    pt += ( inp.tastes[p][inp.musicians[i]] / (inp.pos[p] - P(nx, ny)).abs2() * 1000000.0) as i64;
                }
            }
            ar[i].push(pt);
        }
    }

    //x=maxx
    for y in 1..stage_y {
        let nx =  inp.stage1.0 - 10.0;
        let ny =  inp.stage0.1 + (y as f64) * 10.0;
        candidate.push(P(nx, ny));
        
        for i in 0..inp.musicians.len() {
            let mut pt = 0;
            for p in 0..inp.pos.len() {

                if inp.pos[p].0 > nx {
                    pt += ( inp.tastes[p][inp.musicians[i]] / (inp.pos[p] - P(nx, ny)).abs2() * 1000000.0) as i64;
                }
            }
            ar[i].push(pt);
        }
    }
    
    for x in 2..stage_x - 1  {
        for y in 2..stage_y - 1 {
            if candidate.len() < inp.musicians.len(){
                let nx =  inp.stage0.0 + (x as f64) * 10.0;
                let ny =  inp.stage0.1 + (y as f64) * 10.0;
                candidate.push(P(nx,ny));
                for i in 0..inp.musicians.len() {
                    ar[i].push(0);
                }
            }
        }
    }

    let ans = weighted_matching(&ar);
    
    let mut ret = Vec::new();
    for i in 0..inp.musicians.len() {
        ret.push(P(candidate[ans.1[i]].0, candidate[ans.1[i]].1));
    }

    dbg!(ans.0);
    dbg!(compute_score(&inp, &ret));

    write_output(&ret);

    //dbg!(get_stage_diff(XY{x:inp.pos[0].0, y:inp.pos[0].1} , XY{x:inp.stage0.0, y:inp.stage0.1}, XY{x:inp.stage1.0, y:inp.stage1.1}));

}

/* 
fn get_stage_diff(target:P, lb:P, ru:P) -> f64 {
    let xdiff = {
        if target.0 < lb.0{
            lb.0 - target.0
        }
        else if target.0 > ru.0{
            target.0 - ru.0
        }
        else{
            0.0
        }
    };

    let ydiff = {
        if target.1 < lb.1{
            lb.1 - target.1
        }
        else if target.1 > ru.1{
            target.1 - ru.1
        }
        else{
            0.0
        }
    };
    xdiff + ydiff
}
*/
