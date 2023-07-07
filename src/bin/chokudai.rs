#![allow(unused_imports)]
use std::{collections::BinaryHeap, net::SocketAddr};

use aead::NewAead;
use icfpc2023::{self, Input, read_input, P, mcf::weighted_matching, write_output, compute_score, compute_score_for_instruments, compute_score_for_a_musician_fast};

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
 
    let candidate = add_candidate2(&inp);
    //let ar = ret_cand_and_ar.1;

    let pos_to_music = compute_score_for_instruments(&inp, &candidate);

    dbg!(candidate.len());

    let mut ar = Vec::new();
    for i in 0..inp.musicians.len() {
        let mut br = Vec::new();
        for j in 0..candidate.len() {
            br.push(pos_to_music[j][inp.musicians[i]]);
        }
        ar.push(br);
    }

    let ans = weighted_matching(&ar);
    let mut ret = Vec::new();
    for i in 0..inp.musicians.len() {
        ret.push(P(candidate[ans.1[i]].0, candidate[ans.1[i]].1));
    }
 
    dbg!(ans.0);
    dbg!(compute_score(&inp, &ret));

    let mut cand2 = Vec::new();
    for i in 0..inp.musicians.len() {
        cand2.push(candidate[ans.1[i]]);
        //dbg!(pos_to_music[ans.1[i]][inp.musicians[i]]);
        //dbg!(compute_score_for_a_musician_fast(&inp, &ret, i).0);
    }
    let candidate = cand2;
    

    let pos_to_music = compute_score_for_instruments(&inp, &candidate);

    let mut ar = Vec::new();
    for i in 0..inp.musicians.len() {
        let mut br = Vec::new();
        for j in 0..candidate.len() {
            br.push(pos_to_music[j][inp.musicians[i]]);
        }
        ar.push(br);
    }

    let ans = weighted_matching(&ar);

    let mut ret = Vec::new();
    for i in 0..inp.musicians.len() {
        ret.push(P(candidate[ans.1[i]].0, candidate[ans.1[i]].1));
    }

    dbg!(compute_score(&inp, &ret));

    write_output(&ret);

    //dbg!(get_stage_diff(XY{x:inp.pos[0].0, y:inp.pos[0].1} , XY{x:inp.stage0.0, y:inp.stage0.1}, XY{x:inp.stage1.0, y:inp.stage1.1}));

}

