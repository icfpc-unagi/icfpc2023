#![allow(unused_imports)]
use std::{collections::BinaryHeap, net::SocketAddr};

use aead::NewAead;

use crate::{Input, P};

pub fn get_candidate_tree(inp:&Input) -> (Vec<P>, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<bool>){
    
    let mut ret_ps = vec![];
    let mut parent = vec![];
    let mut child = vec![];
    let mut connect = vec![];
    let mut valid = vec![];

    for i in 0..inp.pos.len() {
        let dist = get_stage_diff(inp.pos[i], inp.stage0, inp.stage1) as i64;

        let mut maxpower = -1000.0;
        for power in &inp.tastes[i] {
            if maxpower < *power{
                maxpower = *power;
            }   
        }
        if maxpower <= 0.0{
            continue;
        }


        let mut pattern = 0;
        if inp.pos[i].0 < inp.stage0.0{
            pattern += 1;
        }
        if inp.pos[i].1 < inp.stage0.1{
            pattern += 2;
        }
        if inp.pos[i].0 > inp.stage1.0{
            pattern += 4;
        }
        if inp.pos[i].1 > inp.stage1.1{
            pattern += 8;
        }


        if pattern & (pattern - 1) != 0{
            continue;
        }

        let d2 = get_stage_diff(inp.pos[i], inp.stage0, inp.stage1);

        if d2 > 20.0{
            continue;
        }


        let mut ps = vec![];
        let mut ids = vec![];

        for pat in 0..3
        {
            
            for num in 0..3{
                let pre_ps = ps.clone();
                let pre_id = ids.clone();
                ps = vec![];
                ids = vec![];

                if pat == 0{
                    if pattern == 1{
                        let a = (inp.stage0.0 + 10.0) - inp.pos[i].0;
                        let b = calc_y_to_x(a);
                        let c = (a*a+b*b).sqrt()-a;
                        let p_0 = P(inp.stage0.0 + 10.0 + c, inp.pos[i].1);
                        let p_1 = P(inp.stage0.0 + 10.0, inp.pos[i].1 + b + 10.0 * num as f64) ;
                        let p_2 = P(inp.stage0.0 + 10.0, inp.pos[i].1 - b - 10.0 * num as f64);
                        let p_3 = p_1 + (p_0-p_1).rot60();
                        let p_4 = p_0 + (p_2-p_0).rot60();
                        ps = vec![p_0, p_1, p_2, p_3, p_4];
                    }
                    else if pattern == 4{
                        let a = inp.pos[i].0 - (inp.stage1.0 - 10.0);
                        let b = calc_y_to_x(a);
                        let c = (a*a+b*b).sqrt()-a;
                        let p_0 = P(inp.stage1.0 - 10.0 - c, inp.pos[i].1);
                        let p_1 = P(inp.stage1.0 - 10.0, inp.pos[i].1 - b - 10.0 * num as f64);
                        let p_2 = P(inp.stage1.0 - 10.0, inp.pos[i].1 + b + 10.0 * num as f64);
                        let p_3 = p_1 + (p_0-p_1).rot60();
                        let p_4 = p_0 + (p_2-p_0).rot60();
                        ps = vec![p_0, p_1, p_2, p_3, p_4];
                    }
                    else if pattern == 2{
                        let a = (inp.stage0.1 + 10.0) - inp.pos[i].1;
                        let b = calc_y_to_x(a);
                        let c = (a*a+b*b).sqrt()-a;
                        let p_0 = P(inp.pos[i].0, inp.stage0.1 + 10.0 + c);
                        let p_1 = P(inp.pos[i].0 + b + 10.0 * num as f64, inp.stage0.1 + 10.0) ;
                        let p_2 = P(inp.pos[i].0 - b - 10.0 * num as f64, inp.stage0.1 + 10.0) ;
                        let p_3 = p_1 + (p_0-p_1).rot60();
                        let p_4 = p_0 + (p_2-p_0).rot60();
                        ps = vec![p_0, p_1, p_2, p_3, p_4];
                    }
                    else if pattern == 8{
        
                        let a = inp.pos[i].1 - (inp.stage1.1 - 10.0);
                        let b = calc_y_to_x(a);
                        let c = (a*a+b*b).sqrt()-a;
                        let p_0 = P(inp.pos[i].0, inp.stage1.1 - 10.0 - c);
                        let p_1 = P(inp.pos[i].0 + b + 10.0 * num as f64, inp.stage1.1 - 10.0) ;
                        let p_2 = P(inp.pos[i].0 - b - 10.0 * num as f64, inp.stage1.1 - 10.0) ;
                        let p_3 = p_1 + (p_0-p_1).rot60();
                        let p_4 = p_0 + (p_2-p_0).rot60();
                        ps = vec![p_0, p_1, p_2, p_3, p_4];
                    }

                    if num == 0{
                        ids.push(ret_ps.len()); ret_ps.push(ps[1]); parent.push(vec![]); child.push(vec![]); 
                        ids.push(ret_ps.len()); ret_ps.push(ps[2]); parent.push(vec![]); child.push(vec![]); 
                        ids.push(ret_ps.len()); ret_ps.push(ps[0]); parent.push(vec![]); child.push(vec![]); set_oyako(&mut parent, &mut child, ids[0], ids[2]); set_oyako(&mut parent, &mut child, ids[1], ids[2]);
                        ids.push(ret_ps.len()); ret_ps.push(ps[3]); parent.push(vec![]); child.push(vec![]); set_oyako(&mut parent, &mut child, ids[0], ids[3]); set_oyako(&mut parent, &mut child, ids[1], ids[3]);
                        ids.push(ret_ps.len()); ret_ps.push(ps[4]); parent.push(vec![]); child.push(vec![]); set_oyako(&mut parent, &mut child, ids[0], ids[4]); set_oyako(&mut parent, &mut child, ids[2], ids[4]);
                    }
                    else
                    {
                        ids.push(ret_ps.len()); ret_ps.push(ps[1]); parent.push(vec![]); child.push(vec![]); set_oyako(&mut parent, &mut child, pre_id[0], ids[0]);
                        ids.push(ret_ps.len()); ret_ps.push(ps[2]); parent.push(vec![]); child.push(vec![]); set_oyako(&mut parent, &mut child, pre_id[1], ids[1]);
                    }
                }
                else if pat == 1{

                    let r3 = 5.0 * 1.73205 + 0.1;
                    if pattern == 2{
                        ps = vec![P(inp.pos[i].0 + (10.0 * num as f64), inp.stage0.1 + 10.0), P(inp.pos[i].0 - (10.0 * num as f64), inp.stage0.1 + 10.0), P(inp.pos[i].0+ (10.0 * num as f64), inp.stage0.1 + 10.0 + r3), P(inp.pos[i].0 - (10.0 * num as f64), inp.stage0.1 + 10.0 + r3)];
                    }
                    else if pattern == 8{
                        ps = vec![P(inp.pos[i].0 + (10.0 * num as f64), inp.stage1.1 - 10.0), P(inp.pos[i].0 - (10.0 * num as f64), inp.stage1.1 - 10.0), P(inp.pos[i].0+ (10.0 * num as f64), inp.stage1.1 - 10.0 - r3), P(inp.pos[i].0 - (10.0 * num as f64), inp.stage1.1 - 10.0 - r3)];
                    }
                    else if pattern == 1{
                        ps = vec![P(inp.stage0.0 + 10.0, inp.pos[i].1 + (10.0 * num as f64)), P(inp.stage0.0 + 10.0, inp.pos[i].1 - (10.0 * num as f64)), P(inp.stage1.0 + 10.0 + r3,  inp.pos[i].1 - (10.0 * num as f64)), P(inp.stage1.0 + 10.0 + r3,  inp.pos[i].1 + (10.0 * num as f64))];
                    }
                    else if pattern == 4{
                        ps = vec![P(inp.stage1.0 - 10.0, inp.pos[i].1 + (10.0 * num as f64)), P(inp.stage1.0 - 10.0, inp.pos[i].1 - (10.0 * num as f64)), P(inp.stage1.1 - 10.0 - r3,  inp.pos[i].1 - (10.0 * num as f64)), P(inp.stage1.1 - 10.0 - r3,  inp.pos[i].1 + (10.0 * num as f64))];
                    }

                    if num == 0{
                        ids.push(ret_ps.len()); ret_ps.push(ps[0]); parent.push(vec![]); child.push(vec![]); 
                        ids.push(ret_ps.len()); ret_ps.push(ps[1]); parent.push(vec![]); child.push(vec![]); 
                        ids.push(ret_ps.len()); ret_ps.push(ps[2]); parent.push(vec![]); child.push(vec![]); set_oyako(&mut parent, &mut child, ids[0], ids[2]); set_oyako(&mut parent, &mut child, ids[1], ids[2]); 
                    }
                    else{
                        ids.push(ret_ps.len()); ret_ps.push(ps[0]); parent.push(vec![]); child.push(vec![]); 
                        ids.push(ret_ps.len()); ret_ps.push(ps[1]); parent.push(vec![]); child.push(vec![]); 
                        ids.push(ret_ps.len()); ret_ps.push(ps[2]); parent.push(vec![]); child.push(vec![]); set_oyako(&mut parent, &mut child,pre_id[2], ids[2]); set_oyako(&mut parent, &mut child, ids[0], ids[2]); 

                        let pre = pre_id.len() - 1;
                        ids.push(ret_ps.len()); ret_ps.push(ps[3]); parent.push(vec![]); child.push(vec![]); set_oyako(&mut parent, &mut child, pre_id[pre], ids[3]); set_oyako(&mut parent, &mut child, ids[1], ids[3]); 
                    }
                }
                else if pat == 2{
                    if pattern == 2{
                        ps = vec![P(inp.pos[i].0 + (10.0 * num as f64), inp.stage0.1 + 10.0), P(inp.pos[i].0 - (10.0 * num as f64), inp.stage0.1 + 10.0)];
                    }
                    else if pattern == 8{
                        ps = vec![P(inp.pos[i].0 + (10.0 * num as f64), inp.stage1.1 - 10.0), P(inp.pos[i].0 - (10.0 * num as f64), inp.stage1.1 - 10.0)];
                    }
                    else if pattern == 1{
                        ps = vec![P(inp.stage0.0 + 10.0, inp.pos[i].1 + (10.0 * num as f64)), P(inp.stage0.0 + 10.0, inp.pos[i].1 - (10.0 * num as f64))];
                    }
                    else if pattern == 4{
                        ps = vec![P(inp.stage1.0 - 10.0, inp.pos[i].1 + (10.0 * num as f64)), P(inp.stage1.0 - 10.0, inp.pos[i].1 - (10.0 * num as f64))];
                    }

                    if num == 0{
                        ids.push(ret_ps.len()); ret_ps.push(ps[0]); parent.push(vec![]); child.push(vec![]); 
                    } 
                    else{
                        ids.push(ret_ps.len()); ret_ps.push(ps[0]); parent.push(vec![]); child.push(vec![]); set_oyako(&mut parent, &mut child, pre_id[0], ids[0]);
                        let pre = pre_id.len() - 1;
                        ids.push(ret_ps.len()); ret_ps.push(ps[1]); parent.push(vec![]); child.push(vec![]); set_oyako(&mut parent, &mut child, pre_id[pre], ids[1]);
                    }
                }
            }
        }
    }

    let mut add_cnt = 0;

    let xmax = (inp.stage1.0-inp.stage0.0) / 10.0;
    let ymax = (inp.stage1.1-inp.stage0.1) / 10.0;
    for i in 2..(xmax as usize - 1){
        for j in 2..(ymax as usize - 1){
            if add_cnt < inp.musicians.len(){
                let p2 = P(inp.stage0.0 + (i as f64 * 10.0), inp.stage0.1 + (j as f64* 10.0));
                ret_ps.push(p2);
                parent.push(vec![]); child.push(vec![]); 
                add_cnt += 1;
            }
        }
    }



    dbg!(ret_ps.len());

    connect = vec![vec![]; ret_ps.len()];
    valid = vec![true; ret_ps.len()];

    for i in 0..ret_ps.len() {
        if ret_ps[i].0 < inp.stage0.0 + 10.0 || ret_ps[i].1 < inp.stage0.1 + 10.0 || ret_ps[i].0 > inp.stage1.0 - 10.0 || ret_ps[i].1 > inp.stage1.1 - 10.0 {
            valid[i] = false;
        }
    }
    
    for i in 0..ret_ps.len() {
        if !valid[i]{
            continue;
        }
        for j in i+1..ret_ps.len(){
            if !valid[j]{
                continue;
            }
            let diff = ret_ps[i] - ret_ps[j];
            if diff.abs2() < 100.0{
                connect[i].push(j);
                connect[j].push(i);
            }
        }
    }

    (ret_ps, parent, child, connect, valid)
}

