use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

#[inline]
fn matched(keys1 : &Vec<u8>,keys2 : &Vec<u8>) -> bool {
    let len_key1 = keys1.len();
    let len_key2 = keys2.len();

    // Confirm length of 2 strings has equal
    if len_key1 - len_key2 != 0 {
        return false;
    }

    // Comfirm first and last charactor has matched
    let first_last = ( keys1[0] - keys2[0] ) + ( keys1[len_key1 - 1] - keys2[len_key2 - 1] );
    if first_last != 0 {
        return false;
    }

    // Mid = ((total length - length of 2 chars) / 2 )  + 1
    // Example :
    // keys = 5000 charactors
    // mid  = 5000 - 2 / 2 + 1
    //      = 2500
    let mid : usize = (len_key1 - 2) / 2 + 1;
    let shif_mid : usize = mid - 1;

    // Loop i = 2 to 2500 - 1 , i+=2
    // Forward step :
    //    ODD step    - x1[i] - x2[i] , 2,4,6,8...2498 (<2500-1 = 2549) , i+=2
    //    EVEN step   - x1[i-1] - x2[i-1] , 1,3,5,7,...2547 , i+=1
    // Backward step :
    //    ODD step    - x1[mid + i] - x2[mid + i] , 2500 + 2,2500 + 4 ,2500 + 6,...2500 + 2498=4998 , i+=2
    //    EVEN        - x1[mid + i - 1] - x2[mid + i - 1] , 2500 + 2 - 1 , 2500 + 4 - 1,...2500 + 2498 - 1 = 4997
    for i in (2..mid - 1).step_by(2)  {
        let result = ( keys1[i] -   keys2[i] ) + 
                         ( keys1[i-1] - keys2[i-1] ) +
                         ( keys1[mid + i] - keys2[mid + i] ) + 
                         ( keys1[shif_mid + i] - keys2[shif_mid + i] );
        if result != 0 {
            return false;
        }
    }
    
    true
}

#[inline]
fn in_matched(keys1 : &Vec<u8>,keys2 : &Vec<u8>) -> bool {
    // Check length of hash key and store hash key.
    // When length of 2 keys has not match.Then return false
    
    if keys1.len() != keys2.len() {
        return false;
    }

    // Check first and last charactor key.
    // If not matched return false
    if  keys1.first() != keys2.first()  || 
        keys1.last()  != keys2.last()
    {
        return false;
    }

    // Loop from charactor at 1 to before last char of key
    for i in 1..keys1.len() - 1 {
        if keys1[i] - keys2[i] != 0 {
            return false;
        } 
    }

    return true;
}

#[inline]
fn builtin_matched(keys1 : &Vec<u8>,keys2 : &Vec<u8>)  -> bool {
    let matching = keys1.iter().zip(keys2).filter(|&(keys1,keys2)| keys1 == keys2).count();
    matching == keys1.len()
}


fn bench_matched(c : &mut Criterion) {
    let mut keys1 = String::from("A");
    let mut keys2 = String::from("A");

    keys1.extend(vec!['H';1000].iter());
    keys1.extend(vec!['K';1000].iter());
    keys1.extend(vec!['D';1000].iter());
    keys1.extend(vec!['O';1000].iter());
    keys1.extend(vec!['R';1000].iter());

    keys2.extend(vec!['H';1000].iter());
    keys2.extend(vec!['K';1000].iter());
    keys2.extend(vec!['D';1000].iter());
    keys2.extend(vec!['O';1000].iter());
    keys2.extend(vec!['R';1000].iter());

    keys1.push('B');
    keys2.push('B');

    let element = black_box(
        (keys1.as_bytes().to_vec(),keys2.as_bytes().to_vec())
    );

    c.bench_function(
        "Compare 2 Strings",
        |b| b.iter(|| matched(&element.0,&element.1))
    );
}

fn inuse_bench_matched(c : &mut Criterion) {
    let mut keys1 = String::from("A");
    let mut keys2 = String::from("A");

    keys1.extend(vec!['H';1000].iter());
    keys1.extend(vec!['K';1000].iter());
    keys1.extend(vec!['D';1000].iter());
    keys1.extend(vec!['O';1000].iter());
    keys1.extend(vec!['R';1000].iter());

    keys2.extend(vec!['H';1000].iter());
    keys2.extend(vec!['K';1000].iter());
    keys2.extend(vec!['D';1000].iter());
    keys2.extend(vec!['O';1000].iter());
    keys2.extend(vec!['R';1000].iter());

    keys1.push('B');
    keys2.push('B');

    let element = black_box(
        (keys1.as_bytes().to_vec(),keys2.as_bytes().to_vec())
    );

    c.bench_function(
        "Compare 2 Strings with in used",
        |b| b.iter(|| in_matched(&element.0,&element.1))
    );
}

fn builtin_bench_matched(c : &mut Criterion) {
    let mut keys1 = String::from("A");
    let mut keys2 = String::from("A");

    keys1.extend(vec!['H';1000].iter());
    keys1.extend(vec!['K';1000].iter());
    keys1.extend(vec!['D';1000].iter());
    keys1.extend(vec!['O';1000].iter());
    keys1.extend(vec!['R';1000].iter());

    keys2.extend(vec!['H';1000].iter());
    keys2.extend(vec!['K';1000].iter());
    keys2.extend(vec!['D';1000].iter());
    keys2.extend(vec!['O';1000].iter());
    keys2.extend(vec!['R';1000].iter());

    keys1.push('B');
    keys2.push('B');

    let element = black_box(
        (keys1.as_bytes().to_vec(),keys2.as_bytes().to_vec())
    );

    c.bench_function(
        "Compare 2 Strings with builtin lib",
        |b| b.iter(|| builtin_matched(&element.0,&element.1))
    );
}

criterion_group!(
    benches,
    bench_matched,
    inuse_bench_matched,
    builtin_bench_matched
);
criterion_main!(benches);