#[allow(dead_code)]
fn add_candidate2(inp:&Input) -> Vec<P>{
    let mut candidate = Vec::new();


    let mut heap = BinaryHeap::new(); 

    for i in 0..inp.pos.len() {
        let dist = get_stage_diff(inp.pos[i], inp.stage0, inp.stage1) as i64;
        let mut pattern = 0;
        if inp.pos[i].0 < inp.stage0.0 + 10.0{
            pattern += 1;
        }
        if inp.pos[i].1 < inp.stage0.1 + 10.0{
            pattern += 2;
        }
        if inp.pos[i].0 > inp.stage1.0 - 10.0{
            pattern += 4;
        }
        if inp.pos[i].1 > inp.stage1.1 - 10.0{
            pattern += 8;
        }
        heap.push((-dist, pattern, 0, i));
    }

    let r3 = 5.0 * 1.73205 + 0.1;
    
    while !heap.is_empty()
    {
        let node = heap.pop().unwrap();
        let dist = node.0;
        let pattern = node.1;
        let num = node.2;
        let id = node.3;

        let mut flag = false;

        
        let mut ps = Vec::new();


        
        /*
        if pattern == 2{
            ps = vec![P(inp.pos[id].0 + (10.0 * num as f64), inp.stage0.1 + 10.0), P(inp.pos[id].0 - (10.0 * num as f64), inp.stage0.1 + 10.0)];
        }
        else if pattern == 8{
            ps = vec![P(inp.pos[id].0 + (10.0 * num as f64), inp.stage1.1 - 10.0), P(inp.pos[id].0 - (10.0 * num as f64), inp.stage1.1 - 10.0)];
        }
        else if pattern == 1{
            ps = vec![P(inp.stage0.0 + 10.0, inp.pos[id].1 + (10.0 * num as f64)), P(inp.stage0.0 + 10.0, inp.pos[id].1 - (10.0 * num as f64))];
        }
        else if pattern == 4{
            ps = vec![P(inp.stage1.0 - 10.0, inp.pos[id].1 + (10.0 * num as f64)), P(inp.stage1.0 - 10.0, inp.pos[id].1 - (10.0 * num as f64))];
        }
        */

        if pattern == 2{
            ps = vec![P(inp.pos[id].0 + (5.01 + 10.0 * num as f64), inp.stage0.1 + 10.0), P(inp.pos[id].0 - (5.01 + 10.0 * num as f64), inp.stage0.1 + 10.0)];
        }
        else if pattern == 8{
            ps = vec![P(inp.pos[id].0 + (5.01 + 10.0 * num as f64), inp.stage1.1 - 10.0), P(inp.pos[id].0 - (5.01 + 10.0 * num as f64), inp.stage1.1 - 10.0)];
        }
        else if pattern == 1{
            ps = vec![P(inp.stage0.0 + 10.0, inp.pos[id].1 + (5.01 + 10.0 * num as f64)), P(inp.stage0.0 + 10.0, inp.pos[id].1 - (5.01 + 10.0 * num as f64))];
        }
        else if pattern == 4{
            ps = vec![P(inp.stage1.0 - 10.0, inp.pos[id].1 + (5.01 + 10.0 * num as f64)), P(inp.stage1.0 - 10.0, inp.pos[id].1 - (5.01 + 10.0 * num as f64))];
        }

        /*
        if pattern == 2{
            ps = vec![P(inp.pos[id].0 + (5.01 + 10.0 * num as f64), inp.stage0.1 + 10.0), P(inp.pos[id].0 - (5.01 + 10.0 * num as f64), inp.stage0.1 + 10.0), P(inp.pos[id].0, inp.stage0.1 + (10.0 + r3))];
        }
        else if pattern == 8{
            ps = vec![P(inp.pos[id].0 + (5.01 + 10.0 * num as f64), inp.stage1.1 - 10.0), P(inp.pos[id].0 - (5.01 + 10.0 * num as f64), inp.stage1.1 - 10.0), P(inp.pos[id].0, inp.stage1.1 - (10.0 + r3))];
        }
        else if pattern == 1{
            ps = vec![P(inp.stage0.0 + 10.0, inp.pos[id].1 + (5.01 + 10.0 * num as f64)), P(inp.stage0.0 + 10.0, inp.pos[id].1 - (5.01 + 10.0 * num as f64)), P(inp.stage0.0 + (10.0 + r3), inp.pos[id].1)];
        }
        else if pattern == 4{
            ps = vec![P(inp.stage1.0 - 10.0, inp.pos[id].1 + (5.01 + 10.0 * num as f64)), P(inp.stage1.0 - 10.0, inp.pos[id].1 - (5.01 + 10.0 * num as f64)), P(inp.stage1.0 - (10.0 + r3), inp.pos[id].1)];
        }
        */

        for p in ps{
            //dbg!(inp.pos[id]);
            //dbg!(inp.stage0);
            //dbg!(inp.stage1);
            //dbg!(p);

            if check_all_cand(&inp, &candidate, p){
                candidate.push(p);
                flag = true;
            }
        }

        if flag{
            heap.push((dist - 1, pattern, num + 1, id));
        }
    }

    dbg!(candidate.len());
    
    let stage_x = ((inp.stage1.0 - inp.stage0.0) / 10.0) as usize;
    let stage_y = ((inp.stage1.1 - inp.stage0.1) / 10.0) as usize;

    
    let add_l = vec![0.01, 0.2, 0.5, 1.0, 2.0, 3.0, 4.0, 5.0,6.0,7.0,8.0,r3, 9.0];
    
    for add in add_l {
                //y=0
                for x in (inp.stage0.0 as i32 + 10)..(inp.stage1.0 as i32 - 10)  {
                    let nx =  x as f64;
                    let ny =  inp.stage0.1 + 10.0 + add;
                    
                    if !check_all_cand(&inp, &candidate, P(nx,ny)){
                        continue;
                    }
                    candidate.push(P(nx, ny));
                }
        
                //x=0
                for y in (inp.stage0.1 as i32 + 10)..(inp.stage1.1 as i32 - 10) {
                    let nx =  inp.stage0.0 + 10.0 + add;
                    let ny =  y as f64;
        
                    if !check_all_cand(&inp, &candidate, P(nx,ny)){
                        continue;
                    }
        
                    candidate.push(P(nx, ny));
                }


        //y=maxy
        for x in (inp.stage0.0 as i32 + 10)..(inp.stage1.0 as i32 - 10)  {
            let nx =  x as f64;
            let ny =  inp.stage1.1 - 10.0 - add;
            
            if !check_all_cand(&inp, &candidate, P(nx,ny)){
                continue;
            }
            candidate.push(P(nx, ny));
        }

        //x=maxx
        for y in (inp.stage0.1 as i32 + 10)..(inp.stage1.1 as i32 - 10) {
            let nx =  inp.stage1.0 - 10.0 - add;
            let ny =  y as f64;

            if !check_all_cand(&inp, &candidate, P(nx,ny)){
                continue;
            }

            candidate.push(P(nx, ny));
        }
    }

    dbg!(candidate.len());

    for x in 2..stage_x - 1  {
        for y in 2..stage_y - 1 {
            if candidate.len() < inp.musicians.len() * 2 {
                let nx =  inp.stage0.0 + (x as f64) * 10.0;
                let ny =  inp.stage0.1 + (y as f64) * 10.0;
                if check_all_cand(&inp, &candidate, P(nx,ny)){
                    candidate.push(P(nx,ny));
                }
            }
        }
    }
    
    dbg!(candidate.len());



    candidate
}