fn set_oyako(parent: &mut Vec<Vec<usize>>, child: &mut Vec<Vec<usize>>, p: usize, c: usize){
    child[p].push(c);
    parent[c].push(p);
}

fn set_more_candidate(inp:&Input, candidate:Vec<P>) -> Vec<P>{
    let mut ret = candidate.clone();
    let stage_x = ((inp.stage1.0 - inp.stage0.0) / 10.0) as usize;
    let stage_y = ((inp.stage1.1 - inp.stage0.1) / 10.0) as usize;
    for x in 2..stage_x - 1  {
        for y in 2..stage_y - 1 {
            if ret.len() < inp.musicians.len() * 4 / 3 {
                let nx =  inp.stage0.0 + (x as f64) * 10.0;
                let ny =  inp.stage0.1 + (y as f64) * 10.0;
                if check_all_cand(&inp, &ret, P(nx,ny)){
                    ret.push(P(nx,ny));
                }
            }
        }
    }
    return ret;
}






fn calc_y_to_x(y: f64) -> f64{
    let mut ok = 11.0;
    let mut ng = 0.0;
    for _ in 0 .. 100 {
        let mid = (ok + ng) / 2.0;
        if f(y, mid){
            ok = mid;
        }
        else{
            ng = mid;
        }
    }
    ok
}

fn f(y: f64, x:f64) -> bool{
    let a = (x*x+y*y).sqrt()-y;
    return a * a  + x * x >= 100.1;
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
