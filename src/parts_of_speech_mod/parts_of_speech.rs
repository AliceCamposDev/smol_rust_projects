use rust_bert::pipelines::{pos_tagging::POSModel};

pub fn parts_of_speech_fn() {
    println!("Parts of Speech");
    pos();
}

fn pos(){
    let pos_model = POSModel::new(Default::default()).unwrap();

    let input = ["hi, is it rainning today?"];
    let output = pos_model.predict(&input);
    for (pos, tag) in output [0].iter().enumerate(){
        println!("{pos}: {tag:?} ");
    }
    
}