fn check_all_cand(inp:&Input, cand:&Vec<P>, pos:P) -> bool{
    
    if pos.0 < inp.stage0.0 + 10.0{
        return false;
    }
    if pos.1 < inp.stage0.1 + 10.0{
        return false;
    }
    if pos.0 > inp.stage1.0 - 10.0{
        return false;
    }
    if pos.1 > inp.stage1.1 - 10.0{
        return false;
    }

    for p2 in cand {
        if (pos-*p2).abs2() < 100.0{
            return  false;
        }
    }
    true
}


#[allow(dead_code)]
fn add_candidate(inp:&Input) -> Vec<P>{

    let mut candidate = Vec::new();

    let stage_x = ((inp.stage1.0 - inp.stage0.0) / 10.0) as usize;
    let stage_y = ((inp.stage1.1 - inp.stage0.1) / 10.0) as usize;

    //y=0
    for x in 1..stage_x - 1  {
        let nx =  inp.stage0.0 + (x as f64) * 10.0;
        let ny =  inp.stage0.1 + 10.0;

        if !check_all_cand(&inp, &candidate, P(nx,ny)){
            continue;
        }

        candidate.push(P(nx, ny));
    }

    //x=0
    for y in 2..stage_y - 1 {
        let nx =  inp.stage0.0 + 10.0;
        let ny =  inp.stage0.1 + (y as f64) * 10.0;

        if !check_all_cand(&inp, &candidate, P(nx,ny)){
            continue;
        }

        candidate.push(P(nx, ny));
    }

    //y=maxy
    for x in 1..stage_x - 1  {
        let nx =  inp.stage0.0 + (x as f64) * 10.0;
        let ny =  inp.stage1.1 - 10.0;
        
        if !check_all_cand(&inp, &candidate, P(nx,ny)){
            continue;
        }

        candidate.push(P(nx, ny));
    }

    //x=maxx
    for y in 1..stage_y {
        let nx =  inp.stage1.0 - 10.0;
        let ny =  inp.stage0.1 + (y as f64) * 10.0;

        if !check_all_cand(&inp, &candidate, P(nx,ny)){
            continue;
        }

        candidate.push(P(nx, ny));
    }
    
    for x in 2..stage_x - 1  {
        for y in 2..stage_y - 1 {
            if candidate.len() < inp.musicians.len() * 2{
                let nx =  inp.stage0.0 + (x as f64) * 10.0;
                let ny =  inp.stage0.1 + (y as f64) * 10.0;
                candidate.push(P(nx,ny));
            }
        }
    }

    candidate
}


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
