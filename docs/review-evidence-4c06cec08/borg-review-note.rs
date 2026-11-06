use borg_rs::{borg_item::{blank_item,set_item_note,item_note},borg_item_id::borg_item_note_needs_id};
fn main() {
    let mut live = [blank_item();3];
    live[0].iqty = 1;
    live[1].iqty = 1;
    live[0].aware = true;
    live[1].aware = true;
    for (i,s) in [(0,"Old"),(1,"New longer {??}")] {
        for (d,c) in live[i].desc.iter_mut().zip(s.bytes()) { *d=c as _; }
    }
    set_item_note(&mut live[0],3);
    set_item_note(&mut live[1],11);
    let safe=live;
    live[2]=safe[0];
    live[0]=safe[1];
    println!("hole note=[{}] needs_id={}",String::from_utf8(item_note(&live[2])).unwrap(),borg_item_note_needs_id(&live[2]));
}
